mod new;

use crate::{IsDefault, LangCode, TrackID, to_ffmpeg_args, to_json_args, to_mkvmerge_args};
use std::collections::HashMap;

/// Settings for track languages.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct TrackLangs {
    pub unmapped: Option<LangCode>,
    pub map_hashed: Option<HashMap<TrackID, LangCode>>,
    pub map_unhashed: Option<Vec<(TrackID, LangCode)>>,
}

impl TrackLangs {
    /// Gets a user-defined value for given [`TrackID`].
    #[inline]
    pub fn get(&self, tid: &TrackID) -> Option<LangCode> {
        if let Some(lang) = &self.unmapped {
            return Some(*lang);
        }

        match tid {
            TrackID::Num(_) => self
                .get_from_hashed(tid)
                .or_else(|| self.get_from_unhashed(tid)),
            TrackID::Lang(_) => self.get_from_hashed(tid),
            TrackID::Range(_) => self.get_from_unhashed(tid),
        }
    }

    fn get_from_hashed(&self, tid: &TrackID) -> Option<LangCode> {
        self.map_hashed
            .as_ref()
            .and_then(|map| map.get(tid))
            .copied()
    }

    fn get_from_unhashed(&self, tid: &TrackID) -> Option<LangCode> {
        self.map_unhashed
            .as_ref()
            .and_then(|map| map.iter().find(|(id, _)| id.contains(tid)))
            .map(|(_, lang)| *lang)
    }
}

to_ffmpeg_args!(@names_or_langs, TrackLangs, Language, langs, MITILang);
to_json_args!(@names_or_langs, TrackLangs, Langs);
to_mkvmerge_args!(@names_or_langs, TrackLangs, Language, langs, MITILang);
