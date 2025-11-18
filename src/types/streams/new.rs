use super::Streams;
use crate::{LangCode, MuxError, RangeUsize, Result};
use std::{collections::HashSet, str::FromStr};

impl FromStr for Streams {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();

        let (inverse, s) = if s.starts_with('!') {
            (true, &s[1..])
        } else {
            (false, s)
        };

        let mut idxs: Option<HashSet<usize>> = None;
        let mut langs: Option<HashSet<LangCode>> = None;
        let mut ranges: Option<Vec<RangeUsize>> = None;

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            if let Ok(i) = part.parse::<usize>() {
                idxs.get_or_insert_default().insert(i);
            } else if let Ok(lang) = part.parse::<LangCode>() {
                langs.get_or_insert_default().insert(lang);
            } else if let Ok(rng) = part.parse::<RangeUsize>() {
                ranges.get_or_insert_default().push(rng);
            } else {
                return Err(err!(
                    "Invalid stream ID '{}' (must be num, range (n-m) of num or lang code)",
                    part
                ));
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
