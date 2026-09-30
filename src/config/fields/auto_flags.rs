use crate::{DispositionType, IsDefault, Result, ToArgs, Value};
use core::fmt::NumBuffer;
use enum_map::{EnumMap, enum_map};
use std::io::Write;

/// An auto-flags configuration.
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ConfigAutoFlags {
    pub no_auto: bool,
    pub defaults: Value<bool>,
    pub forceds: Value<bool>,
    pub titles: Value<bool>,
    pub langs: Value<bool>,
    pub encs: Value<bool>,
}

impl ConfigAutoFlags {
    pub(crate) fn map_dispositions(&self) -> EnumMap<DispositionType, bool> {
        enum_map!(DispositionType::Default => *self.defaults, DispositionType::Forced => *self.forceds )
    }
}

impl Default for ConfigAutoFlags {
    fn default() -> ConfigAutoFlags {
        ConfigAutoFlags {
            no_auto: false,
            defaults: Value::Auto(true),
            forceds: Value::Auto(true),
            titles: Value::Auto(true),
            langs: Value::Auto(true),
            encs: Value::Auto(true),
        }
    }
}
impl IsDefault for ConfigAutoFlags {
    fn is_default(&self) -> bool {
        matches!(self.no_auto, false)
            && matches!(self.defaults, Value::Auto(true))
            && matches!(self.forceds, Value::Auto(true))
            && matches!(self.titles, Value::Auto(true))
            && matches!(self.langs, Value::Auto(true))
            && matches!(self.encs, Value::Auto(true))
    }
}

macro_rules! write_args {
    ($writer:ident; $( $val:expr, $arg:ident, $no_arg:ident ),*) => {{
        $(
            let _ = match $val {
                Value::User(true) => to_args!($writer, $arg),
                Value::User(false) => to_args!($writer, $no_arg),
                _ => Ok(0),
            }?;
        )*
    }}
}

impl ToArgs for ConfigAutoFlags {
    fn write_with_num_buffer<W>(&self, w: &mut W, _: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if self.no_auto {
            to_args!(w, NoAuto)?;
        }

        write_args!(
            w;
            self.defaults, AutoDefaults, NoAutoDefaults,
            self.forceds, AutoForceds, NoAutoForceds,
            self.titles, AutoTitles, NoAutoTitles,
            self.langs, AutoLangs, NoAutoLangs,
            self.encs, AutoEncs, NoAutoEncs
        );

        Ok(())
    }
}
