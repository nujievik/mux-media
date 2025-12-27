use super::{RetimedStream, Retiming, try_concat, write_stream_copy_header};
use crate::{
    Duration, Result,
    ffmpeg::{Rescale, format},
};
use std::path::{Path, PathBuf};

impl Retiming<'_, '_> {
    pub(crate) fn try_audio(&self, i: usize, src: &Path, i_stream: usize) -> Result<RetimedStream> {
        if self.is_save_single_part() && src == **self.base {
            return Ok(self.single_part_base_retimed_stream(src, i_stream));
        }

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
            src_time: None,
        })
    }

    fn try_base_audio(&self, i_stream: usize) -> Result<Vec<PathBuf>> {
        let mut len_offset = 0f64;

        self.parts
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let dest = self
                    .temp_dir
                    .join(format!("{}-aud-base-{}-{}.mka", self.job, i_stream, i));

                let start_f64 = p.start.as_secs_f64();
                let start = if start_f64 - len_offset > 0.0 {
                    Duration::from_secs_f64(start_f64 - len_offset)
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
        let mut len_offset = 0f64;

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

                let mut start = chp.start.as_secs_f64() + p.start_offset + chp_nonuid;
                if start - len_offset > 0.0 {
                    start -= len_offset;
                }

                let end_offset = if i_chp == p.i_end_chp {
                    p.end_offset
                } else {
                    p.start_offset
                };
                let end = chp.end.as_secs_f64() + end_offset + chp_nonuid;

                let trg_start = Duration::from_secs_f64(start);
                let trg_end = Duration::from_secs_f64(end);
                len_offset = try_split(src, i_stream, &dest, trg_start, trg_end)?;

                segments.push(dest);
            }
        }

        Ok(segments)
    }
}

fn try_split(
    src: &Path,
    i_stream: usize,
    dest: &Path,
    trg_start: Duration,
    trg_end: Duration,
) -> Result<f64> {
    let mut ictx = format::input(&src)?;
    let mut octx = format::output(&dest)?;

    let (ist_time_base, ost_time_base, ost_index) =
        write_stream_copy_header(&ictx, i_stream, &mut octx)?;

    let duration_to_ts = |dur: Duration| {
        let tb = ost_time_base.0 as f64 / ost_time_base.1 as f64;
        (dur.as_secs_f64() / tb).round() as i64
    };
    let start_ts = duration_to_ts(trg_start);
    let end_ts = duration_to_ts(trg_end);

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

    let expected_duration = end_ts - start_ts;
    let offset = last_pts - expected_duration;
    let offset = offset as f64 * ost_time_base.0 as f64 / ost_time_base.1 as f64;

    Ok(offset)
}
