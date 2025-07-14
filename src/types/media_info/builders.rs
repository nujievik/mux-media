mod saved;

use super::MediaInfo;
use super::cache::{CacheMIOfFileAttach, CacheMIOfFileTrack};
use super::mkvinfo::{MKVILang, MKVIName, Mkvinfo};
use crate::{
    EXTENSIONS, LangCode, MICmnStem, MIMkvinfo, MIMkvmergeI, MIPathTail, MIRelativeUpmost,
    MITILang, MITIName, MITargetGroup, MITracksInfo, MuxError, SubCharset, Target, TargetGroup,
    Tool, TrackID, TrackType, types::helpers::os_str_tail,
};
use smallvec::SmallVec;
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::Path,
    sync::Arc,
};

macro_rules! from_name_tail_relative_or_fallback {
    ($mi:ident, $path:ident, $num:expr, $typ:ident, $try_from_words:ident, $fall:ident) => {
        $mi.try_get_ti::<MITIName>($path, $num)
            .and_then(|name| $typ::$try_from_words(str_to_words(name).as_slice()))
            .or_else(|_| {
                $mi.try_get::<MIPathTail>($path)
                    .and_then(|s| $typ::$try_from_words(str_to_words(&s).as_slice()))
            })
            .or_else(|_| {
                $mi.try_get::<MIRelativeUpmost>($path)
                    .and_then(|s| $typ::$try_from_words(str_to_words(&s).as_slice()))
            })
            .unwrap_or($typ::$fall)
    };
}

impl MediaInfo<'_> {
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

    pub(super) fn build_target_group(&mut self, path: &Path) -> Result<TargetGroup, MuxError> {
        let num_types: Vec<(u64, TrackType)> = self
            .try_get::<MITracksInfo>(path)?
            .iter()
            .map(|(num, cache)| (*num, cache.track_type))
            .collect();

        for tt in [TrackType::Video, TrackType::Audio, TrackType::Sub] {
            if let Some(&(num, _)) = num_types.iter().find(|(_, t)| *t == tt) {
                return Ok(if tt != TrackType::Sub {
                    tt.into()
                } else {
                    from_name_tail_relative_or_fallback!(
                        self,
                        path,
                        num,
                        TargetGroup,
                        try_signs_from_slice_string,
                        Subs
                    )
                });
            }
        }

        Err("Not found any Media Track".into())
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
        let mkvmerge_i = self.try_get::<MIMkvmergeI>(path)?;
        let re = regex::Regex::new(r"Track ID (\d+):")?;

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
        let mkvmerge_i = self.try_get::<MIMkvmergeI>(path)?;
        let re = regex::Regex::new(r"Attachment ID (\d+):")?;

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

    pub(super) fn build_path_tail(&mut self, path: &Path) -> Result<String, MuxError> {
        let cmn_stem = self.try_get_cmn::<MICmnStem>()?;
        let stem = path
            .file_stem()
            .ok_or_else(|| format!("Path '{}' has not file_stem()", path.display()))?;
        os_str_tail(cmn_stem, stem).map(|os| os.to_string_lossy().into_owned())
    }

    pub(super) fn build_relative_upmost(&mut self, path: &Path) -> Result<String, MuxError> {
        path.parent()
            .ok_or_else(|| format!("Path '{}' has not parent()", path.display()).into())
            .and_then(|parent| {
                os_str_tail(self.upmost.as_os_str(), parent.as_os_str())
                    .map(|os| os.to_string_lossy().into_owned())
            })
    }

    pub(super) fn build_ti_name(&mut self, path: &Path, num: u64) -> Result<String, MuxError> {
        Ok(self
            .get_mut_track_cache(path, num)
            .and_then(|cache| {
                cache
                    .mkvinfo_cutted
                    .as_mut()
                    .and_then(|mkvi| mkvi.get::<MKVIName>().cloned())
            })
            .or_else(|| {
                self.try_get::<MIPathTail>(path).ok().and_then(|s| {
                    let s = s.trim_matches(&['.', ' ']);
                    (s.len() > 2).then(|| s.to_string())
                })
            })
            .or_else(|| {
                path.parent()
                    .and_then(|p| p.file_name())
                    .map(|p| p.to_string_lossy().into_owned())
            })
            .unwrap_or_default())
    }

    pub(super) fn build_ti_lang(&mut self, path: &Path, num: u64) -> Result<LangCode, MuxError> {
        Ok(self
            .get_mut_track_cache(path, num)
            .and_then(|cache| {
                cache
                    .mkvinfo_cutted
                    .as_mut()
                    .and_then(|mkvi| mkvi.get::<MKVILang>().copied())
            })
            .unwrap_or_else(|| {
                from_name_tail_relative_or_fallback!(self, path, num, LangCode, try_from, Und)
            }))
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

#[inline]
fn str_to_words(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|w| {
            w.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
        })
        .filter(|w| !w.is_empty())
        .collect()
}
