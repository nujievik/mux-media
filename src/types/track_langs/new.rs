use super::TrackLangs;
use crate::{LangCode, MuxError, TrackID};
use std::{collections::HashMap, str::FromStr};

impl FromStr for TrackLangs {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if !s.contains(':') {
            let unmapped = s.parse::<LangCode>()?;

            return Ok(Self {
                unmapped: Some(unmapped),
                map_hashed: None,
                map_unhashed: None,
            });
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

        Ok(Self {
            unmapped: None,
            map_hashed,
            map_unhashed,
        })
    }
}
