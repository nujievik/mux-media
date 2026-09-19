use super::*;
use crate::ffmpeg::{self, Rescale};
use crate::{CacheState, Result, StreamType, Time, types::helpers};
use std::{iter, path::Path};

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

        let audio = self.immut(MarkMediaInfoAudioDuration, src).copied();
        let video = self.immut(MarkMediaInfoVideoDuration, src).copied();

        // The playable duration is the longest duration of any video or audio track,
        // not a subtitle track.
        let playable = audio
            .into_iter()
            .chain(video)
            .max()
            .ok_or_else(|| err!("Not found playable time"));

        let cache = self.cache.of_files.get_mut(src).unwrap();
        cache.playable_duration = CacheState::from_res(playable);

        Ok(())
    }
}

fn try_duration(mi: &MediaInfo<'_>, src: &Path, ty: StreamType) -> Result<Time> {
    const BASE: i64 = ffmpeg::ffi::AV_TIME_BASE as i64;
    const MILLISECOND_TIME_BASE: ffmpeg::Rational = ffmpeg::Rational(1, 1000);

    let streams = mi.try_immut(MarkMediaInfoStreams, src)?;
    let mut ictx = ffmpeg::format::input(src)?;
    let mut duration = (0i64, 0i64, MILLISECOND_TIME_BASE);

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
        let time_base = stream.time_base();

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
                        duration.2 = time_base;
                    }
                }
            }
        })
    });

    return if duration.0 != 0 {
        let millis = (duration.0 + duration.1).rescale(duration.2, MILLISECOND_TIME_BASE);
        Ok(Time::from_millis(millis as u64))
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
