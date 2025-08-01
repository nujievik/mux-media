mod from_arg_matches;
mod special_opt;
mod to_json_args;
mod to_mkvmerge_args;

use crate::{IsDefault, MuxError};
use special_opt::SpecialOpt;
use std::str::FromStr;

/// Contains arbitrary valid mkvmerge arguments.
#[derive(Clone, Default, PartialEq, IsDefault)]
pub struct Specials(Option<Vec<String>>);

impl FromStr for Specials {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut specials = Vec::new();

        for part in s.split_whitespace() {
            if part.starts_with('-') {
                SpecialOpt::from_str(part.trim_start_matches('-'))
                    .map_err(|_| MuxError::from(format!("unexpected argument: '{}'", part)))?;
                specials.push(part.to_string());
            } else {
                specials.push(part.to_string());
            }
        }

        if specials.is_empty() {
            return Err("Not found any special option".into());
        }

        Ok(Self(Some(specials)))
    }
}
