use super::{LangMetadata, Metadata, NameMetadata};
use crate::{IsDefault, LangCode, MuxError, RangeUsize, Result};
use std::{
    collections::HashMap,
    error,
    fmt::{Debug, Display},
    str::FromStr,
};

impl<T> FromStr for Metadata<T>
where
    T: Clone + Debug + Display + PartialEq + IsDefault + FromStr,
    <T as FromStr>::Err: error::Error,
{
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Metadata<T>> {
        let s = s.trim();

        if !s.contains(':') {
            let single_val = s.parse::<T>().map_err(|e| MuxError::from_any(e))?;

            return Ok(Metadata {
                single_val: Some(single_val),
                idxs: None,
                ranges: None,
                langs: None,
            });
        }

        let mut idxs: Option<HashMap<usize, T>> = None;
        let mut langs: Option<HashMap<LangCode, T>> = None;
        let mut ranges: Option<Vec<(RangeUsize, T)>> = None;

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (id, val) = part
                .split_once(':')
                .ok_or_else(|| "Invalid format: Must be [n:]T[,m:T]...")?;
            let val = val.parse::<T>().map_err(|e| MuxError::from_any(e))?;

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

        Ok(Metadata {
            single_val: None,
            idxs,
            langs,
            ranges,
        })
    }
}

macro_rules! from_str_impl {
    ($ty:ty, $v:ty) => {
        impl FromStr for $ty {
            type Err = MuxError;

            fn from_str(s: &str) -> Result<$ty> {
                let meta = Metadata::from_str(s)?;
                Ok(Self(meta))
            }
        }
    };
}

from_str_impl!(NameMetadata, String);
from_str_impl!(LangMetadata, LangCode);
