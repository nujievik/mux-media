use crate::{IsDefault, ToJsonArgs, TrackFlagType, Value};
use enum_map::{EnumMap, enum_map};

/// Values of auto-settings flags.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AutoFlags {
    pub pro: Value<bool>,
    pub track: EnumMap<TrackFlagType, Value<bool>>,
    pub names: Value<bool>,
    pub langs: Value<bool>,
    pub charsets: Value<bool>,
}

impl Default for AutoFlags {
    fn default() -> AutoFlags {
        AutoFlags {
            pro: Value::Auto(false),
            track: enum_map! {
                TrackFlagType::Default | TrackFlagType::Forced | TrackFlagType::Enabled => Value::Auto(true),
            },
            names: Value::Auto(true),
            langs: Value::Auto(true),
            charsets: Value::Auto(true),
        }
    }
}

impl IsDefault for AutoFlags {
    fn is_default(&self) -> bool {
        matches!(self.pro, Value::Auto(false))
            && matches!(self.names, Value::Auto(true))
            && matches!(self.langs, Value::Auto(true))
            && matches!(self.charsets, Value::Auto(true))
            && self.track.values().all(|&v| matches!(v, Value::Auto(true)))
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
            self.track[TrackFlagType::Default], AutoDefaults, NoAutoDefaults,
            self.track[TrackFlagType::Forced], AutoForceds, NoAutoForceds,
            self.track[TrackFlagType::Enabled], AutoEnableds, NoAutoEnableds,
            self.names, AutoNames, NoAutoNames,
            self.langs, AutoLangs, NoAutoLangs,
            self.charsets, AutoCharsets, NoAutoCharsets
        );
    }
}
