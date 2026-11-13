use super::MediaInfoCacheOfFile;
use crate::ffmpeg::{self, Rational, Rescale};
use crate::{NANOSECOND_TIME_BASE, Result, StreamType};
use std::path::Path;
use subtitle_lines::Time;

#[derive(Clone, Debug, PartialEq)]
pub struct VideoAndAudioDurations {
    value: DurationsValue,
}
#[derive(Clone, Debug, PartialEq)]
enum DurationsValue {
    AudioOnly(Time),
    VideoOnly(Time),
    VideoAndAudio((Time, Time)),
}
use DurationsValue::*;

impl VideoAndAudioDurations {
    pub(crate) fn try_from_streams(
        src: &Path,
        cache: &mut MediaInfoCacheOfFile,
    ) -> Result<VideoAndAudioDurations> {
        value_from_streams(src, cache).map(|value| VideoAndAudioDurations { value })
    }

    #[inline]
    pub const fn get_video(&self) -> Option<Time> {
        match self.value {
            VideoOnly(t) | VideoAndAudio((t, _)) => Some(t),
            AudioOnly(_) => None,
        }
    }

    #[inline]
    pub const fn get_audio(&self) -> Option<Time> {
        match self.value {
            AudioOnly(t) | VideoAndAudio((_, t)) => Some(t),
            VideoOnly(_) => None,
        }
    }

    /// Returns maximum video or audio duration.
    #[inline]
    pub fn max(&self) -> Time {
        match self.value {
            AudioOnly(t) | VideoOnly(t) => t,
            VideoAndAudio((t1, t2)) => t1.max(t2),
        }
    }
}

fn value_from_streams(src: &Path, cache: &mut MediaInfoCacheOfFile) -> Result<DurationsValue> {
    let streams = cache.caching_streams(src)?;
    let mut ictx = ffmpeg::format::input(src)?;

    let mut is_has_video = false;
    let mut is_has_audio = false;

    let time_bases: Vec<_> = streams
        .iter()
        .map(|st| {
            let is_has = match st.ty {
                StreamType::Video => &mut is_has_video,
                StreamType::Audio => &mut is_has_audio,
                _ => return None,
            };
            let ist = ictx.stream(st.i)?;
            *is_has = true;
            Some(ist.time_base())
        })
        .collect();

    if !is_has_video && !is_has_audio {
        return Err(err!("not found a video or audio stream"));
    }

    let mut a_dur = (i64::MIN, 0i64, NANOSECOND_TIME_BASE);
    let mut v_dur = (i64::MIN, 0i64, NANOSECOND_TIME_BASE);

    let ictx_dur = ictx.duration();

    let seek_targets = [
        ictx_dur,
        ictx_dur - (ictx_dur / 50),
        ictx_dur - (ictx_dur / 10),
        ictx_dur / 2,
        0i64,
    ];

    for seek in seek_targets {
        some_or!(continue; ictx.seek(seek, ..).ok());

        for (st, packet) in ictx.packets() {
            let i = st.index();
            let tb = some_or!(continue; time_bases[i]);

            let pts = match packet.pts().or_else(|| packet.dts()) {
                Some(ts) => ts,
                None => continue,
            };

            let dur = match streams[i].ty {
                StreamType::Video => &mut v_dur,
                StreamType::Audio => &mut a_dur,
                _ => unreachable!(), // time bases filters video and audio only
            };

            let rescaled_pts = pts.rescale(tb, NANOSECOND_TIME_BASE);

            if rescaled_pts > dur.0 {
                dur.0 = rescaled_pts;
                dur.1 = packet.duration();
                dur.2 = tb;
            }
        }

        match (v_dur.0, a_dur.0) {
            (i64::MIN, i64::MIN) => (),
            (_, i64::MIN) if is_has_audio => (),
            (_, i64::MIN) => return Ok(VideoOnly(dur_to_time(v_dur))),

            (i64::MIN, _) if is_has_video => (),
            (i64::MIN, _) => return Ok(AudioOnly(dur_to_time(a_dur))),

            (_, _) => return Ok(VideoAndAudio((dur_to_time(v_dur), dur_to_time(a_dur)))),
        }
    }

    Err(err!("fail get video or audio duration"))
}

fn dur_to_time(dur: (i64, i64, Rational)) -> Time {
    let duration = dur.1.rescale(dur.2, NANOSECOND_TIME_BASE);

    // dur.0 already rescaled to NANOSECOND_TIME_BASE
    // + 500_000 required for round up
    let millis = (dur.0 + duration + 500_000) as u64 / 1_000_000;
    Time::from_millis(millis)
}
