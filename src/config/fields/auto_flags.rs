use crate::{DispositionType, IsDefault, ToTxtConfig, Value};
use enum_map::{EnumMap, enum_map};
use std::ffi::OsString;

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

macro_rules! push_args {
    ($args:ident; $( $val:expr, $arg:ident, $no_arg:ident ),*) => {{
        $(
            match $val {
                Value::User(true) => $args.push(to_args!($arg)),
                Value::User(false) => $args.push(to_args!($no_arg)),
                _ => (),
            }
        )*
    }};
}

impl ToTxtConfig for ConfigAutoFlags {
    fn append_args(&self, args: &mut Vec<OsString>) {
        if self.no_auto {
            args.push(to_args!(NoAuto));
        }

        push_args!(
            args;
            self.defaults, AutoDefaults, NoAutoDefaults,
            self.forceds, AutoForceds, NoAutoForceds,
            self.titles, AutoTitles, NoAutoTitles,
            self.langs, AutoLangs, NoAutoLangs,
            self.encs, AutoEncs, NoAutoEncs
        );
    }
}
