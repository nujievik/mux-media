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

/*
macro_rules! upd_val {
    ($matches:ident, $old_pro:expr, $new_pro:expr, $old:expr, $auto:ident, $no_auto:ident) => {{
        if let Some(true) = from_arg_matches!($matches, bool, $auto, @no_default) {
            true
        } else if let Some(true) = from_arg_matches!($matches, bool, $no_auto, @no_default) {
            false
        } // Is manual value
        else if $old != $old_pro {
            $old
        } else {
            !$new_pro
        }
    }};
}
*/

/*
impl FromArgMatches for AutoFlags {
    from_arg_matches!(@fn);
    from_arg_matches!(@fn_update);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        let pro = from_arg_matches!(matches, bool, Pro, || false);

        let track: EnumMap<TrackFlagType, bool> = enum_map! {
            TrackFlagType::Default => from_arg_matches!(matches, AutoDefaults, NoAutoDefaults, pro, @auto_flags),
            TrackFlagType::Forced => from_arg_matches!(matches, AutoForceds, NoAutoForceds, pro, @auto_flags),
            TrackFlagType::Enabled => from_arg_matches!(matches, AutoEnableds, NoAutoEnableds, pro, @auto_flags),
        };

        Ok(Self {
            pro,
            track,
            names: from_arg_matches!(matches, AutoNames, NoAutoNames, pro, @auto_flags),
            langs: from_arg_matches!(matches, AutoLangs, NoAutoLangs, pro, @auto_flags),
            charsets: from_arg_matches!(matches, AutoCharsets, NoAutoCharsets, pro, @auto_flags),
        })
    }

    fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error> {
        let old_pro = self.pro;
        let new_pro = from_arg_matches!(matches, bool, Pro, || old_pro);

        self.pro = new_pro;

        self.track[TrackFlagType::Default] = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.track[TrackFlagType::Default],
            AutoDefaults,
            NoAutoDefaults
        );
        self.track[TrackFlagType::Forced] = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.track[TrackFlagType::Forced],
            AutoForceds,
            NoAutoForceds
        );
        self.track[TrackFlagType::Enabled] = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.track[TrackFlagType::Enabled],
            AutoEnableds,
            NoAutoEnableds
        );
        self.names = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.names,
            AutoNames,
            NoAutoNames
        );
        self.langs = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.langs,
            AutoLangs,
            NoAutoLangs
        );
        self.charsets = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.charsets,
            AutoCharsets,
            NoAutoCharsets
        );

        Ok(())
    }
}
*/

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
