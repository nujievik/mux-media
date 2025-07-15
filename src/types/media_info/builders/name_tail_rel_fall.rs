use crate::{
    LangCode, MICmnRegexWord, MIGroupStem, MIPathTail, MIRelativeUpmost, MITIName, MITracksInfo,
    MediaInfo, MuxError, TargetGroup, TrackType,
    types::helpers,
    types::tools::mkvinfo::{MKVILang, MKVIName},
};
use regex::Regex;
use std::path::Path;

macro_rules! from_name_tail_relative_or_fallback {
    ($mi:ident, $path:ident, $num:expr, $typ:ident, $try_from_words:ident, $fall:ident) => {
        match $mi.try_get_cmn::<MICmnRegexWord>() {
            Err(_) => $typ::$fall,
            Ok(re) => {
                let re = re.clone();

                $mi.try_get_ti::<MITIName>($path, $num)
                    .and_then(|name| $typ::$try_from_words(str_to_words(&re, name).as_slice()))
                    .or_else(|_| {
                        $mi.try_get::<MIPathTail>($path)
                            .and_then(|s| $typ::$try_from_words(str_to_words(&re, &s).as_slice()))
                    })
                    .or_else(|_| {
                        $mi.try_get::<MIRelativeUpmost>($path)
                            .and_then(|s| $typ::$try_from_words(str_to_words(&re, &s).as_slice()))
                    })
                    .unwrap_or($typ::$fall)
            }
        }
    };
}

impl MediaInfo<'_> {
    pub(crate) fn build_target_group(&mut self, path: &Path) -> Result<TargetGroup, MuxError> {
        let num_types: Vec<(u64, TrackType)> = self
            .try_get::<MITracksInfo>(path)?
            .iter()
            .map(|(num, cache)| (*num, cache.track_type))
            .collect();

        for tt in [TrackType::Video, TrackType::Audio, TrackType::Sub] {
            if let Some(&(num, _)) = num_types.iter().find(|(_, t)| *t == tt) {
                if tt != TrackType::Sub {
                    return Ok(tt.into());
                }

                let val = from_name_tail_relative_or_fallback!(
                    self,
                    path,
                    num,
                    TargetGroup,
                    try_signs_from_slice_string,
                    Subs
                );

                return Ok(val);
            }
        }

        Err("Not found any Media Track".into())
    }

    pub(crate) fn build_path_tail(&mut self, path: &Path) -> Result<String, MuxError> {
        let cmn_stem = self.try_get_cmn::<MIGroupStem>()?;
        let stem = path
            .file_stem()
            .ok_or_else(|| format!("Path '{}' has not file_stem()", path.display()))?;
        helpers::os_str_tail(cmn_stem, stem).map(|os| os.to_string_lossy().into_owned())
    }

    pub(crate) fn build_relative_upmost(&mut self, path: &Path) -> Result<String, MuxError> {
        path.parent()
            .ok_or_else(|| format!("Path '{}' has not parent()", path.display()).into())
            .and_then(|parent| {
                helpers::os_str_tail(self.upmost.as_os_str(), parent.as_os_str())
                    .map(|os| os.to_string_lossy().into_owned())
            })
    }

    pub(crate) fn build_ti_name(&mut self, path: &Path, num: u64) -> Result<String, MuxError> {
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

    pub(crate) fn build_ti_lang(&mut self, path: &Path, num: u64) -> Result<LangCode, MuxError> {
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
}

#[inline(always)]
fn str_to_words(re: &Regex, s: &str) -> Vec<String> {
    re.find_iter(s)
        .map(|mat| mat.as_str().to_string())
        .collect()
}
