use super::*;
use crate::Result;
use crate::ffmpeg::{Rescale, format};
use std::path::{Path, PathBuf};

impl Retiming<'_, '_> {
    pub(crate) fn try_audio(&self, i: usize, src: &Path, i_stream: usize) -> Result<RetimedStream> {
        let splits = if src == **self.base {
            self.try_base_audio(i_stream)
        } else {
            self.try_external_audio(i, src, i_stream)
        }?;

        let dest = self.temp_dir.join(format!("{}-aud-{}.mka", self.job, i));
        try_concat(src, &splits, &dest)?;

        Ok(RetimedStream {
            src: Some(dest),
            i_stream: 0,
        })
    }

    fn try_base_audio(&self, i_stream: usize) -> Result<Vec<PathBuf>> {
        let mut len_offset = SignedTime::ZERO;

        self.parts
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let dest = self
                    .temp_dir
                    .join(format!("{}-aud-base-{}-{}.mka", self.job, i_stream, i));

                let start_minus_len_offset = SignedTime::new(true, p.start) - len_offset;

                let start = if start_minus_len_offset.is_positive() {
                    start_minus_len_offset.as_unsigned_time()
                } else {
                    p.start
                };

                len_offset = try_split(&p.src, i_stream, &dest, start, p.end)?;
                Ok(dest)
            })
            .collect()
    }

    fn try_external_audio(&self, i: usize, src: &Path, i_stream: usize) -> Result<Vec<PathBuf>> {
        let mut segments: Vec<PathBuf> = Vec::with_capacity(self.chapters.len());
        let mut len_offset = SignedTime::ZERO;

        for p in self.parts.iter() {
            let uid = &self.chapters[p.i_start_chp].uid;
            for i_chp in p.i_start_chp..=p.i_end_chp {
                let chp = &self.chapters[i_chp];
                if uid != &chp.uid {
                    continue;
                }
                let dest = self
                    .temp_dir
                    .join(format!("{}-aud-{}-{}.mka", self.job, i, i_chp));

                let chp_nonuid = self.chapters_nonuid(i_chp);

                let signed_start = p.start_offset + chp.start + chp_nonuid;
                let start_minus_len_offset = signed_start - len_offset;

                let trg_start = if start_minus_len_offset.is_positive() {
                    start_minus_len_offset.as_unsigned_time()
                } else {
                    signed_start.as_unsigned_time()
                };

                let end_offset = if i_chp == p.i_end_chp {
                    p.end_offset
                } else {
                    p.start_offset
                };
                let trg_end = (end_offset + chp.end + chp_nonuid).as_unsigned_time();

                len_offset = try_split(src, i_stream, &dest, trg_start, trg_end)?;
                segments.push(dest);
            }
        }

        Ok(segments)
    }
}

// returns end offset
fn try_split(
    src: &Path,
    i_stream: usize,
    dest: &Path,
    trg_start: Time,
    trg_end: Time,
) -> Result<SignedTime> {
    let mut ictx = format::input(&src)?;
    let mut octx = format::output(&dest)?;

    let (ist_time_base, ost_time_base, ost_index) =
        write_stream_copy_header(&ictx, i_stream, &mut octx)?;

    let start_ts = time_to_ts(trg_start, ost_time_base);
    let end_ts = time_to_ts(trg_end, ost_time_base);

    let rescale = |ts: i64| ts.rescale(ist_time_base, ost_time_base);

    let mut last_pts = 0i64;
    let mut offset = None::<i64>;

    for (ist, mut packet) in ictx.packets() {
        if ist.index() != i_stream {
            continue;
        }
        let pts = some_or!(packet.pts(), continue);
        let pts = rescale(pts);

        if pts < start_ts {
            continue;
        }
        if pts > end_ts {
            break;
        }

        last_pts = pts - *offset.get_or_insert_with(|| start_ts + pts - start_ts);

        packet.set_duration(0);
        packet.set_pts(Some(last_pts));
        packet.set_dts(Some(last_pts));
        packet.set_stream(ost_index);
        packet.write_interleaved(&mut octx)?;
    }

    octx.write_trailer()?;

    let expected_duration_ts = end_ts - start_ts;
    let offset_ts = last_pts - expected_duration_ts;
    let is_positive = offset_ts.is_positive();

    let offset = offset_ts.rescale(ost_time_base, MILLISECOND_TIME_BASE);
    let offset = Time::from_millis(offset.abs() as u64);

    Ok(SignedTime::new(is_positive, offset))
}
