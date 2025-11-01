mod name_tail_rel_fall;
mod saved;

use super::MediaInfo;
use crate::{
    AttachType, CacheMIOfFileAttach, CacheMIOfFileTrack, CacheState, CodecID, Duration, EXTENSIONS,
    FfmpegStream, LangCode, Result, SubCharset, Target, TargetGroup, TrackID, TrackOrder,
    TrackType, ffmpeg,
    markers::{
        MIAudioDuration, MICache, MIFfmpegStreams, MIPlayableDuration, MITILang, MITargetGroup,
        MITracksInfo, MIVideoDuration,
    },
    types::helpers::{ffmpeg_stream_i_tb, try_ffmpeg_opened},
};
use lazy_regex::{Lazy, Regex, regex};
use matroska::Matroska;
use std::{
    collections::HashMap,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};

static REGEX_WORD: &Lazy<Regex> = regex!(r"[a-zA-Z]+|[а-яА-ЯёЁ]+");

impl MediaInfo<'_> {
    pub(crate) fn build_external_fonts(&self) -> Result<Arc<Vec<PathBuf>>> {
        Ok(Arc::new(
            self.cfg.input.collect_fonts_with_filter_and_sort(),
        ))
    }

    pub(super) fn build_stem(&self) -> Result<OsString> {
        let shortest = self
            .cache
            .of_files
            .iter()
            .filter_map(|(p, _)| p.file_stem())
            .min_by_key(|s| s.len())
            .ok_or("Not found any file_stem()")?;

        Ok(shortest.to_owned())
    }

    pub(super) fn build_track_order(&mut self) -> Result<TrackOrder> {
        TrackOrder::try_from(self)
    }

    pub(super) fn build_matroska(&self, media: &Path) -> Result<Matroska> {
        Self::help_build_matroska(media)
    }

    pub(crate) fn help_build_matroska(media: &Path) -> Result<Matroska> {
        if !media.extension().map_or(false, |ext| {
            EXTENSIONS.matroska.contains(ext.as_encoded_bytes())
        }) {
            return Err(format!("File '{}' not has Matroska extension", media.display()).into());
        }

        matroska::open(media).map_err(|e| e.into())
    }

    pub(super) fn build_ffmpeg_streams(&self, media: &Path) -> Result<Vec<FfmpegStream>> {
        let ictx = ffmpeg::format::input(media)?;
        let streams = ictx
            .streams()
            .map(|stream| {
                let p = stream.parameters();
                let ty = p.medium();
                let id = p.id();
                FfmpegStream { ty, id }
            })
            .collect();
        Ok(streams)
    }

    pub(super) fn build_sub_charset(&self, media: &Path) -> Result<SubCharset> {
        SubCharset::try_from(media)
    }

    pub(super) fn build_target_group(&mut self, media: &Path) -> Result<TargetGroup> {
        let map = self.try_get::<MITracksInfo>(media)?;

        TrackType::iter_customizable()
            .find_map(|ty| {
                map.iter()
                    .find(|(_, cache)| ty == cache.ty)
                    .and_then(|_| TargetGroup::try_from(ty).ok())
            })
            .ok_or_else(|| "Not found customizable media track".into())
    }

    pub(super) fn build_targets(&mut self, media: &Path) -> Result<Vec<Target>> {
        let mut targets = Vec::<Target>::new();

        if let Some(trg) = self.cfg.get_key(media) {
            targets.push(trg);
        }

        if let Some(trg) = media.parent().and_then(|p| self.cfg.get_key(p)) {
            targets.push(trg);
        }

        if let Some(&group) = self.get::<MITargetGroup>(media) {
            if let Some(trg) = self.cfg.get_key(group) {
                targets.push(trg);
            }
        }

        Ok(targets)
    }

    fn try_cache_durations(&mut self, media: &Path) -> Result<()> {
        let _ = self.try_init::<MITracksInfo>(media)?;
        let cache = self.cache.of_files.get(media).unwrap();

        let a = TrackType::Audio;
        let v = TrackType::Video;
        let durations = rayon::join(
            || (!cache.audio_duration.is_cached()).then(|| try_duration(self, media, a)),
            || (!cache.video_duration.is_cached()).then(|| try_duration(self, media, v)),
        );

        let cache = self.cache.of_files.get_mut(media).unwrap();

        if let Some(res) = durations.0 {
            cache.audio_duration = CacheState::from_res(res);
        }
        if let Some(res) = durations.1 {
            cache.video_duration = CacheState::from_res(res);
        }

        let audio = self.immut::<MIAudioDuration>(media).copied();
        let video = self.immut::<MIVideoDuration>(media).copied();

        // The playable duration is the longest duration of any video or audio track,
        // not a subtitle track.
        let playable = audio
            .into_iter()
            .chain(video)
            .max()
            .ok_or_else(|| "Not found playable time".into());

        let cache = self.cache.of_files.get_mut(media).unwrap();
        cache.playable_duration = CacheState::from_res(playable);

        return Ok(());

        fn try_duration(mi: &MediaInfo<'_>, media: &Path, ty: TrackType) -> Result<Duration> {
            let tracks = mi.try_immut::<MITracksInfo>(media)?;

            if !tracks.values().any(|cache| ty == cache.ty) {
                return Err(err!("Not found any '{}' track", ty.as_str_mkvtoolnix()));
            }

            let mut dur = 0f64;
            let mty = ty.as_ffmpeg_mty();
            let mut ictx = ffmpeg::format::input(media)?;

            for i in collect_stream_idxs(&ictx) {
                let stream = match ictx.stream(i) {
                    Some(s) => s,
                    None => continue,
                };
                if mty != stream.parameters().medium() {
                    continue;
                }

                let (i, tb) = ffmpeg_stream_i_tb(&stream);
                let mut opened = try_ffmpeg_opened(ty, &stream)?;

                let seek_target = (99999999999.0 * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
                ictx.seek(seek_target, ..)?;
                opened.flush();

                for (s, packet) in ictx.packets() {
                    if s.index() != i {
                        continue;
                    }

                    opened.send_packet(&packet)?;
                    let mut frame = ffmpeg::util::frame::Video::empty();

                    while let Ok(_) = opened.receive_frame(&mut frame) {
                        let pts_time = frame.pts().map(|pts| pts as f64 * tb).unwrap_or(0.0);
                        if pts_time > dur {
                            dur = pts_time;
                        }
                    }
                }
            }

            return if dur > 0.0 {
                Ok(Duration::from_secs_f64(dur))
            } else {
                Err(err!("Not found duration"))
            };

            fn collect_stream_idxs(ictx: &ffmpeg::format::context::Input) -> Vec<usize> {
                ictx.streams().map(|s| s.index()).collect()
            }
        }
    }

    pub(super) fn build_audio_duration(&mut self, media: &Path) -> Result<Duration> {
        self.try_cache_durations(media)?;
        let d = *self.try_get::<MIAudioDuration>(media)?;
        Ok(d)
    }

    pub(super) fn build_video_duration(&mut self, media: &Path) -> Result<Duration> {
        self.try_cache_durations(media)?;
        let d = *self.try_get::<MIVideoDuration>(media)?;
        Ok(d)
    }

    pub(super) fn build_playable_duration(&mut self, media: &Path) -> Result<Duration> {
        self.try_cache_durations(media)?;
        let d = *self.try_get::<MIPlayableDuration>(media)?;
        Ok(d)
    }

    fn help_build_tracks_attachs_info(
        &mut self,
        media: &Path,
        need_tracks: bool,
    ) -> Result<(
        Option<HashMap<u64, CacheMIOfFileTrack>>,
        Option<HashMap<u64, CacheMIOfFileAttach>>,
    )> {
        use crate::ffmpeg::util::media::Type;

        let streams = self.try_get::<MIFfmpegStreams>(media)?;

        let mut num_track = 0u64;
        let mut num_attach = 1u64;
        let mut tracks = HashMap::new();
        let mut attachs = HashMap::new();

        let mut push_track = |stream_i, codec_id, ty| {
            let v = CacheMIOfFileTrack {
                stream_i,
                codec_id,
                ty,
                ..Default::default()
            };
            let _ = tracks.insert(num_track, v);
            num_track += 1;
        };
        let mut push_attach = |stream_i, codec_id, ty| {
            let v = CacheMIOfFileAttach {
                stream_i,
                codec_id,
                ty,
            };
            let _ = attachs.insert(num_attach, v);
            num_attach += 1;
        };

        streams.iter().enumerate().for_each(|(i, stream)| {
            let id = CodecID(stream.id);
            match stream.ty {
                Type::Audio => push_track(i, id, TrackType::Audio),
                Type::Subtitle => push_track(i, id, TrackType::Sub),
                Type::Video if id.is_attach_other() => push_attach(i, id, AttachType::Other),
                Type::Video => push_track(i, id, TrackType::Video),
                Type::Attachment if id.is_font() => push_attach(i, id, AttachType::Font),
                Type::Attachment => push_attach(i, id, AttachType::Other),
                _ => push_track(i, id, TrackType::NonCustomizable),
            }
        });

        let cache = self.try_mut::<MICache>(media)?;

        if need_tracks {
            cache.attachs_info = CacheState::Cached(attachs);
            Ok((Some(tracks), None))
        } else {
            cache.tracks_info = CacheState::Cached(tracks);
            Ok((None, Some(attachs)))
        }
    }

    pub(super) fn build_tracks_info(
        &mut self,
        media: &Path,
    ) -> Result<HashMap<u64, CacheMIOfFileTrack>> {
        self.help_build_tracks_attachs_info(media, true)
            .map(|v| v.0.unwrap())
    }

    pub(super) fn build_attachs_info(
        &mut self,
        media: &Path,
    ) -> Result<HashMap<u64, CacheMIOfFileAttach>> {
        self.help_build_tracks_attachs_info(media, false)
            .map(|v| v.1.unwrap())
    }

    pub(super) fn build_ti_track_ids(&mut self, media: &Path, track: u64) -> Result<[TrackID; 2]> {
        let lang = self
            .get_ti::<MITILang>(media, track)
            .map(|val| val.deref())
            .unwrap_or(&LangCode::Und);

        Ok([TrackID::Num(track), TrackID::Lang(*lang)])
    }
}
