use super::TrackNames;
use crate::{MuxError, TrackID, from_arg_matches};
use clap::{ArgMatches, Error, FromArgMatches};
use std::{collections::HashMap, str::FromStr};

impl FromArgMatches for TrackNames {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        Ok(from_arg_matches!(matches, Self, Names, Self::default))
    }
}

impl FromStr for TrackNames {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if !s.contains(':') {
            return Ok(Self {
                unmapped: Some(s.to_owned()),
                map_hashed: None,
                map_unhashed: None,
            });
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

        Ok(Self {
            unmapped: None,
            map_hashed,
            map_unhashed,
        })
    }
}
