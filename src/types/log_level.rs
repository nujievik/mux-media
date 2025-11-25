use crate::{IsDefault, ToJsonArgs};
use log::LevelFilter;

/// A wrapper around [`log::LevelFilter`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LogLevel(pub LevelFilter);

deref_singleton_tuple_struct!(LogLevel, LevelFilter);

impl LogLevel {
    pub(crate) fn from_count(cnt: u8) -> LogLevel {
        match cnt {
            0 => Self::default(),
            1 => Self(LevelFilter::Debug),
            _ => Self(LevelFilter::Trace),
        }
    }
}

impl Default for LogLevel {
    fn default() -> LogLevel {
        LogLevel(LevelFilter::Info)
    }
}
impl IsDefault for LogLevel {
    fn is_default(&self) -> bool {
        matches!(self.0, LevelFilter::Info)
    }
}

impl ToJsonArgs for LogLevel {
    fn append_json_args(&self, args: &mut Vec<String>) {
        match self.0 {
            LevelFilter::Off | LevelFilter::Error => args.push(to_json_args!(Quiet)),
            LevelFilter::Warn | LevelFilter::Info => (),
            LevelFilter::Debug => args.push("-v".into()),
            LevelFilter::Trace => args.push("-vv".into()),
        }
    }
}
