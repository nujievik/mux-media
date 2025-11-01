mod new;

use crate::{IsDefault, TrackID};
use std::collections::HashMap;

/// Settings for track names.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct TrackNames {
    pub unmapped: Option<String>,
    pub map_hashed: Option<HashMap<TrackID, String>>,
    pub map_unhashed: Option<Vec<(TrackID, String)>>,
}

impl TrackNames {
    /// Gets a user-defined value for given [`TrackID`].
    pub fn get(&self, tid: &TrackID) -> Option<&String> {
        if let Some(name) = &self.unmapped {
            return Some(name);
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
    fn get_from_hashed(&self, tid: &TrackID) -> Option<&String> {
        self.map_hashed.as_ref().and_then(|map| map.get(tid))
    }

    #[inline(always)]
    fn get_from_unhashed(&self, tid: &TrackID) -> Option<&String> {
        self.map_unhashed
            .as_ref()
            .and_then(|map| map.iter().find(|(id, _)| id.contains(tid)))
            .map(|(_, name)| name)
    }
}

to_ffmpeg_args!(@names_or_langs, TrackNames, Title, names, MITIName);
to_json_args!(@names_or_langs, TrackNames, Names);
