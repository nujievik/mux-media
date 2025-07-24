use super::id::TrackID;
use crate::{
    IsDefault, LangCode, MuxError, from_arg_matches, mkvmerge_arg, to_json_args, to_mkvmerge_args,
};
use std::collections::HashMap;

/// Settings for track languages.
#[derive(Clone, Default)]
pub struct TrackLangs {
    unmapped: Option<LangCode>,
    map_hashed: Option<HashMap<TrackID, LangCode>>,
    map_unhashed: Option<Vec<(TrackID, LangCode)>>,
}

mkvmerge_arg!(TrackLangs, "--language");
to_mkvmerge_args!(@names_or_langs, TrackLangs, Langs, auto_langs, MITILang);

to_json_args!(@names_or_langs, TrackLangs, Langs);

impl clap::FromArgMatches for TrackLangs {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(from_arg_matches!(matches, Self, Langs, Self::default))
    }
}

impl IsDefault for TrackLangs {
    fn is_default(&self) -> bool {
        self.unmapped.is_none() && self.map_hashed.is_none() && self.map_unhashed.is_none()
    }
}

impl TrackLangs {
    /// Returns user-defined value if exists; otherwise, returns `None`.
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

impl std::str::FromStr for TrackLangs {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if !s.contains(':') {
            return Ok(Self::new().unmapped(Some(s.parse::<LangCode>()?)));
        }

        let mut map_hashed: Option<HashMap<TrackID, LangCode>> = None;
        let mut map_unhashed: Option<Vec<(TrackID, LangCode)>> = None;

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (id, lng) = part
                .split_once(':')
                .ok_or("Invalid format: Must be [n:]L[,m:L]...")?;
            let id = TrackID::from_str(id)?;
            let lng = LangCode::from_str(lng)?;

            if id.is_range() {
                map_unhashed.get_or_insert_with(Vec::new).push((id, lng));
            } else {
                map_hashed.get_or_insert_with(HashMap::new).insert(id, lng);
            }
        }

        if map_hashed.is_none() && map_unhashed.is_none() {
            return Err("No languages found".into());
        }

        Ok(Self::new()
            .map_hashed(map_hashed)
            .map_unhashed(map_unhashed))
    }
}

impl TrackLangs {
    fn new() -> Self {
        Self::default()
    }

    fn unmapped(mut self, name: Option<LangCode>) -> Self {
        self.unmapped = name;
        self
    }

    fn map_hashed(mut self, map: Option<HashMap<TrackID, LangCode>>) -> Self {
        self.map_hashed = map;
        self
    }

    fn map_unhashed(mut self, map: Option<Vec<(TrackID, LangCode)>>) -> Self {
        self.map_unhashed = map;
        self
    }
}
