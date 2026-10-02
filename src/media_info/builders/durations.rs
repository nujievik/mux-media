use super::*;
use crate::ffmpeg::{self, Rescale};
use crate::{CacheState, Result, StreamType, Time};
use std::path::Path;

impl MediaInfo<'_> {
    pub(crate) fn build_audio_duration(&mut self, src: &Path) -> Result<Time> {
        self.try_cache_durations(src)?;
        self.try_get(MarkMediaInfoAudioDuration, src).copied()
    }

    pub(crate) fn build_video_duration(&mut self, src: &Path) -> Result<Time> {
        self.try_cache_durations(src)?;
        self.try_get(MarkMediaInfoVideoDuration, src).copied()
    }

    pub(crate) fn build_playable_duration(&mut self, src: &Path) -> Result<Time> {
        self.try_cache_durations(src)?;
        self.try_get(MarkMediaInfoPlayableDuration, src).copied()
    }

    fn try_cache_durations(&mut self, src: &Path) -> Result<()> {
        let _ = self.try_init(MarkMediaInfoStreams, src)?;
        let cache = self.try_immut(MarkMediaInfoCacheOfFile, src)?;

        if cache.playable_duration.is_cached() {
            return Ok(());
        }

        let (a_dur, v_dur) = match audio_video_duration(self, src) {
            Ok(av) => av,
            Err(_) => (
                Err(err!("fail get an audio time")),
                Err(err!("fail get a video time")),
            ),
        };

        // The playable duration is the longest duration of any video or audio track,
        // not a subtitle track.
        let playable_dur = match (&a_dur, &v_dur) {
            (Ok(a), Ok(v)) => Ok(*a.max(v)),
            (Ok(a), Err(_)) => Ok(*a),
            (Err(_), Ok(v)) => Ok(*v),
            (Err(_), Err(_)) => Err(err!("fail get a playable time")),
        };

        let cache = self.cache.of_files.get_mut(src).unwrap();
        cache.audio_duration = CacheState::convert_result(a_dur).0;
        cache.video_duration = CacheState::convert_result(v_dur).0;
        cache.playable_duration = CacheState::convert_result(playable_dur).0;

        Ok(())
    }
}

fn audio_video_duration(mi: &MediaInfo<'_>, src: &Path) -> Result<(Result<Time>, Result<Time>)> {
    const NANOSECOND_TIME_BASE: ffmpeg::Rational = ffmpeg::Rational(1, 1_000_000_000);

    let streams = mi.try_immut(MarkMediaInfoStreams, src)?;
    let mut ictx = ffmpeg::format::input(src)?;

    let time_bases: Vec<_> = streams
        .iter()
        .map(|st| {
            if !matches!(st.ty, StreamType::Audio | StreamType::Video) {
                return None;
            }
            let ictx_st = ictx.stream(st.i)?;
            Some(ictx_st.time_base())
        })
        .collect();

    let is_has_audio = streams.iter().any(|st| st.ty.is_audio());
    let is_has_video = streams.iter().any(|st| st.ty.is_video());

    let mut a_dur = (0i64, 0i64, NANOSECOND_TIME_BASE);
    let mut v_dur = (0i64, 0i64, NANOSECOND_TIME_BASE);

    let ictx_dur = ictx.duration();

    let seek_targets = [
        ictx_dur,
        ictx_dur - (ictx_dur / 50),
        ictx_dur - (ictx_dur / 10),
        ictx_dur / 2,
        0i64,
    ];

    if is_has_audio || is_has_video {
        for seek in seek_targets {
            some_or!(break; ictx.seek(seek, ..).ok());

            for (st, packet) in ictx.packets() {
                let i = st.index();
                let dur = match streams[i].ty {
                    StreamType::Audio => &mut a_dur,
                    StreamType::Video => &mut v_dur,
                    _ => continue,
                };

                let pts = match packet.pts().or_else(|| packet.dts()) {
                    Some(ts) => ts,
                    None => continue,
                };
                let tb = some_or!(continue; time_bases[i]);

                let rescaled_pts = pts.rescale(tb, NANOSECOND_TIME_BASE);

                if rescaled_pts > dur.0 {
                    dur.0 = rescaled_pts;
                    dur.1 = packet.duration();
                    dur.2 = tb;
                }
            }

            let is_audio_dur_zero = a_dur.0 == 0;
            let is_video_dur_zero = v_dur.0 == 0;

            if !is_audio_dur_zero {
                if !is_video_dur_zero || !is_has_video {
                    break;
                }
            }

            if !is_video_dur_zero {
                if !is_audio_dur_zero || !is_has_audio {
                    break;
                }
            }
        }
    }

    let res = |dur: (i64, i64, ffmpeg::Rational)| {
        if dur.0 <= 0 {
            return Err(err!("fail get duration"));
        }

        let duration = dur.1.rescale(dur.2, NANOSECOND_TIME_BASE);
        let millis = (dur.0 + duration + 500_000) as u64 / 1_000_000;

        Ok(Time::from_millis(millis))
    };

    Ok((res(a_dur), res(v_dur)))
}
