use super::*;
use crate::{MuxError, Result};
use std::str::FromStr;

impl FromStr for ConfigDispositions {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(b) = get_bool(s) {
            return Ok(Self {
                single_val: Some(b),
                ..Default::default()
            });
        }

        let mut idxs: FxIndexMap<usize, Bool> = Default::default();
        let mut ranges: Vec<(RangeUsize, Bool)> = Vec::new();
        let mut langs: FxIndexMap<Lang, Bool> = Default::default();

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (id, b) = part.split_once(':').unwrap_or((part, "true"));

            let b = get_bool(b).ok_or_else(|| err!("invalid bool key ({})", b))?;

            if let Ok(i) = id.parse::<usize>() {
                idxs.insert(i, b);
            } else if let Ok(rng) = id.parse::<RangeUsize>() {
                ranges.push((rng, b));
            } else {
                langs.insert(Lang::new(id), b);
            }
        }

        return Ok(Self {
            idxs: some_if_unempty!(idxs),
            langs: some_if_unempty!(langs),
            ranges: some_if_unempty!(ranges),
            ..Default::default()
        });
    }
}

fn get_bool(s: &str) -> Option<Bool> {
    match s {
        "1" | "true" | "on" => Some(Bool(true)),
        "0" | "false" | "off" => Some(Bool(false)),
        _ => None,
    }
}
