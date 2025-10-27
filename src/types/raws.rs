mod raw;
mod to_args;

use crate::{IsDefault, MuxError};
use raw::Raw;
use std::str::FromStr;

/// Contains arbitrary valid mkvmerge arguments.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct Raws(pub Option<Vec<String>>);

impl FromStr for Raws {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut raws = Vec::new();

        for part in s.split_whitespace() {
            if part.starts_with('-') {
                Raw::from_str(part.trim_start_matches('-'))
                    .map_err(|_| err!("unexpected argument: '{}'", part))?;
                raws.push(part.to_string());
            } else {
                raws.push(part.to_string());
            }
        }

        if raws.is_empty() {
            return Err("Not found any special option".into());
        }

        Ok(Self(Some(raws)))
    }
}
