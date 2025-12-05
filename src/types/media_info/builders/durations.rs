use super::MediaInfo;
use crate::{CacheState, Duration, Result, StreamType, ffmpeg, markers::*, types::helpers};
use std::{iter, path::Path};

impl MediaInfo<'_> {
    pub(crate) fn build_audio_duration(&mut self, src: &Path) -> Result<Duration> {
        self.try_cache_durations(src)?;
        self.try_get(MIAudioDuration, src).copied()
    }

    pub(crate) fn build_video_duration(&mut self, src: &Path) -> Result<Duration> {
        self.try_cache_durations(src)?;
        self.try_get(MIVideoDuration, src).copied()
    }

    pub(crate) fn build_playable_duration(&mut self, src: &Path) -> Result<Duration> {
        self.try_cache_durations(src)?;
        self.try_get(MIPlayableDuration, src).copied()
    }

    fn try_cache_durations(&mut self, src: &Path) -> Result<()> {
        let _ = self.try_init(MIStreams, src)?;
        let cache = self.try_immut(MICache, src)?;

        let a = StreamType::Audio;
        let v = StreamType::Video;
        let durations = rayon::join(
            || (!cache.audio_duration.is_cached()).then(|| try_duration(self, src, a)),
            || (!cache.video_duration.is_cached()).then(|| try_duration(self, src, v)),
        );

        let cache = self.cache.of_files.get_mut(src).unwrap();

        if let Some(res) = durations.0 {
            cache.audio_duration = CacheState::from_res(res);
        }
        if let Some(res) = durations.1 {
            cache.video_duration = CacheState::from_res(res);
        }

        let audio = self.immut(MIAudioDuration, src).copied();
        let video = self.immut(MIVideoDuration, src).copied();

        // The playable duration is the longest duration of any video or audio track,
        // not a subtitle track.
        let playable = audio
            .into_iter()
            .chain(video)
            .max()
            .ok_or_else(|| "Not found playable time".into());

        let cache = self.cache.of_files.get_mut(src).unwrap();
        cache.playable_duration = CacheState::from_res(playable);

        Ok(())
    }
}

fn try_duration(mi: &MediaInfo<'_>, src: &Path, ty: StreamType) -> Result<Duration> {
    const BASE: i64 = ffmpeg::ffi::AV_TIME_BASE as i64;

    let streams = mi.try_immut(MIStreams, src)?;
    let mut ictx = ffmpeg::format::input(src)?;
    let mut duration = (0i64, 0i64, 0f64);

    let ictx_dur = ictx.duration();
    let seek_targets = [
        ictx_dur - 30 * BASE,
        ictx_dur - 120 * BASE,
        ictx_dur / 2,
        0i64,
    ];

    streams.iter().filter(|s| s.ty == ty).for_each(|s| {
        let stream = some_or!(ictx.streams().skip(s.i).next(), return);
        let mut opened = some_or!(helpers::try_ffmpeg_opened(ty, &stream).ok(), return);
        let base = helpers::ffmpeg_stream_time_base(&stream);

        seek_targets.iter().for_each(|seek| {
            if duration.0 != 0 {
                return;
            }
            some_or!(ictx.seek(*seek, ..).ok(), return);
            opened.flush();

            for (stream, packet) in ictx.packets() {
                if stream.index() != s.i {
                    continue;
                }
                some_or!(opened.send_packet(&packet).ok(), return);

                let iter = if ty.is_audio() {
                    pts_iter_audio(&mut opened)
                } else {
                    pts_iter_video(&mut opened)
                };

                for time in iter {
                    if time > duration.0 {
                        duration.0 = time;
                        duration.1 = packet.duration();
                        duration.2 = base;
                    }
                }
            }
        })
    });

    return if duration.0 != 0 {
        let secs = (duration.0 as f64 + duration.1 as f64) * duration.2;
        Ok(Duration::from_secs_f64(secs))
    } else {
        Err(err!("Fail get duration"))
    };
}

macro_rules! pts_iter {
    ($fn:ident, $frame:ident) => {
        fn $fn(opened: &mut ffmpeg::decoder::Opened) -> Box<dyn Iterator<Item = i64> + '_> {
            let mut frame = ffmpeg::util::frame::$frame::empty();

            Box::new(iter::from_fn(move || {
                loop {
                    match opened.receive_frame(&mut frame) {
                        Ok(_) => {
                            if let Some(pts) = frame.pts() {
                                return Some(pts);
                            }
                        }
                        Err(_) => return None,
                    }
                }
            }))
        }
    };
}

pts_iter!(pts_iter_audio, Audio);
pts_iter!(pts_iter_video, Video);
