use super::*;
use crate::{MuxError, Result, helpers};
use std::str::FromStr;

impl FromStr for ConfigStreams {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let (inverse, s) = helpers::parse_inverse_str(s);

        let mut idxs: FxIndexSet<usize> = Default::default();
        let mut ranges: Vec<RangeUsize> = Vec::new();
        let mut langs: FxIndexSet<Lang> = Default::default();

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            if let Ok(i) = part.parse::<usize>() {
                idxs.insert(i);
            } else if let Ok(rng) = part.parse::<RangeUsize>() {
                ranges.push(rng);
            } else {
                langs.insert(Lang::new(part));
            }
        }

        Ok(Self {
            no_flag: false,
            inverse,
            idxs: some_if_unempty!(idxs),
            langs: some_if_unempty!(langs),
            ranges: some_if_unempty!(ranges),
        })
    }
}
