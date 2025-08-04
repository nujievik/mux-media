mod name_tail_rel_fall;
mod saved;

use super::MediaInfo;
use crate::{
    CacheMIOfFileAttach, CacheMIOfFileTrack, EXTENSIONS, LangCode, MuxError, RawTrackCache,
    SubCharset, Target, TargetGroup, Tool, TrackID, TrackOrder, TrackType, immut,
    markers::{
        MCInput, MICmnRegexAttachID, MICmnRegexCodec, MICmnRegexTrackID, MIMatroska, MIMkvmergeI,
        MITICache, MITILang, MITargetGroup, MITracksInfo,
    },
    mux_err,
};
use matroska::Matroska;
use regex::Regex;
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

impl MediaInfo<'_> {
    pub(super) fn build_external_fonts(&mut self) -> Result<Vec<PathBuf>, MuxError> {
        let input = self.mux_config.field::<MCInput>();
        Ok(input.collect_fonts_with_filter_and_sort())
    }

    pub(super) fn build_regex_attach_id(&mut self) -> Result<Regex, MuxError> {
        Regex::new(r"Attachment ID (\d+):").map_err(|e| e.into())
    }

    pub(super) fn build_regex_codec(&mut self) -> Result<Regex, MuxError> {
        Regex::new(r"Track ID\s*\d+:\s*.*?\(([^)]+)\)").map_err(|e| e.into())
    }

    pub(super) fn build_regex_track_id(&mut self) -> Result<Regex, MuxError> {
        Regex::new(r"Track ID (\d+):").map_err(|e| e.into())
    }

    pub(super) fn build_regex_word(&mut self) -> Result<Regex, MuxError> {
        Regex::new(r"[a-zA-Z]+|[а-яА-ЯёЁ]+").map_err(|e| e.into())
    }

    pub(super) fn build_stem(&mut self) -> Result<OsString, MuxError> {
        let shortest: &OsStr = self
            .cache
            .of_files
            .keys()
            .filter_map(|path| path.file_stem().filter(|s| !s.is_empty()))
            .min_by_key(|s| s.len())
            .ok_or("Not found any file_stem()")?;

        Ok(shortest.to_os_string())
    }

    pub(super) fn build_track_order(&mut self) -> Result<TrackOrder, MuxError> {
        TrackOrder::try_from(self)
    }

    pub(super) fn build_matroska(&mut self, media: &Path) -> Result<Matroska, MuxError> {
        if !media.extension().map_or(false, |ext| {
            EXTENSIONS.matroska.contains(ext.as_encoded_bytes())
        }) {
            return Err(format!("File '{}' not has Matroska extension", media.display()).into());
        }

        let matroska = matroska::open(media)?;

        Ok(matroska)
    }

    pub(super) fn build_mkvmerge_i(&mut self, media: &Path) -> Result<Vec<String>, MuxError> {
        let args = &[Path::new("-i"), media];
        let out = self.tools.run(Tool::Mkvmerge, args)?;
        let out = out.as_str_stdout().lines().map(String::from).collect();
        Ok(out)
    }

    pub(super) fn build_sub_charset(&mut self, media: &Path) -> Result<SubCharset, MuxError> {
        SubCharset::try_from(media)
    }

    pub(super) fn build_target_group(&mut self, media: &Path) -> Result<TargetGroup, MuxError> {
        let map = self.try_get::<MITracksInfo>(media)?;

        TrackType::iter_customizable()
            .find_map(|ty| {
                map.iter()
                    .find(|(_, cache)| ty == cache.track_type)
                    .and_then(|_| TargetGroup::try_from(ty).ok())
            })
            .ok_or_else(|| "Not found any Media Track".into())
    }

    pub(super) fn build_targets(&mut self, media: &Path) -> Result<Vec<Target>, MuxError> {
        let mut targets = Vec::<Target>::new();

        if let Some(trg) = self.mux_config.get_clone_target(media) {
            targets.push(trg);
        }

        if let Some(trg) = media
            .parent()
            .and_then(|p| self.mux_config.get_clone_target(p))
        {
            targets.push(trg);
        }

        if let Some(&group) = self.get::<MITargetGroup>(media) {
            if let Some(trg) = self.mux_config.get_clone_target(group) {
                targets.push(trg);
            }
        }

        Ok(targets)
    }

    pub(super) fn build_tracks_info(
        &mut self,
        media: &Path,
    ) -> Result<HashMap<u64, CacheMIOfFileTrack>, MuxError> {
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
                .collect::<Result<HashMap<_, _>, _>>()?;

            return Ok(map);
        }

        let _ = self.try_init_cmn::<MICmnRegexTrackID>()?;
        let mkvmerge_i = immut!(@try, self, MIMkvmergeI, media)?;
        let re = self.try_immut_cmn::<MICmnRegexTrackID>()?;

        let map: HashMap<u64, CacheMIOfFileTrack> = mkvmerge_i
            .iter()
            .filter_map(|line| {
                re.captures(line).and_then(|caps| {
                    caps.get(1)?
                        .as_str()
                        .parse::<u64>()
                        .ok()
                        .map(|num| (num, line.to_owned()))
                })
            })
            .map(|(num, line)| CacheMIOfFileTrack::try_from(line).map(|cache| (num, cache)))
            .collect::<Result<_, _>>()?;

        Ok(map)
    }

    pub(super) fn build_attachs_info(
        &mut self,
        media: &Path,
    ) -> Result<HashMap<u64, CacheMIOfFileAttach>, MuxError> {
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

        let _ = self.try_get_cmn::<MICmnRegexAttachID>()?;
        let mkvmerge_i = immut!(@try, self, MIMkvmergeI, media)?;
        let re = self.try_immut_cmn::<MICmnRegexAttachID>()?;

        let map: HashMap<u64, CacheMIOfFileAttach> = mkvmerge_i
            .into_iter()
            .filter_map(|line| {
                re.captures(line).and_then(|caps| {
                    let num = caps.get(1)?.as_str().parse::<u64>().ok()?;
                    let cache = CacheMIOfFileAttach::try_init(num, line).ok()?;
                    Some((num, cache))
                })
            })
            .collect();

        Ok(map)
    }

    pub(super) fn build_ti_track_ids(
        &mut self,
        media: &Path,
        track: u64,
    ) -> Result<[TrackID; 2], MuxError> {
        let lang = self
            .get_ti::<MITILang>(media, track)
            .map(|val| val.inner())
            .unwrap_or(&LangCode::Und);

        Ok([TrackID::Num(track), TrackID::Lang(*lang)])
    }

    pub(super) fn build_ti_codec(&mut self, media: &Path, track: u64) -> Result<String, MuxError> {
        let _ = self.try_init_ti::<MITICache>(media, track)?;
        let re = immut!(@try, self, MICmnRegexCodec);
        let cache = self.try_immut_ti::<MITICache>(media, track)?;

        match &cache.raw {
            RawTrackCache::Matroska(raw) => Ok(raw.codec_id.clone()),
            RawTrackCache::Mkvmerge(raw) => match re {
                Ok(re) => re
                    .captures(raw)
                    .and_then(|caps| caps.get(1).map(|m| m.as_str().to_owned()))
                    .ok_or_else(|| mux_err!("Not found codec string")),
                Err(e) => Err(e),
            },
        }
    }
}
