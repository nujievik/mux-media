use crate::{IsDefault, Result, ToArgs, dashed};
use core::fmt::NumBuffer;
use log::LevelFilter;
use std::io::Write;

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

    pub(crate) const fn is_need_info(&self) -> bool {
        matches!(
            self.0,
            LevelFilter::Info | LevelFilter::Debug | LevelFilter::Trace
        )
    }

    pub(crate) const fn is_need_debug(&self) -> bool {
        matches!(self.0, LevelFilter::Debug | LevelFilter::Trace)
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

impl ToArgs for ConfigLogLevel {
    fn write_with_num_buffer<W>(&self, w: &mut W, _: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        let s = match self.0 {
            LevelFilter::Off | LevelFilter::Error => dashed!(Quiet),
            LevelFilter::Warn | LevelFilter::Info => return Ok(()),
            LevelFilter::Debug => "-v",
            LevelFilter::Trace => "-vv",
        };

        to_args!(w, s.as_bytes(), @v)?;
        Ok(())
    }
}
