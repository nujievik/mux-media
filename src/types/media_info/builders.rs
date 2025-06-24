mod saved;

use super::MediaInfo;
use super::cache::{CacheMIOfFileAttach, CacheMIOfFileTrack};
use super::mkvinfo::{MKVILang, MKVIName, Mkvinfo};
use crate::{
    EXTENSIONS, LangCode, MICmnStem, MIMkvinfo, MIMkvmergeI, MIPathTail, MIRelativeUpmost,
    MITILang, MITIName, MITargetGroup, MITracksInfo, MuxError, Target, TargetGroup, Tool, TrackID,
    TrackType, os_helpers,
};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

const READ_LIMIT: usize = 32 * 1024; // 32 KiB

macro_rules! from_name_tail_relative_or_fallback {
    ($mi:ident, $path:ident, $num:expr, $typ:ident, $try_str:ident, $try_slice_string:ident, $fall:ident) => {
        $mi.try_get_ti::<MITIName>($path, $num)
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
            .unwrap_or($typ::$fall)
    };
}

impl MediaInfo<'_> {
    pub(super) fn build_stem(&mut self) -> Result<OsString, MuxError> {
        let shortest: &OsStr = self
            .cache
            .of_files
            .keys()
            .filter_map(|path| path.file_stem().filter(|s| !s.is_empty()))
            .min_by_key(|s| s.len())
            .ok_or("Not found any file_strm()")?;

        Ok(shortest.to_os_string())
    }

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
                        try_signs_from_str,
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
        os_helpers::os_str_tail(cmn_stem, stem).map(|os| os.to_string_lossy().into_owned())
    }

    pub(super) fn build_relative_upmost(&mut self, path: &Path) -> Result<String, MuxError> {
        path.parent()
            .ok_or_else(|| format!("Path '{}' has not parent()", path.display()).into())
            .and_then(|parent| {
                os_helpers::os_str_tail(self.upmost.as_os_str(), parent.as_os_str())
                    .map(|os| os.to_string_lossy().into_owned())
            })
    }

    pub(super) fn build_ti_name(&mut self, path: &Path, num: u64) -> Result<String, MuxError> {
        let tic = self
            .get_mut_ti_cache(path, num)
            .ok_or_else(|| "Unexpected None CacheMIOfFileTrack")?;

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

    pub(super) fn build_ti_lang(&mut self, path: &Path, num: u64) -> Result<LangCode, MuxError> {
        let tic = self
            .get_mut_ti_cache(path, num)
            .ok_or_else(|| "Unexpected None CacheMIOfFileTrack")?;
        let lang = tic
            .mkvinfo_cutted
            .as_ref()
            .and_then(|mkvi| mkvi.get::<MKVILang>().copied())
            .unwrap_or_else(|| {
                use std::str::FromStr;
                from_name_tail_relative_or_fallback!(
                    self, path, num, LangCode, from_str, try_from, Und
                )
            });
        Ok(lang)
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
