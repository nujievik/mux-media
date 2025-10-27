mod name_tail_rel_fall;
mod saved;

use super::MediaInfo;
use crate::{
    CacheMIOfFileAttach, CacheMIOfFileTrack, CacheState, Duration, EXTENSIONS, LangCode,
    RawTrackCache, Result, SubCharset, Target, TargetGroup, Tool, TrackID, TrackOrder, TrackType,
    ffmpeg, immut,
    markers::{
        MIAudioDuration, MIMatroska, MIMkvmergeI, MIPlayableDuration, MITICache, MITILang,
        MITargetGroup, MITracksInfo, MIVideoDuration,
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

static REGEX_ATTACH_ID: &Lazy<Regex> = regex!(r"Attachment ID (\d+):");
static REGEX_CODEC: &Lazy<Regex> = regex!(r"Track ID\s*\d+:\s*.*?\(([^)]+)\)");
static REGEX_TRACK_ID: &Lazy<Regex> = regex!(r"Track ID (\d+):");
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

    pub(super) fn build_mkvmerge_i(&self, media: &Path) -> Result<Vec<String>> {
        let args = &[Path::new("-i"), media];
        let out = self.tools.run(Tool::Mkvmerge, args)?;
        let out = out.as_str_stdout().lines().map(String::from).collect();
        Ok(out)
    }

    pub(super) fn build_sub_charset(&self, media: &Path) -> Result<SubCharset> {
        SubCharset::try_from(media)
    }

    pub(super) fn build_target_group(&mut self, media: &Path) -> Result<TargetGroup> {
        let map = self.try_get::<MITracksInfo>(media)?;

        TrackType::iter_customizable()
            .find_map(|ty| {
                map.iter()
                    .find(|(_, cache)| ty == cache.track_type)
                    .and_then(|_| TargetGroup::try_from(ty).ok())
            })
            .ok_or_else(|| "Not found any Media Track".into())
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

            if !tracks.values().any(|cache| ty == cache.track_type) {
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

    pub(super) fn build_tracks_info(
        &mut self,
        media: &Path,
    ) -> Result<HashMap<u64, CacheMIOfFileTrack>> {
        if let Some(matroska) = self.get::<MIMatroska>(media) {
            let map = matroska
                .tracks
                .iter()
                .map(|track| {
                    // track num in Matroska has 1-based indexing
                    // crate is used 0-based indexing
                    let num = track.number - 1;
                    CacheMIOfFileTrack::try_from(track).map(|cache| (num, cache))
                })
                .collect::<Result<HashMap<_, _>>>()?;

            return Ok(map);
        }

        let mkvmerge_i = immut!(@try, self, MIMkvmergeI, media)?;

        let map: HashMap<u64, CacheMIOfFileTrack> = mkvmerge_i
            .iter()
            .filter_map(|line| {
                REGEX_TRACK_ID.captures(line).and_then(|caps| {
                    caps.get(1)?
                        .as_str()
                        .parse::<u64>()
                        .ok()
                        .map(|num| (num, line.to_owned()))
                })
            })
            .map(|(num, line)| CacheMIOfFileTrack::try_from(line).map(|cache| (num, cache)))
            .collect::<Result<_>>()?;

        Ok(map)
    }

    pub(super) fn build_attachs_info(
        &mut self,
        media: &Path,
    ) -> Result<HashMap<u64, CacheMIOfFileAttach>> {
        if let Some(matroska) = self.get::<MIMatroska>(media) {
            let map: HashMap<u64, CacheMIOfFileAttach> = matroska
                .attachments
                .iter()
                .enumerate()
                .filter_map(|(i, raw)| {
                    // attach nums in Matroska has 1-based indexing
                    let num = i as u64 + 1;
                    match CacheMIOfFileAttach::try_init(num, &raw.mime_type) {
                        Ok(cache) => Some((num, cache)),
                        Err(_) => None,
                    }
                })
                .collect();

            return Ok(map);
        }

        let mkvmerge_i = immut!(@try, self, MIMkvmergeI, media)?;

        let map: HashMap<u64, CacheMIOfFileAttach> = mkvmerge_i
            .into_iter()
            .filter_map(|line| {
                REGEX_ATTACH_ID.captures(line).and_then(|caps| {
                    let num = caps.get(1)?.as_str().parse::<u64>().ok()?;
                    let cache = CacheMIOfFileAttach::try_init(num, line).ok()?;
                    Some((num, cache))
                })
            })
            .collect();

        Ok(map)
    }

    pub(super) fn build_ti_track_ids(&mut self, media: &Path, track: u64) -> Result<[TrackID; 2]> {
        let lang = self
            .get_ti::<MITILang>(media, track)
            .map(|val| val.deref())
            .unwrap_or(&LangCode::Und);

        Ok([TrackID::Num(track), TrackID::Lang(*lang)])
    }

    pub(super) fn build_ti_codec(&mut self, media: &Path, track: u64) -> Result<String> {
        let cache = immut!(@try, self, MITICache, media, track)?;
        match &cache.raw {
            RawTrackCache::Matroska(raw) => Ok(raw.codec_id.clone()),
            RawTrackCache::Mkvmerge(raw) => REGEX_CODEC
                .captures(raw)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_owned()))
                .ok_or_else(|| err!("Not found codec string")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_a_regex_id {
        ($fn:ident, $re:ident, $begin_pattern: expr) => {
            #[test]
            fn $fn() {
                [
                    vec!["1", "2", "3", "4"],
                    vec!["4", "2", "3", "1"],
                    vec!["10", "99", "100", "999", "1000", "9999"],
                ]
                .into_iter()
                .for_each(|s_aids| {
                    let compare_with = |sep: &str| {
                        let s: String = s_aids
                            .iter()
                            .map(|s| format!("{} {}:", $begin_pattern, s))
                            .collect::<Vec<_>>()
                            .join(sep);

                        let extracted: Vec<&str> = $re
                            .captures_iter(&s)
                            .map(|m| m.get(1).unwrap().as_str())
                            .collect();

                        assert_eq!(s_aids, extracted);
                    };

                    compare_with("\n");
                    compare_with(" ");
                    compare_with("abc");
                    compare_with("ID");
                    compare_with("12345");
                })
            }
        };
    }

    test_a_regex_id!(test_regex_attach_id, REGEX_ATTACH_ID, "Attachment ID");
    test_a_regex_id!(test_regex_track_id, REGEX_TRACK_ID, "Track ID");

    #[test]
    fn test_regex_word() {
        [
            vec!["ab", "c", "def", "xyz"],
            vec!["def", "xyz", "ab", "c"],
            vec!["AB", "C", "dEf", "XYZ"],
            vec!["аб", "в", "где", "эюя"],
            vec!["АБ", "В", "ГдЕ", "ЭЮЯ"],
            vec!["ё", "Ё"],
        ]
        .into_iter()
        .for_each(|s_words| {
            let compare_with = |sep: &str| {
                let s: String = s_words.join(sep);
                let extracted: Vec<&str> = REGEX_WORD.find_iter(&s).map(|m| m.as_str()).collect();
                assert_eq!(s_words, extracted);
            };

            compare_with("\n");
            compare_with(" ");
            compare_with(".");
            compare_with(",");
            compare_with(":");
            compare_with("123");
        })
    }

    #[test]
    fn test_regex_codec() {
        [
            "ab",
            "c",
            "def",
            "xyz",
            "def",
            "xyz",
            "ab",
            "c",
            "AAC",
            "AC-3",
            "AVC/H.264/MPEG-4p10",
            "A_AC3",
            "A_VORBIS",
            "MP3",
            "V_MPEG4/ISO/AVC",
            "Vorbis",
        ]
        .into_iter()
        .for_each(|codec| {
            let compare_with = |track: &str, add: &str| {
                let s = format!("Track ID {}:{}({})", track, add, codec);
                let extracted = REGEX_CODEC.captures(&s).unwrap().get(1).unwrap().as_str();

                assert_eq!(codec, extracted);
            };

            ["1", "2", "3", "4", "999", "1000", "9999"]
                .iter()
                .for_each(|track| {
                    compare_with(track, "\n");
                    compare_with(track, " ");
                    compare_with(track, "");
                    compare_with(track, ".");
                    compare_with(track, ",");
                    compare_with(track, ":");
                    compare_with(track, "123");
                })
        })
    }
}
