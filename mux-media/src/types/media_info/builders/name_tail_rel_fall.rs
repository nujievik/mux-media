use crate::{
    LangCode, MediaInfo, MuxError, TargetGroup, TrackID, TrackType,
    markers::{
        MCTrackLangs, MCTrackNames, MICmnRegexWord, MIGroupStem, MIPathTail, MIRelativeUpmost,
        MITIName, MITIWordsName, MITargets, MITracksInfo, MIWordsPathTail, MIWordsRelativeUpmost,
    },
    types::helpers,
    unmut,
};
use regex::Regex;
use std::path::Path;

macro_rules! try_from_tail_rel {
    ($mi:ident, $path:ident, $typ:ident, $try_from_words:ident) => {
        $mi.try_get::<MIWordsPathTail>($path)
            .and_then(|words| $typ::$try_from_words(words.as_slice()))
            .or_else(|_| {
                $mi.try_get::<MIWordsRelativeUpmost>($path)
                    .and_then(|words| $typ::$try_from_words(words.as_slice()))
            })
    };
}

impl MediaInfo<'_> {
    pub(crate) fn build_path_tail(&mut self, media: &Path) -> Result<String, MuxError> {
        let cmn_stem = self.try_get_cmn::<MIGroupStem>()?;
        let stem = media
            .file_stem()
            .ok_or_else(|| format!("Path '{}' has not file_stem()", media.display()))?;
        helpers::os_str_tail(cmn_stem, stem).map(|os| os.to_string_lossy().into_owned())
    }

    pub(crate) fn build_words_path_tail(&mut self, media: &Path) -> Result<Vec<String>, MuxError> {
        let _ = self.try_get::<MIPathTail>(media)?;
        let re = unmut!(@try, self, MICmnRegexWord)?;
        let tail = self.unmut::<MIPathTail>(media).ok_or("Unexpected None")?;

        Ok(str_to_words(re, tail))
    }

    pub(crate) fn build_relative_upmost(&mut self, media: &Path) -> Result<String, MuxError> {
        media
            .parent()
            .ok_or_else(|| format!("Path '{}' has not parent()", media.display()).into())
            .and_then(|parent| {
                helpers::os_str_tail(self.upmost.as_os_str(), parent.as_os_str())
                    .map(|os| os.to_string_lossy().into_owned())
            })
    }

    pub(crate) fn build_words_relative_upmost(
        &mut self,
        media: &Path,
    ) -> Result<Vec<String>, MuxError> {
        let _ = self.try_get::<MIRelativeUpmost>(media)?;
        let re = unmut!(@try, self, MICmnRegexWord)?;
        let tail = self
            .unmut::<MIRelativeUpmost>(media)
            .ok_or("Unexpected None")?;

        Ok(str_to_words(re, tail))
    }

    pub(crate) fn build_target_group(&mut self, media: &Path) -> Result<TargetGroup, MuxError> {
        let num_types: Vec<(u64, TrackType)> = self
            .try_get::<MITracksInfo>(media)?
            .iter()
            .map(|(num, cache)| (*num, cache.track_type))
            .collect();

        for tt in [TrackType::Video, TrackType::Audio, TrackType::Sub] {
            if let Some(&(_, _)) = num_types.iter().find(|(_, t)| *t == tt) {
                if tt != TrackType::Sub {
                    let group = TargetGroup::try_from(tt)?;
                    return Ok(group);
                }

                if let Ok(signs) = try_from_tail_rel!(self, media, TargetGroup, try_signs_many) {
                    return Ok(signs);
                }

                return Ok(TargetGroup::Subs);
            }
        }

        Err("Not found any Media Track".into())
    }

    pub(crate) fn build_ti_name(&mut self, media: &Path, track: u64) -> Result<String, MuxError> {
        // User-defined
        if let Some(name) = match unmut!(@try, self, MITargets, media) {
            Ok(targets) => self.mux_config.trg_field::<MCTrackNames>(&targets),
            Err(_) => self.mux_config.field::<MCTrackNames>(),
        }
        .get(&TrackID::Num(track))
        {
            return Ok(name.to_owned());
        }

        // From matroska tags
        if let Some(name) = self.try_get::<MITracksInfo>(media).ok().and_then(|ti| {
            ti.get(&track).and_then(|cache| {
                cache
                    .matroska
                    .as_ref()
                    .and_then(|matroska| matroska.name.as_ref().map(|n| n.to_owned()))
            })
        }) {
            return Ok(name);
        }

        // From path tail
        if let Some(name) = self.try_get::<MIPathTail>(media).ok().and_then(|s| {
            let s = s.trim_matches(&['.', ' ']);
            (s.len() > 2).then(|| s.to_owned())
        }) {
            return Ok(name);
        }

        // From parent
        if let Some(name) = media
            .parent()
            .filter(|p| p.as_os_str().len() != self.upmost.as_os_str().len())
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned())
        {
            return Ok(name);
        }

        Ok(String::new())
    }

    pub(crate) fn build_ti_words_name(
        &mut self,
        media: &Path,
        track: u64,
    ) -> Result<Vec<String>, MuxError> {
        let _ = self.try_get_ti::<MITIName>(media, track)?;
        let re = unmut!(@try, self, MICmnRegexWord)?;
        let name = self
            .unmut_ti::<MITIName>(media, track)
            .ok_or("Unexpected None")?;

        Ok(str_to_words(re, name))
    }

    pub(crate) fn build_ti_lang(&mut self, media: &Path, track: u64) -> Result<LangCode, MuxError> {
        // User-defined
        if let Some(lang) = match unmut!(@try, self, MITargets, media) {
            Ok(targets) => self.mux_config.trg_field::<MCTrackLangs>(&targets),
            Err(_) => self.mux_config.field::<MCTrackLangs>(),
        }
        .get(&TrackID::Num(track))
        {
            return Ok(lang);
        }

        // Matroska tags
        if let Some(lang) = self.try_get::<MITracksInfo>(media).ok().and_then(|ti| {
            ti.get(&track).and_then(|cache| {
                cache.matroska.as_ref().and_then(|matroska| {
                    matroska.language.as_ref().and_then(|lang| {
                        let s = match lang {
                            matroska::Language::ISO639(s) => s,
                            matroska::Language::IETF(s) => s,
                        };
                        s.parse::<LangCode>().ok()
                    })
                })
            })
        }) {
            return Ok(lang);
        }

        // From name
        if let Ok(lang) = self
            .try_get_ti::<MITIWordsName>(media, track)
            .and_then(|words| LangCode::try_from(words.as_slice()))
        {
            return Ok(lang);
        }

        // From tail or relative
        if let Ok(lang) = try_from_tail_rel!(self, media, LangCode, try_from) {
            return Ok(lang);
        }

        Ok(LangCode::Und)
    }
}

#[inline]
fn str_to_words(re: &Regex, s: &str) -> Vec<String> {
    re.find_iter(s)
        .map(|mat| mat.as_str().to_string())
        .collect()
}
