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
    let streams = mi.try_immut(MIStreams, src)?;
    let mut ictx = ffmpeg::format::input(src)?;
    let mut duration: Option<f64> = None;

    streams.iter().filter(|s| s.ty == ty).for_each(|s| {
        let stream = some_or_return!(ictx.streams().skip(s.i).next());

        let (i, tb) = helpers::ffmpeg_stream_i_tb(&stream);
        let mut opened = some_or_return!(helpers::try_ffmpeg_opened(ty, &stream).ok());

        let seek_target = (99999999999.0 * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
        some_or_return!(ictx.seek(seek_target, ..).ok());
        opened.flush();

        for (s, packet) in ictx.packets() {
            if s.index() != i {
                continue;
            }
            some_or_return!(opened.send_packet(&packet).ok());

            let iter = if ty.is_audio() {
                pts_iter_audio(&mut opened)
            } else {
                pts_iter_video(&mut opened)
            };

            for time in iter.map(|t| t as f64 * tb) {
                match duration {
                    Some(d) if time > d => duration = Some(time),
                    None => duration = Some(time),
                    _ => (),
                }
            }
        }
    });

    return match duration {
        Some(d) => Ok(Duration::from_secs_f64(d)),
        None => Err(err!("Fail get duration")),
    };

    fn pts_iter_audio(opened: &mut ffmpeg::decoder::Opened) -> Box<dyn Iterator<Item = i64> + '_> {
        use ffmpeg::util::frame::Audio;
        let mut frame = Audio::empty();

        let iter = iter::from_fn(move || match opened.receive_frame(&mut frame) {
            Ok(_) => frame.pts(),
            Err(_) => None,
        });
        Box::new(iter)
    }

    fn pts_iter_video(opened: &mut ffmpeg::decoder::Opened) -> Box<dyn Iterator<Item = i64> + '_> {
        use ffmpeg::util::frame::Video;
        let mut frame = Video::empty();

        let iter = iter::from_fn(move || match opened.receive_frame(&mut frame) {
            Ok(_) => frame.pts(),
            Err(_) => None,
        });
        Box::new(iter)
    }
}
