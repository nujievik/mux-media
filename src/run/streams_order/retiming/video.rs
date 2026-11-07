use super::*;
use crate::Result;
use crate::ffmpeg::{Rescale, format};
use std::path::Path;

impl Retiming<'_, '_> {
    pub(super) fn try_video(&self, src: &Path, i_stream: usize) -> Result<RetimedStream> {
        if i_stream != self.i_base_stream && src != self.base.as_path() {
            return Err(err!(
                "unsupported retiming more than 1 video track at a time"
            ));
        }

        self.try_base_video()
    }

    pub(super) fn init_base_splits(&mut self) -> Result<()> {
        for (i, p) in self.parts.iter_mut().enumerate() {
            let split = self
                .temp_dir
                .join(format!("{}-vid-base-{}.mkv", self.job, i));

            let (start, end) = try_split(&p.src, self.i_base_stream, &split, p.start, p.end)?;

            p.start_offset += SignedTime::new(true, start) - p.start;
            p.end_offset += SignedTime::new(true, end) - p.end;
            p.start = start;
            p.end = end;

            self.base_splits.push(split);
        }
        Ok(())
    }

    fn try_base_video(&self) -> Result<RetimedStream> {
        let dest = self.temp_dir.join(format!("{}-vid-base.mkv", self.job));
        try_concat(&self.base, &self.base_splits, &dest)?;

        Ok(RetimedStream {
            src: dest,
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

    let accept = |time: Time| time_to_ts(time.saturating_sub(ACCEPT_VIDEO_OFFSET), ist_time_base);
    let accept_start = accept(trg_start);
    let accept_end = accept(trg_end);

    let rescale = |ts: i64| ts.rescale(ist_time_base, ost_time_base);

    let mut min_pts = None::<i64>;
    let mut max_pts: (i64, i64) = (i64::MIN, 0);

    for (ist, mut packet) in ictx.packets() {
        if ist.index() != i_stream {
            continue;
        }

        let is_key = packet.is_key();

        if min_pts.is_none() && !is_key {
            continue;
        }

        let pts = match packet.pts().or(packet.dts()) {
            Some(ts) => ts,
            None => return Err(err!("fail get packet pts")),
        };

        if min_pts.is_none() {
            if pts < accept_start {
                continue;
            } else {
                // start i-frame has lowest dts & pts
                min_pts = Some(pts);
            }
        }

        if pts >= max_pts.0 {
            max_pts.0 = pts;
            max_pts.1 = packet.duration();
        }

        let offset = min_pts.unwrap();
        let new_pts = packet.pts().map(|pts| rescale(pts - offset));
        let new_dts = packet.dts().map(|dts| rescale(dts - offset));

        packet.set_pts(new_pts);
        packet.set_dts(new_dts);
        packet.set_duration(rescale(packet.duration()));
        packet.set_stream(ost_index);

        packet.write(&mut octx)?;

        if is_key && pts >= accept_end {
            break;
        }
    }

    let min_pts = min_pts.ok_or_else(|| err!("not written a packet"))?;
    octx.write_trailer()?;

    Ok((
        ts_to_time(min_pts, ist_time_base),
        ts_to_time(max_pts.0 + max_pts.1, ist_time_base),
    ))
}
