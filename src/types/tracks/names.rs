use super::id::TrackID;
use crate::{IsDefault, MuxError, from_arg_matches, mkvmerge_arg, to_json_args, to_mkvmerge_args};
use std::collections::HashMap;

/// Settings for track names.
#[derive(Clone, Default)]
pub struct TrackNames {
    unmapped: Option<String>,
    map_hashed: Option<HashMap<TrackID, String>>,
    map_unhashed: Option<Vec<(TrackID, String)>>,
}

mkvmerge_arg!(TrackNames, "--track-name");
to_mkvmerge_args!(@names_or_langs, TrackNames, Names, add_names, MITIName);

to_json_args!(@names_or_langs, TrackNames, Names);

impl clap::FromArgMatches for TrackNames {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(from_arg_matches!(matches, Self, Names, Self::default))
    }
}

impl IsDefault for TrackNames {
    fn is_default(&self) -> bool {
        self.unmapped.is_none() && self.map_hashed.is_none() && self.map_unhashed.is_none()
    }
}

impl TrackNames {
    /// Returns `Some` if `self` contains a name for the given `TrackID`.
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

impl TrackNames {
    fn new() -> Self {
        Self::default()
    }

    fn unmapped(mut self, name: Option<String>) -> Self {
        self.unmapped = name;
        self
    }

    fn map_hashed(mut self, map: Option<HashMap<TrackID, String>>) -> Self {
        self.map_hashed = map;
        self
    }

    fn map_unhashed(mut self, map: Option<Vec<(TrackID, String)>>) -> Self {
        self.map_unhashed = map;
        self
    }
}

impl std::str::FromStr for TrackNames {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if !s.contains(':') {
            return Ok(Self::new().unmapped(Some(s.to_string())));
        }

        let mut map_hashed: Option<HashMap<TrackID, String>> = None;
        let mut map_unhashed: Option<Vec<(TrackID, String)>> = None;

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (id, name) = part
                .split_once(':')
                .ok_or("Invalid format: Must be [n:]N[,m:N]...")?;
            let id = id.parse::<TrackID>()?;
            let name = name.to_string();

            if id.is_range() {
                map_unhashed.get_or_insert_with(Vec::new).push((id, name));
            } else {
                map_hashed.get_or_insert_with(HashMap::new).insert(id, name);
            }
        }

        if map_hashed.is_none() && map_unhashed.is_none() {
            return Err("No names found".into());
        }

        Ok(Self::new()
            .map_hashed(map_hashed)
            .map_unhashed(map_unhashed))
    }
}
