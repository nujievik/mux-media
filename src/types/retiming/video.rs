use super::{RetimedStream, Retiming, write_split_header};
use crate::{
    Duration, Result,
    ffmpeg::{Packet, Rescale, format},
};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

impl Retiming<'_, '_> {
    pub(super) fn try_video(&self, src: &Path, i_stream: usize) -> Result<RetimedStream> {
        if i_stream != self.i_base_stream && src != self.base.as_path() {
            return Err(err!(
                "Unsupported retiming more than 1 video track at a time. Skipping {} stream {}",
                src.display(),
                i_stream
            ));
        }

        if self.parts.len() == 1 && self.parts[0].start.is_zero() {
            Ok(self.single_part_base_retimed_stream(src, i_stream))
        } else {
            self.try_base_video()
        }
    }

    pub(super) fn init_base_splits(&mut self) -> Result<()> {
        const SHIFT_START: f64 = 0.3;
        const SHIFT_END: f64 = -0.1;

        let raw_splits: Vec<(usize, f64, f64, PathBuf)> = self
            .parts
            .par_iter()
            .enumerate()
            .map(|(i, p)| {
                let split = self
                    .temp_dir
                    .join(format!("{}-vid-base-{}.mkv", self.job, i));

                let start = if p.start.0.is_zero() {
                    p.start
                } else {
                    Duration::from_secs_f64(p.start.as_secs_f64() + SHIFT_START)
                };
                let end = Duration::from_secs_f64(p.end.as_secs_f64() + SHIFT_END);

                try_split(&p.src, self.i_base_stream, &split, start, end)
                    .map(|(start, end)| (i, start, end, split))
            })
            .collect::<Result<_>>()?;

        let base_splits: Vec<_> = raw_splits
            .into_iter()
            .map(|(i, start, end, split)| {
                let p = &mut self.parts[i];
                p.start_offset += start - p.start.as_secs_f64();
                p.end_offset += end - p.end.as_secs_f64();

                p.start = Duration::from_secs_f64(start);
                p.end = Duration::from_secs_f64(end);
                split
            })
            .collect();

        self.base_splits = base_splits;
        Ok(())
    }

    fn try_base_video(&self) -> Result<RetimedStream> {
        let txt = self
            .temp_dir
            .join(format!("{}-vid-base-parts.txt", self.job));

        let dest = self.temp_dir.join(format!("{}-vid-base.mkv", self.job));
        self.try_concat(&self.base_splits, &txt, &dest)?;

        Ok(RetimedStream {
            src: Some(dest),
            i_stream: 0,
            src_time: None,
        })
    }
}

fn try_split(
    src: &Path,
    i_stream: usize,
    dest: &Path,
    trg_start: Duration,
    trg_end: Duration,
) -> Result<(f64, f64)> {
    const ACCEPT_VIDEO_OFFSET: f64 = 1.0; // seconds

    let mut ictx = format::input(&src)?;
    let mut octx = format::output(&dest)?;

    let (ist_time_base, ost_time_base, ost_index) = write_split_header(&ictx, i_stream, &mut octx)?;

    let seconds_to_ts = |secs: f64| {
        let tb = ost_time_base.0 as f64 / ost_time_base.1 as f64;
        (secs / tb).round() as i64
    };
    let start_ts = seconds_to_ts(trg_start.as_secs_f64());
    let end_ts = seconds_to_ts(trg_end.as_secs_f64());
    let accept = seconds_to_ts(ACCEPT_VIDEO_OFFSET);

    let rescale = |ts: i64| ts.rescale(ist_time_base, ost_time_base);

    let mut min_pts = None::<i64>;
    let mut max_pts: i64 = 0;
    let mut ts_offset = None::<i64>;
    let mut was_out_of_end = false;
    let mut last_packet = None::<Packet>;

    for (ist, mut packet) in ictx.packets() {
        if ist.index() != i_stream {
            continue;
        }
        let pts = some_or!(packet.pts(), continue);
        let pts = rescale(pts);

        let is_key = packet.is_key();

        if min_pts.is_none() {
            if !is_key || start_ts - pts > accept {
                continue;
            }
        }

        if let Some(pkt) = last_packet {
            pkt.write_interleaved(&mut octx)?;
        }

        if pts > end_ts {
            was_out_of_end = true;
        }
        let is_end = was_out_of_end && is_key;

        let min = *min_pts.get_or_insert_with(|| pts);
        min_pts = Some(min.min(pts));
        max_pts = max_pts.max(pts);

        let offset = *ts_offset.get_or_insert_with(|| start_ts + pts - start_ts);
        let new_pts = pts - offset;
        let new_dts = packet.dts().map(|ts| rescale(ts) - offset);

        if is_end {
            packet.set_duration(0);
        }

        packet.set_pts(Some(new_pts));
        packet.set_dts(new_dts);
        packet.set_stream(ost_index);
        last_packet = Some(packet);

        if is_end {
            break;
        }
    }

    if let Some(mut pkt) = last_packet {
        pkt.set_duration(0);
        pkt.write_interleaved(&mut octx)?;
    }

    let min_pts = min_pts.ok_or_else(|| err!("Not written a packet"))?;
    octx.write_trailer()?;

    let to_seconds = |ts| ts as f64 * ost_time_base.0 as f64 / ost_time_base.1 as f64;
    Ok((to_seconds(min_pts), to_seconds(max_pts)))
}
