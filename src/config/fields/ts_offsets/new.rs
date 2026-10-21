use super::*;
use crate::{MuxError, Result};
use std::str::FromStr;

impl FromStr for ConfigTsOffsets {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(v) = s.parse::<i32>() {
            return Ok(Self {
                single_val: Some(v),
                ..Default::default()
            });
        }

        let mut idxs: FxIndexMap<usize, i32> = Default::default();
        let mut ranges: Vec<(RangeUsize, i32)> = Vec::new();
        let mut langs: FxIndexMap<Lang, i32> = Default::default();

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (id, v) = part
                .split_once(':')
                .ok_or_else(|| err!("format must be id:value"))?;

            let v = v.parse::<i32>()?;

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
