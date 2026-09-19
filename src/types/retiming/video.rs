use super::*;
use crate::{
    Result, display,
    ffmpeg::{Packet, Rescale, format},
};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

impl Retiming<'_, '_> {
    pub(super) fn try_video(&self, src: &Path, i_stream: usize) -> Result<RetimedStream> {
        if i_stream != self.i_base_stream && src != self.base.as_path() {
            return Err(err!(
                "Unsupported retiming more than 1 video track at a time. Skipping {} stream {}",
                display(src),
                i_stream
            ));
        }

        self.try_base_video()
    }

    pub(super) fn init_base_splits(&mut self) -> Result<()> {
        let raw_splits: Vec<(usize, Time, Time, PathBuf)> = self
            .parts
            .par_iter()
            .enumerate()
            .map(|(i, p)| {
                let split = self
                    .temp_dir
                    .join(format!("{}-vid-base-{}.mkv", self.job, i));

                try_split(&p.src, self.i_base_stream, &split, p.start, p.end)
                    .map(|(start, end)| (i, start, end, split))
            })
            .collect::<Result<_>>()?;

        let base_splits: Vec<_> = raw_splits
            .into_iter()
            .map(|(i, start, end, split)| {
                let p = &mut self.parts[i];
                p.start_offset += SignedTime::new(true, start) - p.start;
                p.end_offset += SignedTime::new(true, end) - p.end;

                p.start = start;
                p.end = end;
                split
            })
            .collect();

        self.base_splits = base_splits;
        Ok(())
    }

    fn try_base_video(&self) -> Result<RetimedStream> {
        let dest = self.temp_dir.join(format!("{}-vid-base.mkv", self.job));
        try_concat(&self.base, &self.base_splits, &dest)?;

        Ok(RetimedStream {
            src: Some(dest),
            i_stream: 0,
        })
    }
}

// returns (start, end)
fn try_split(
    src: &Path,
    i_stream: usize,
    dest: &Path,
    trg_start: Time,
    trg_end: Time,
) -> Result<(Time, Time)> {
    const ACCEPT_VIDEO_OFFSET: Time = Time::from_secs(1);

    let mut ictx = format::input(&src)?;
    let mut octx = format::output(&dest)?;

    let (ist_time_base, ost_time_base, ost_index) =
        write_stream_copy_header(&ictx, i_stream, &mut octx)?;

    let start_ts = time_to_ts(trg_start, ost_time_base);
    let end_ts = time_to_ts(trg_end, ost_time_base);
    let accept = time_to_ts(ACCEPT_VIDEO_OFFSET, ost_time_base);

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

    Ok((
        ts_to_time(min_pts, ost_time_base),
        ts_to_time(max_pts, ost_time_base),
    ))
}
