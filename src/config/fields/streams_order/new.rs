use super::*;
use crate::{MuxError, Result};
use std::str::FromStr;

impl FromStr for ConfigStreamsOrder {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(v) = s.parse::<u8>() {
            return Ok(Self {
                single_val: Some(v),
                ..Default::default()
            });
        }

        let mut idxs: FxIndexMap<usize, u8> = Default::default();
        let mut ranges: Vec<(RangeUsize, u8)> = Vec::new();
        let mut langs: FxIndexMap<Lang, u8> = Default::default();

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (id, v) = part
                .split_once(':')
                .ok_or_else(|| err!("format must be id:value"))?;

            let v = v.parse::<u8>()?;

            if let Ok(i) = id.parse::<usize>() {
                idxs.insert(i, v);
            } else if let Ok(rng) = id.parse::<RangeUsize>() {
                ranges.push((rng, v));
            } else {
                langs.insert(Lang::new(id), v);
            }
        }

        Ok(Self {
            idxs: some_if_unempty!(idxs),
            langs: some_if_unempty!(langs),
            ranges: some_if_unempty!(ranges),
            ..Default::default()
        })
    }
}
