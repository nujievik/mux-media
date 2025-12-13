use super::*;
use crate::{MuxError, Result};
use std::str::FromStr;

impl FromStr for Dispositions {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(b) = parse_bool(s) {
            return Ok(Self {
                single_val: Some(b),
                ..Default::default()
            });
        }

        let mut idxs: Option<HashMap<usize, bool>> = None;
        let mut ranges: Option<Vec<(RangeUsize, bool)>> = None;
        let mut langs: Option<HashMap<Lang, bool>> = None;

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (id, b) = part.split_once(':').unwrap_or((part, "true"));
            let b = parse_bool(b)?;

            if let Ok(i) = id.parse::<usize>() {
                idxs.get_or_insert_default().insert(i, b);
            } else if let Ok(rng) = id.parse::<RangeUsize>() {
                ranges.get_or_insert_default().push((rng, b));
            } else {
                langs.get_or_insert_default().insert(Lang::new(id), b);
            }
        }

        if idxs.is_none() && langs.is_none() && ranges.is_none() {
            return Err(err!("No stream IDs found"));
        }

        return Ok(Self {
            idxs,
            langs,
            ranges,
            ..Default::default()
        });

        fn parse_bool(s: &str) -> Result<bool> {
            match s.trim().to_ascii_lowercase().as_str() {
                "1" | "true" | "on" => Ok(true),
                "0" | "false" | "off" => Ok(false),
                _ => Err(err!("Invalid bool key '{}'", s)),
            }
        }
    }
}
