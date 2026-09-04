use crate::{IsDefault, ToTxtConfig, dashed};
use std::ffi::OsString;

/// A chapters configuration.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
#[non_exhaustive]
pub struct ConfigChapters {
    pub no_flag: bool,
}

impl ToTxtConfig for ConfigChapters {
    fn append_args(&self, args: &mut Vec<OsString>) {
        if self.no_flag {
            args.push(dashed!(NoChapters).into());
        }
    }
}
