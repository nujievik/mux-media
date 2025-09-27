use crate::{IsDefault, ToJsonArgs, TrackFlagType, to_json_args};
use enum_map::{EnumMap, enum_map};

/// Values of auto-settings flags.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AutoFlags {
    pub pro: bool,
    pub track: EnumMap<TrackFlagType, bool>,
    pub names: bool,
    pub langs: bool,
    pub charsets: bool,
}

impl Default for AutoFlags {
    fn default() -> AutoFlags {
        AutoFlags {
            pro: false,
            track: enum_map! {
                TrackFlagType::Default | TrackFlagType::Forced | TrackFlagType::Enabled => true,
            },
            names: true,
            langs: true,
            charsets: true,
        }
    }
}

impl IsDefault for AutoFlags {
    fn is_default(&self) -> bool {
        !self.pro && self.names && self.langs && self.charsets && self.track.values().all(|&v| v)
    }
}

macro_rules! push_json_args {
    ($args:ident, $pro:expr; $( $val:expr, $arg:ident, $no_arg:ident ),*) => {{
        $(
            if $pro && $val {
                $args.push(to_json_args!($arg));
            } else if !$pro && !$val {
                $args.push(to_json_args!($no_arg));
            }
        )*
    }};
}

impl ToJsonArgs for AutoFlags {
    fn append_json_args(&self, args: &mut Vec<String>) {
        if self.pro {
            args.push(to_json_args!(Pro));
        }

        push_json_args!(
            args, self.pro;
            self.track[TrackFlagType::Default], AutoDefaults, NoAutoDefaults,
            self.track[TrackFlagType::Forced], AutoForceds, NoAutoForceds,
            self.track[TrackFlagType::Enabled], AutoEnableds, NoAutoEnableds,
            self.names, AutoNames, NoAutoNames,
            self.langs, AutoLangs, NoAutoLangs,
            self.charsets, AutoCharsets, NoAutoCharsets
        );
    }
}
