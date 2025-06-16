use super::{TrackLangs, id::TrackID};
use crate::{IsDefault, LangCode, MuxError};
use std::collections::HashMap;
use std::str::FromStr;

impl IsDefault for TrackLangs {
    fn is_default(&self) -> bool {
        self.unmapped.is_none() && self.map_hashed.is_none() && self.map_unhashed.is_none()
    }
}

impl TrackLangs {
    pub fn get(&self, tid: &TrackID) -> Option<&LangCode> {
        if let Some(lang) = &self.unmapped {
            return Some(lang);
        }

        if let Some(langs) = &self.map_hashed {
            if let Some(lang) = langs.get(&tid) {
                return Some(lang);
            }
        }

        if let Some(langs) = &self.map_unhashed {
            for (id, lang) in langs.iter() {
                if id.contains(&tid) {
                    return Some(lang);
                }
            }
        }

        None
    }
}

impl FromStr for TrackLangs {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if !s.contains(':') {
            return Ok(Self::new().unmapped(Some(LangCode::from_str(s)?)));
        }

        let mut map_hashed: Option<HashMap<TrackID, LangCode>> = None;
        let mut map_unhashed: Option<Vec<(TrackID, LangCode)>> = None;

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (id, lng) = part
                .split_once(':')
                .ok_or(MuxError::from("Invalid format: Must be [n:]L[,m:L]..."))?;
            let id = TrackID::from_str(id)?;
            let lng = LangCode::from_str(lng)?;

            if id.is_range() {
                map_unhashed.get_or_insert_with(Vec::new).push((id, lng));
            } else {
                map_hashed.get_or_insert_with(HashMap::new).insert(id, lng);
            }
        }

        if map_hashed.is_none() && map_unhashed.is_none() {
            return Err(MuxError::from("No languages found"));
        }

        Ok(Self::new()
            .map_hashed(map_hashed)
            .map_unhashed(map_unhashed))
    }
}

impl TrackLangs {
    pub fn new() -> Self {
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
