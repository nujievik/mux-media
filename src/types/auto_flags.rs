use crate::{DispositionType, IsDefault, ToJsonArgs, Value};
use enum_map::{EnumMap, enum_map};

/// An auto-flags configuration.
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AutoFlags {
    pub pro: Value<bool>,
    pub defaults: Value<bool>,
    pub forceds: Value<bool>,
    pub names: Value<bool>,
    pub langs: Value<bool>,
    pub encs: Value<bool>,
}

impl AutoFlags {
    pub(crate) fn map_dispositions(&self) -> EnumMap<DispositionType, bool> {
        enum_map!(DispositionType::Default => *self.defaults, DispositionType::Forced => *self.forceds )
    }
}

impl Default for AutoFlags {
    fn default() -> AutoFlags {
        AutoFlags {
            pro: Value::Auto(false),
            defaults: Value::Auto(true),
            forceds: Value::Auto(true),
            names: Value::Auto(true),
            langs: Value::Auto(true),
            encs: Value::Auto(true),
        }
    }
}
impl IsDefault for AutoFlags {
    fn is_default(&self) -> bool {
        matches!(self.pro, Value::Auto(false))
            && matches!(self.defaults, Value::Auto(true))
            && matches!(self.forceds, Value::Auto(true))
            && matches!(self.names, Value::Auto(true))
            && matches!(self.langs, Value::Auto(true))
            && matches!(self.encs, Value::Auto(true))
    }
}

macro_rules! push_json_args {
    ($args:ident; $( $val:expr, $arg:ident, $no_arg:ident ),*) => {{
        $(
            match $val {
                Value::User(true) => $args.push(to_json_args!($arg)),
                Value::User(false) => $args.push(to_json_args!($no_arg)),
                _ => (),
            }
        )*
    }};
}

impl ToJsonArgs for AutoFlags {
    fn append_json_args(&self, args: &mut Vec<String>) {
        if *self.pro {
            args.push(to_json_args!(Pro));
        }

        push_json_args!(
            args;
            self.defaults, AutoDefaults, NoAutoDefaults,
            self.forceds, AutoForceds, NoAutoForceds,
            self.names, AutoNames, NoAutoNames,
            self.langs, AutoLangs, NoAutoLangs,
            self.encs, AutoEncs, NoAutoEncs
        );
    }
}
