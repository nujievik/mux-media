use crate::{IsDefault, ToTxtConfig};
use log::LevelFilter;
use std::ffi::OsString;

/// A wrapper around [`log::LevelFilter`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigLogLevel(pub LevelFilter);

deref_singleton_tuple_struct!(ConfigLogLevel, LevelFilter);

impl ConfigLogLevel {
    pub(crate) fn from_count(cnt: u8) -> ConfigLogLevel {
        match cnt {
            0 => Self::default(),
            1 => Self(LevelFilter::Debug),
            _ => Self(LevelFilter::Trace),
        }
    }
}

impl Default for ConfigLogLevel {
    fn default() -> ConfigLogLevel {
        ConfigLogLevel(LevelFilter::Info)
    }
}
impl IsDefault for ConfigLogLevel {
    fn is_default(&self) -> bool {
        matches!(self.0, LevelFilter::Info)
    }
}

impl ToTxtConfig for ConfigLogLevel {
    fn append_args(&self, args: &mut Vec<OsString>) {
        match self.0 {
            LevelFilter::Off | LevelFilter::Error => args.push(to_args!(Quiet)),
            LevelFilter::Warn | LevelFilter::Info => (),
            LevelFilter::Debug => args.push("-v".into()),
            LevelFilter::Trace => args.push("-vv".into()),
        }
    }
}
