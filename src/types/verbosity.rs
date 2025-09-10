use crate::{IsDefault, ToJsonArgs, to_json_args};
use log::LevelFilter;

/// Defines logging level.
#[derive(Copy, Clone, Debug, Default, PartialEq, IsDefault)]
pub enum Verbosity {
    Quiet,
    #[default]
    Normal,
    Verbose,
    Debug,
}

impl From<u8> for Verbosity {
    fn from(count: u8) -> Self {
        match count {
            0 => Verbosity::default(),
            1 => Verbosity::Verbose,
            _ => Verbosity::Debug,
        }
    }
}

impl From<Verbosity> for LevelFilter {
    fn from(val: Verbosity) -> Self {
        match val {
            Verbosity::Quiet => LevelFilter::Error,
            Verbosity::Normal => LevelFilter::Info,
            Verbosity::Verbose => LevelFilter::Debug,
            Verbosity::Debug => LevelFilter::Trace,
        }
    }
}

impl ToJsonArgs for Verbosity {
    fn append_json_args(&self, args: &mut Vec<String>) {
        match self {
            Self::Quiet => args.push(to_json_args!(Quiet)),
            Self::Normal => (),
            Self::Verbose => args.push("-v".to_owned()),
            Self::Debug => args.push("-vv".to_owned()),
        }
    }
}
