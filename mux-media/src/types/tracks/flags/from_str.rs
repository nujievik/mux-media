use crate::{MuxError, TFlags, TrackID};
use std::collections::HashMap;
use std::str::FromStr;

impl FromStr for TFlags {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(b) = parse_bool(s) {
            return Ok(Self {
                unmapped: Some(b),
                ..Default::default()
            });
        }

        let mut map_hashed: Option<HashMap<TrackID, bool>> = None;
        let mut map_unhashed: Option<Vec<(TrackID, bool)>> = None;

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (id, b) = part.split_once(':').unwrap_or((part, "true"));
            let id = TrackID::from_str(id)?;
            let bool = parse_bool(b)?;

            if id.is_range() {
                map_hashed.get_or_insert_with(HashMap::new).insert(id, bool);
            } else {
                map_unhashed.get_or_insert_with(Vec::new).push((id, bool));
            }
        }

        if map_hashed.is_none() && map_unhashed.is_none() {
            return Err("No track IDs found".into());
        }

        Ok(Self {
            map_hashed,
            map_unhashed,
            ..Default::default()
        })
    }
}

#[inline]
fn parse_bool(s: &str) -> Result<bool, MuxError> {
    match s.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "on" => Ok(true),
        "0" | "false" | "off" => Ok(false),
        _ => Err(format!("Invalid bool key '{}'", s).into()),
    }
}
