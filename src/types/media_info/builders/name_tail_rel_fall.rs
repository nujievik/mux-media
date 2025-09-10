use super::REGEX_WORD;
use crate::{
    LangCode, MediaInfo, MuxError, RawTrackCache, TrackID, TrackType, Value, immut,
    markers::{
        MCTrackLangs, MCTrackNames, MICmnStem, MIPathTail, MIRelativeUpmost, MITICache, MITIName,
        MITIWordsName, MITargets, MITracksInfo, MIWordsPathTail, MIWordsRelativeUpmost,
    },
    types::helpers,
};
use std::path::Path;

macro_rules! try_from_tail_rel {
    ($mi:ident, $media:ident, $ty:ty, $try_from_words:ident) => {
        $mi.try_get::<MIWordsPathTail>($media)
            .and_then(|words| <$ty>::$try_from_words(words.as_slice()))
            .or_else(|_| {
                $mi.try_get::<MIWordsRelativeUpmost>($media)
                    .and_then(|words| <$ty>::$try_from_words(words.as_slice()))
            })
    };
}

impl MediaInfo<'_> {
    pub(crate) fn build_path_tail(&mut self, media: &Path) -> Result<String, MuxError> {
        let cmn_stem = self.try_get_cmn::<MICmnStem>()?;
        let stem = media
            .file_stem()
            .ok_or_else(|| format!("Path '{}' has not file_stem()", media.display()))?;
        helpers::os_str_tail(cmn_stem, stem).map(|os| os.to_string_lossy().into_owned())
    }

    pub(crate) fn build_words_path_tail(&mut self, media: &Path) -> Result<Vec<String>, MuxError> {
        let tail = immut!(@try, self, MIPathTail, media)?;
        Ok(str_to_words(tail))
    }

    pub(crate) fn build_relative_upmost(&mut self, media: &Path) -> Result<String, MuxError> {
        media
            .parent()
            .ok_or_else(|| format!("Path '{}' has not parent()", media.display()).into())
            .and_then(|parent| {
                helpers::os_str_tail(self.cfg.input.dir.as_os_str(), parent.as_os_str())
                    .map(|os| os.to_string_lossy().into_owned())
            })
    }

    pub(crate) fn build_words_relative_upmost(
        &mut self,
        media: &Path,
    ) -> Result<Vec<String>, MuxError> {
        let tail = immut!(@try, self, MIRelativeUpmost, media)?;
        Ok(str_to_words(tail))
    }

    pub(crate) fn build_ti_name(
        &mut self,
        media: &Path,
        track: u64,
    ) -> Result<Value<String>, MuxError> {
        // User-defined
        if let Some(name) = match immut!(@try, self, MITargets, media) {
            Ok(targets) => self.cfg.target(MCTrackNames, targets),
            Err(_) => &self.cfg.track_names,
        }
        .get(&TrackID::Num(track))
        {
            return Ok(Value::User(name.to_owned()));
        }

        // From matroska tags
        if let Some(name) = self.try_get::<MITracksInfo>(media).ok().and_then(|ti| {
            match ti.get(&track).map(|cache| &cache.raw) {
                Some(RawTrackCache::Matroska(mat)) => mat.name.clone(),
                _ => None,
            }
        }) {
            return Ok(Value::Auto(name));
        }

        // From path tail
        if let Some(name) = self.try_get::<MIPathTail>(media).ok().and_then(|s| {
            let s = s.trim_matches(&['.', ' ']);
            (s.len() > 2).then(|| s.to_owned())
        }) {
            return Ok(Value::Auto(name));
        }

        // From parent
        if let Some(name) = media
            .parent()
            .filter(|p| p.as_os_str().len() != self.cfg.input.dir.as_os_str().len())
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned())
        {
            return Ok(Value::Auto(name));
        }

        Ok(Value::Auto(String::new()))
    }

    pub(crate) fn build_ti_words_name(
        &mut self,
        media: &Path,
        track: u64,
    ) -> Result<Vec<String>, MuxError> {
        let name = immut!(@try, self, MITIName, media, track)?;
        Ok(str_to_words(name))
    }

    pub(crate) fn build_ti_lang(
        &mut self,
        media: &Path,
        track: u64,
    ) -> Result<Value<LangCode>, MuxError> {
        // User-defined
        if let Some(lang) = match immut!(@try, self, MITargets, media) {
            Ok(targets) => self.cfg.target(MCTrackLangs, targets),
            Err(_) => &self.cfg.track_langs,
        }
        .get(&TrackID::Num(track))
        {
            return Ok(Value::User(lang));
        }

        // Matroska tags
        if let Some(lang) = self.try_get::<MITracksInfo>(media).ok().and_then(|ti| {
            match ti.get(&track).map(|cache| &cache.raw) {
                Some(RawTrackCache::Matroska(mat)) => mat
                    .language
                    .as_ref()
                    .and_then(|lang| {
                        match lang {
                            matroska::Language::ISO639(s) => s,
                            matroska::Language::IETF(s) => s,
                        }
                        .parse::<LangCode>()
                        .ok()
                    })
                    .filter(|lang| lang != &LangCode::Und),

                _ => None,
            }
        }) {
            return Ok(Value::Auto(lang));
        }

        // From name
        if let Ok(lang) = self
            .try_get_ti::<MITIWordsName>(media, track)
            .and_then(|words| LangCode::try_priority(words.as_slice()))
        {
            return Ok(Value::Auto(lang));
        }

        // From tail or relative
        if let Ok(lang) = try_from_tail_rel!(self, media, LangCode, try_priority) {
            return Ok(Value::Auto(lang));
        }

        Ok(Value::Auto(LangCode::Und))
    }

    pub(crate) fn build_ti_it_signs(&mut self, media: &Path, track: u64) -> Result<bool, MuxError> {
        match self
            .get_ti::<MITICache>(media, track)
            .map(|cache| cache.track_type)
        {
            Some(TrackType::Sub) => {}
            _ => return Ok(false),
        }

        if let Some(words) = self.get::<MIWordsPathTail>(media) {
            if ItSigns::it_signs_many(words) {
                return Ok(true);
            }
        }

        if let Some(words) = self.get::<MIWordsRelativeUpmost>(media) {
            if ItSigns::it_signs_many(words) {
                return Ok(true);
            }
        }

        if let Some(words) = self.get_ti::<MITIWordsName>(media, track) {
            if ItSigns::it_signs_many(words) {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

#[inline]
fn str_to_words(s: &str) -> Vec<String> {
    REGEX_WORD
        .find_iter(s)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

struct ItSigns;

impl ItSigns {
    fn it_signs_many(s: &Vec<String>) -> bool {
        s.iter().find(|s| Self::it_signs(s)).is_some()
    }

    fn it_signs(s: &str) -> bool {
        match s.trim().to_lowercase().as_str() {
            "signs" => true,
            "надписи" => true,
            _ => false,
        }
    }
}
