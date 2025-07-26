mod name_tail_rel_fall;
mod saved;

use super::MediaInfo;
use crate::{
    CacheMIOfFileAttach, CacheMIOfFileTrack, EXTENSIONS, MuxError, SubCharset, Target, Tool,
    TrackID,
    markers::{
        MICmnRegexAttachID, MICmnRegexTrackID, MIMatroska, MIMkvmergeI, MITILang, MITargetGroup,
    },
};
use matroska::Matroska;
use regex::Regex;
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::Path,
};

impl MediaInfo<'_> {
    pub(super) fn build_regex_aid(&mut self) -> Result<Regex, MuxError> {
        Regex::new(r"Attachment ID (\d+):").map_err(|e| e.into())
    }

    pub(super) fn build_regex_tid(&mut self) -> Result<Regex, MuxError> {
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

    pub(super) fn build_sub_charset(&mut self, media: &Path) -> Result<SubCharset, MuxError> {
        media.try_into()
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

        if let Ok(&group) = self.try_get::<MITargetGroup>(media) {
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
        if let Ok(matroska) = self.try_get::<MIMatroska>(media) {
            let map = matroska
                .tracks
                .clone()
                .into_iter()
                .map(|track| {
                    // track num in Matroska has 1-based indexing
                    // crate is used 0-based indexing
                    let num = track.number - 1;
                    CacheMIOfFileTrack::try_from(track).map(|cache| (num, cache))
                })
                .collect::<Result<HashMap<_, _>, _>>()?;

            return Ok(map);
        }

        let re = self.try_get_cmn::<MICmnRegexTrackID>()?.clone();
        let mkvmerge_i = self.try_get::<MIMkvmergeI>(media)?;

        let num_lines: Vec<(u64, String)> = mkvmerge_i
            .into_iter()
            .filter_map(|line| {
                re.captures(line).and_then(|caps| {
                    caps.get(1)?
                        .as_str()
                        .parse::<u64>()
                        .ok()
                        .map(|num| (num, line.to_string()))
                })
            })
            .collect();

        let map: HashMap<u64, CacheMIOfFileTrack> = num_lines
            .into_iter()
            .map(|(num, line)| CacheMIOfFileTrack::try_from(line).map(|cache| (num, cache)))
            .collect::<Result<_, _>>()?;

        Ok(map)
    }

    pub(super) fn build_attachs_info(
        &mut self,
        media: &Path,
    ) -> Result<HashMap<u64, CacheMIOfFileAttach>, MuxError> {
        let re = self.try_get_cmn::<MICmnRegexAttachID>()?.clone();
        let mkvmerge_i = self.try_get::<MIMkvmergeI>(media)?;

        let ai: HashMap<u64, CacheMIOfFileAttach> = mkvmerge_i
            .into_iter()
            .filter_map(|line| {
                re.captures(line).and_then(|caps| {
                    let num = caps.get(1)?.as_str().parse::<u64>().ok()?;
                    let cache = CacheMIOfFileAttach::try_init(num, line.to_string()).ok()?;
                    Some((num, cache))
                })
            })
            .collect();

        Ok(ai)
    }

    pub(super) fn build_ti_track_ids(
        &mut self,
        media: &Path,
        track: u64,
    ) -> Result<[TrackID; 2], MuxError> {
        let lang = self.try_get_ti::<MITILang>(media, track)?;
        Ok([TrackID::Num(track), TrackID::Lang(*lang)])
    }
}
