use super::*;
use crate::{MuxError, Result};
use std::{error, str::FromStr};

impl<T> FromStr for ConfigMetadata<T>
where
    T: Clone + Debug + Display + PartialEq + IsDefault + FromStr,
    <T as FromStr>::Err: error::Error,
{
    type Err = MuxError;

    fn from_str(s: &str) -> Result<ConfigMetadata<T>> {
        let s = s.trim();

        if !s.contains(':') {
            let single_val = s.parse::<T>().map_err(|_| err!("fail parse"))?;

            return Ok(ConfigMetadata {
                single_val: Some(single_val),
                idxs: None,
                ranges: None,
                langs: None,
            });
        }

        let mut idxs: FxIndexMap<usize, T> = Default::default();
        let mut ranges: Vec<(RangeUsize, T)> = Vec::new();
        let mut langs: FxIndexMap<Lang, T> = Default::default();

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (id, val) = part
                .split_once(':')
                .ok_or_else(|| err!("invalid format. Must be [n:]T[,m:T]..."))?;

            let val = val.parse::<T>().map_err(|_| err!("fail parse"))?;

            if let Ok(i) = id.parse::<usize>() {
                idxs.insert(i, val);
            } else if let Ok(rng) = id.parse::<RangeUsize>() {
                ranges.push((rng, val));
            } else {
                langs.insert(Lang::new(id), val);
            }
        }

        Ok(ConfigMetadata {
            single_val: None,
            idxs: some_if_unempty!(idxs),
            langs: some_if_unempty!(langs),
            ranges: some_if_unempty!(ranges),
        })
    }
}

macro_rules! from_str_impl {
    ($ty:ty, $v:ty) => {
        impl FromStr for $ty {
            type Err = MuxError;

            fn from_str(s: &str) -> Result<$ty> {
                let meta = ConfigMetadata::<$v>::from_str(s)?;
                Ok(Self(meta))
            }
        }
    };
}

from_str_impl!(ConfigTitleMetadata, String);
from_str_impl!(ConfigLangMetadata, Lang);
