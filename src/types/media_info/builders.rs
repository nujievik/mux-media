mod name_tail_rel_fall;
mod saved;

use super::MediaInfo;
use crate::{
    CacheMIOfFileAttach, CacheMIOfFileTrack, EXTENSIONS, MICmnRegexAID, MICmnRegexTID, MIMkvinfo,
    MIMkvmergeI, MITILang, MITargetGroup, Mkvinfo, MuxError, SubCharset, Target, Tool, TrackID,
};
use regex::Regex;
use smallvec::SmallVec;
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::Path,
    sync::Arc,
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

    pub(super) fn build_stem(&mut self) -> Result<Arc<OsString>, MuxError> {
        let shortest: &OsStr = self
            .cache
            .of_files
            .keys()
            .filter_map(|path| path.file_stem().filter(|s| !s.is_empty()))
            .min_by_key(|s| s.len())
            .ok_or("Not found any file_stem()")?;

        Ok(Arc::new(shortest.to_os_string()))
    }

    pub(super) fn build_sub_charset(&mut self, path: &Path) -> Result<SubCharset, MuxError> {
        path.try_into()
    }

    pub(super) fn build_mkvinfo(&mut self, path: &Path) -> Result<Mkvinfo, MuxError> {
        if !path.extension().map_or(false, |ext| {
            EXTENSIONS.matroska.contains(ext.as_encoded_bytes())
        }) {
            return Err(format!("File '{}' not has Matroska extension", path.display()).into());
        }

        let out = self.tools.run(Tool::Mkvinfo, &[path])?;
        let s: Vec<String> = out.as_str_stdout().lines().map(String::from).collect();

        Ok(s.into())
    }

    pub(super) fn build_mkvmerge_i(&mut self, path: &Path) -> Result<Vec<String>, MuxError> {
        let args = &[OsStr::new("-i"), path.as_os_str()];
        let out = self.tools.run(Tool::Mkvmerge, args)?;
        let out = out.as_str_stdout().lines().map(String::from).collect();
        Ok(out)
    }

    pub(super) fn build_targets(&mut self, path: &Path) -> Result<SmallVec<[Target; 3]>, MuxError> {
        let mut targets: SmallVec<[Target; 3]> = SmallVec::new();

        if let Some(trg) = self.mc.get_clone_target(path) {
            targets.push(trg);
        }

        if let Some(trg) = path.parent().and_then(|p| self.mc.get_clone_target(p)) {
            targets.push(trg);
        }

        if let Ok(&group) = self.try_get::<MITargetGroup>(path) {
            if let Some(trg) = self.mc.get_clone_target(group) {
                targets.push(trg);
            }
        }

        Ok(targets)
    }

    pub(super) fn build_tracks_info(
        &mut self,
        path: &Path,
    ) -> Result<HashMap<u64, CacheMIOfFileTrack>, MuxError> {
        let re = self.try_get_cmn::<MICmnRegexTID>()?.clone();
        let mkvmerge_i = self.try_get::<MIMkvmergeI>(path)?;

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

        let mkvinfo = self.get::<MIMkvinfo>(path);
        let ti: HashMap<u64, CacheMIOfFileTrack> = num_lines
            .into_iter()
            .map(|(num, line)| {
                CacheMIOfFileTrack::try_init(num, line, mkvinfo).map(|cache| (num, cache))
            })
            .collect::<Result<_, _>>()?;

        Ok(ti)
    }

    pub(super) fn build_attachs_info(
        &mut self,
        path: &Path,
    ) -> Result<HashMap<u64, CacheMIOfFileAttach>, MuxError> {
        let re = self.try_get_cmn::<MICmnRegexAID>()?.clone();
        let mkvmerge_i = self.try_get::<MIMkvmergeI>(path)?;

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
        path: &Path,
        num: u64,
    ) -> Result<[TrackID; 2], MuxError> {
        let lang = self.try_get_ti::<MITILang>(path, num)?;
        Ok([TrackID::Num(num), TrackID::Lang(*lang)])
    }
}
