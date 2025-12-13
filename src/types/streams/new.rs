use super::*;
use crate::{MuxError, Result, helpers};
use std::str::FromStr;

impl FromStr for Streams {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let (inverse, s) = helpers::parse_inverse_str(s);

        let mut idxs: Option<HashSet<usize>> = None;
        let mut ranges: Option<Vec<RangeUsize>> = None;
        let mut langs: Option<HashSet<Lang>> = None;

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            if let Ok(i) = part.parse::<usize>() {
                idxs.get_or_insert_default().insert(i);
            } else if let Ok(rng) = part.parse::<RangeUsize>() {
                ranges.get_or_insert_default().push(rng);
            } else {
                langs.get_or_insert_default().insert(Lang::new(part));
            }
        }

        if idxs.is_none() && langs.is_none() && ranges.is_none() {
            return Err(err!("No stream IDs found"));
        }

        Ok(Self {
            no_flag: false,
            inverse,
            idxs,
            langs,
            ranges,
        })
    }
}
