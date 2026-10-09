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
            src: dest,
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

    let start_ts = time_to_ts(trg_start, ist_time_base);
    let end_ts = time_to_ts(trg_end, ist_time_base);

    let rescale = |ts: i64| ts.rescale(ist_time_base, ost_time_base);

    let mut max_pts: (i64, i64) = (i64::MIN, 0);
    let mut min_pts = None::<i64>;

    for (ist, mut packet) in ictx.packets() {
        if ist.index() != i_stream {
            continue;
        }

        let pts = match packet.pts().or(packet.dts()) {
            Some(ts) => ts,
            None => return Err(err!("fail get packet pts")),
        };

        if pts < start_ts {
            continue;
        }
        if pts > end_ts {
            break;
        }

        if pts >= max_pts.0 {
            max_pts.0 = pts;
            max_pts.1 = packet.duration();
        }

        let offset = *min_pts.get_or_insert(pts);
        let new_pts = packet.pts().map(|pts| rescale(pts - offset));
        let new_dts = packet.dts().map(|dts| rescale(dts - offset));

        packet.set_pts(new_pts);
        packet.set_dts(new_dts);
        packet.set_duration(rescale(packet.duration()));
        packet.set_stream(ost_index);

        packet.write(&mut octx)?;
    }

    let min_pts = min_pts.ok_or_else(|| err!("not written a packet"))?;
    octx.write_trailer()?;

    let expected_duration_ts = end_ts - start_ts;
    let offset_ts = (max_pts.0 + max_pts.1 - min_pts) - expected_duration_ts;

    let is_positive = offset_ts.is_positive();
    let offset = offset_ts.rescale(ist_time_base, MILLISECOND_TIME_BASE);
    let offset = Time::from_millis(offset.abs() as u64);

    Ok(SignedTime::new(is_positive, offset))
}
