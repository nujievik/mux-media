mod from;
mod to_args;

use crate::{IsDefault, LangCode, TrackID};
use std::collections::HashMap;

/// Settings for track languages.
#[derive(Clone, Default, IsDefault)]
pub struct TrackLangs {
    unmapped: Option<LangCode>,
    map_hashed: Option<HashMap<TrackID, LangCode>>,
    map_unhashed: Option<Vec<(TrackID, LangCode)>>,
}

impl TrackLangs {
    /// Gets a user-defined value for given [`TrackID`].
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

    #[inline(always)]
    fn get_from_hashed(&self, tid: &TrackID) -> Option<LangCode> {
        self.map_hashed
            .as_ref()
            .and_then(|map| map.get(tid))
            .copied()
    }

    #[inline(always)]
    fn get_from_unhashed(&self, tid: &TrackID) -> Option<LangCode> {
        self.map_unhashed
            .as_ref()
            .and_then(|map| map.iter().find(|(id, _)| id.contains(tid)))
            .map(|(_, lang)| *lang)
    }
}
