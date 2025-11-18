use super::{LangMetadata, Metadata, NameMetadata};
use crate::{LangCode, MuxError, RangeUsize, Result};
use std::{collections::HashMap, str::FromStr};

macro_rules! from_str_impl {
    ($ty:ty, $v:ty, $h:literal) => {
        impl FromStr for $ty {
            type Err = MuxError;

            fn from_str(s: &str) -> Result<Self> {
                let s = s.trim();

                if !s.contains(':') {
                    let single_val = s.parse::<$v>().map_err(|e| MuxError::from_any(e))?;

                    return Ok(Self(Metadata {
                        single_val: Some(single_val),
                        ..Default::default()
                    }));
                }

                let mut idxs: Option<HashMap<usize, $v>> = None;
                let mut langs: Option<HashMap<LangCode, $v>> = None;
                let mut ranges: Option<Vec<(RangeUsize, $v)>> = None;

                for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                    let (id, val) = part.split_once(':').ok_or_else(|| {
                        concat!("Invalid format: Must be [n:]", $h, "[,m:", $h, "]...")
                    })?;
                    let val = val.parse::<$v>().map_err(|e| MuxError::from_any(e))?;

                    if let Ok(i) = id.parse::<usize>() {
                        idxs.get_or_insert_default().insert(i, val);
                    } else if let Ok(lang) = id.parse::<LangCode>() {
                        langs.get_or_insert_default().insert(lang, val);
                    } else if let Ok(rng) = id.parse::<RangeUsize>() {
                        ranges.get_or_insert_default().push((rng, val));
                    } else {
                        return Err(err!(
                            "Invalid stream ID '{}' (must be num, range (n-m) of num or lang code)",
                            id
                        ));
                    }
                }

                if idxs.is_none() && langs.is_none() && ranges.is_none() {
                    return Err(err!("No values found"));
                }

                Ok(Self(Metadata {
                    single_val: None,
                    idxs,
                    langs,
                    ranges,
                }))
            }
        }
    };
}

from_str_impl!(NameMetadata, String, "N");
from_str_impl!(LangMetadata, LangCode, "L");
