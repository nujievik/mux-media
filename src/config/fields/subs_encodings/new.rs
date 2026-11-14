use super::*;
use crate::{MuxError, Result};
use std::str::FromStr;

impl FromStr for ConfigSubsEncodings {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<ConfigSubsEncodings> {
        if let Some(v) = Encoding::for_label_no_replacement(s.as_bytes()) {
            return Ok(Self {
                single_val: Some(v),
                ..Default::default()
            });
        }

        let mut idxs: FxIndexMap<usize, &'static Encoding> = Default::default();
        let mut ranges: Vec<(RangeUsizeInclusive, &'static Encoding)> = Vec::new();
        let mut langs: FxIndexMap<Lang, &'static Encoding> = Default::default();

        for part in s.split(',') {
            let (id, v) = part
                .split_once(':')
                .ok_or_else(|| err!("format must be id:value"))?;

            let v = Encoding::for_label_no_replacement(v.as_bytes())
                .ok_or_else(|| err!("unrecognized encoding"))?;

            if let Ok(i) = id.parse::<usize>() {
                idxs.insert(i, v);
            } else if let Ok(rng) = id.parse::<RangeUsizeInclusive>() {
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
