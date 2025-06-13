//mod aids;
mod tids;

use super::mkvinfo::{MKVILang, MKVIName, Mkvinfo};
use super::{AICache, MediaInfo, TICache};
use crate::{
    AttachID, EXTENSIONS, LangCode, MIMkvinfo, MIMkvmergeI, MIPathTail, MIRelativeUpmost, MITIName,
    MITargetGroup, MITracksInfo, MuxError, Target, TargetGroup, Tool, TrackID, TrackType,
    os_helpers,
};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

const READ_LIMIT: usize = 32 * 1024; // 32 KiB

macro_rules! from_name_tail_relative_or_fallback {
    ($mi:ident, $path:ident, $tid:ident, $typ:ident, $try_str:ident, $try_slice_string:ident, $dflt:ident) => {
        $mi.try_get_ti::<MITIName>($path, $tid)
            .ok()
            .and_then(|name| $typ::$try_str(name.to_lowercase().as_ref()).ok())
            .or_else(|| {
                $mi.try_get::<MIPathTail>($path)
                    .ok()
                    .and_then(|s| $typ::$try_slice_string(str_to_words(&s).as_slice()).ok())
            })
            .or_else(|| {
                $mi.try_get::<MIRelativeUpmost>($path)
                    .ok()
                    .and_then(|s| $typ::$try_slice_string(str_to_words(&s).as_slice()).ok())
            })
            .unwrap_or($typ::$dflt)
    };
}

impl MediaInfo<'_> {
    pub(super) fn build_char_encoding(&mut self, path: &Path) -> Result<String, MuxError> {
        let enc = if path.extension().map_or(false, |ext| {
            EXTENSIONS.matroska.contains(ext.as_encoded_bytes())
        }) {
            // All text in a Matroska(tm) file is encoded in UTF-8
            "utf-8".to_string()
        } else {
            let bytes = read_limited(path, READ_LIMIT)?;
            let detected = chardet::detect(&bytes);
            detected.0
        };
        Ok(enc)
    }

    pub(super) fn build_target_group(&mut self, path: &Path) -> Result<TargetGroup, MuxError> {
        let tid_types: Vec<(TrackID, TrackType)> = self
            .try_get::<MITracksInfo>(path)?
            .iter()
            .map(|(tid, cache)| (*tid, cache.track_type))
            .collect();

        for tt in [TrackType::Video, TrackType::Audio, TrackType::Sub] {
            if let Some(&(tid, _)) = tid_types.iter().find(|(_, t)| *t == tt) {
                return Ok(if tt != TrackType::Sub {
                    tt.into()
                } else {
                    from_name_tail_relative_or_fallback!(
                        self,
                        path,
                        tid,
                        TargetGroup,
                        try_signs_from_str,
                        try_signs_from_slice_string,
                        Subs
                    )
                });
            }
        }

        Err("No found any media track".into())
    }

    pub(super) fn build_mkvinfo(&mut self, path: &Path) -> Result<Mkvinfo, MuxError> {
        if !path.extension().map_or(false, |ext| {
            EXTENSIONS.matroska.contains(ext.as_encoded_bytes())
        }) {
            return Err(format!("File '{}' not has Matroska extension", path.display()).into());
        }

        let stdout = self.tools.run(Tool::Mkvinfo, &[path], None)?;
        let mkvinfo: Mkvinfo = stdout.lines().map(String::from).collect::<Vec<_>>().into();

        Ok(mkvinfo)
    }

    pub(super) fn build_mkvmerge_i(&mut self, path: &Path) -> Result<Vec<String>, MuxError> {
        let args = &[OsStr::new("-i"), path.as_os_str()];
        let stdout = self.tools.run(Tool::Mkvmerge, args, None)?;
        let stdout = stdout.lines().map(String::from).collect();
        Ok(stdout)
    }

    pub(super) fn build_targets(&mut self, path: &Path) -> Result<[Target; 3], MuxError> {
        let group = Target::Group(*self.try_get::<MITargetGroup>(path)?);
        let parent = Target::Path(path.parent().unwrap_or(path).to_path_buf());
        let path = Target::Path(path.to_path_buf());
        Ok([path, parent, group])
    }

    pub(super) fn build_tracks_info(
        &mut self,
        path: &Path,
    ) -> Result<HashMap<TrackID, TICache>, MuxError> {
        let mkvmerge_i = self.try_get::<MIMkvmergeI>(path)?;
        let re = regex::Regex::new(r"Track ID (\d+):")?;

        let raw: Vec<(u32, String)> = mkvmerge_i
            .into_iter()
            .filter_map(|line| {
                re.captures(line).and_then(|caps| {
                    caps.get(1)?
                        .as_str()
                        .parse::<u32>()
                        .ok()
                        .map(|u32| (u32, line.to_string()))
                })
            })
            .collect();

        let mkvinfo = self.get::<MIMkvinfo>(path);
        let ti: HashMap<TrackID, TICache> = raw
            .into_iter()
            .map(|(u32, line)| {
                let id = TrackID::U32(u32);
                TICache::try_init(u32, line, mkvinfo).map(|cache| (id, cache))
            })
            .collect::<Result<_, _>>()?;

        Ok(ti)
    }

    pub(super) fn build_attachs_info(
        &mut self,
        path: &Path,
    ) -> Result<HashMap<AttachID, AICache>, MuxError> {
        let mkvmerge_i = self.try_get::<MIMkvmergeI>(path)?;
        let re = regex::Regex::new(r"Attachment ID (\d+):")?;

        let ai: HashMap<AttachID, AICache> = mkvmerge_i
            .into_iter()
            .filter_map(|line| {
                re.captures(line).and_then(|caps| {
                    let id = caps.get(1)?.as_str().parse::<u32>().ok()?;
                    let ai_cache = AICache::try_init(id, line.to_string()).ok()?;
                    Some((AttachID::U32(id), ai_cache))
                })
            })
            .collect();

        Ok(ai)
    }

    pub(super) fn build_path_tail(&mut self, path: &Path) -> Result<String, MuxError> {
        os_helpers::os_str_tail(self.stem.as_os_str(), path.as_os_str())
            .map(|os| os.to_string_lossy().into_owned())
    }

    pub(super) fn build_relative_upmost(&mut self, path: &Path) -> Result<String, MuxError> {
        path.parent()
            .ok_or(format!("Path '{}' has not parent directory", path.display()).into())
            .and_then(|parent| {
                os_helpers::os_str_tail(self.upmost.as_os_str(), parent.as_os_str())
                    .map(|os| os.to_string_lossy().into_owned())
            })
    }

    pub(super) fn build_ti_name(&mut self, path: &Path, tid: TrackID) -> Result<String, MuxError> {
        let tic = self
            .get_mut_ti_cache(path, &tid)
            .ok_or("Unexpected None TICache")?;

        let name = tic
            .mkvinfo_cutted
            .as_ref()
            .and_then(|mkvi| mkvi.get::<MKVIName>().cloned())
            .or_else(|| {
                self.try_get::<MIPathTail>(path)
                    .ok()
                    .filter(|s| s.len() > 2)
                    .cloned()
            })
            .or_else(|| {
                path.parent()
                    .and_then(|p| p.file_name())
                    .map(|p| p.to_string_lossy().into_owned())
            })
            .unwrap_or_default();

        Ok(name.clone())
    }

    pub(super) fn build_ti_lang(
        &mut self,
        path: &Path,
        tid: TrackID,
    ) -> Result<LangCode, MuxError> {
        let tic = self
            .get_mut_ti_cache(path, &tid)
            .ok_or("Unexpected None TICache")?;
        let lang = tic
            .mkvinfo_cutted
            .as_ref()
            .and_then(|mkvi| mkvi.get::<MKVILang>().copied())
            .unwrap_or_else(|| {
                use std::str::FromStr;
                from_name_tail_relative_or_fallback!(
                    self, path, tid, LangCode, from_str, try_from, Und
                )
            });
        Ok(lang)
    }
}

#[inline]
fn read_limited(path: &Path, max_bytes: usize) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = vec![0u8; max_bytes];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);
    Ok(buffer)
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
