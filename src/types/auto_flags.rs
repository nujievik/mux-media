use crate::{TFlagType, ToJsonArgs, from_arg_matches, json_arg};
use clap::{ArgMatches, Error, FromArgMatches};
use enum_map::{EnumMap, enum_map};

/// Settings for `Off on Pro` flags.
#[derive(Copy, Clone)]
pub struct AutoFlags {
    pub pro: bool,
    t_flags: EnumMap<TFlagType, bool>,
    pub auto_names: bool,
    pub auto_langs: bool,
    pub auto_charsets: bool,
}

impl AutoFlags {
    /// Returns a value of auto set track flag by type.
    pub fn auto_t_flags(&self, ft: TFlagType) -> bool {
        self.t_flags[ft]
    }
}

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

impl FromArgMatches for AutoFlags {
    from_arg_matches!(@fn);
    from_arg_matches!(@fn_update);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        let pro = from_arg_matches!(matches, bool, Pro, || false);

        let t_flags: EnumMap<TFlagType, bool> = enum_map! {
            TFlagType::Default => from_arg_matches!(matches, AutoDefaults, NoAutoDefaults, pro, @auto_flags),
            TFlagType::Forced => from_arg_matches!(matches, AutoForceds, NoAutoForceds, pro, @auto_flags),
            TFlagType::Enabled => from_arg_matches!(matches, AutoEnableds, NoAutoEnableds, pro, @auto_flags),
        };

        Ok(Self {
            pro,
            t_flags,
            auto_names: from_arg_matches!(matches, AutoNames, NoAutoNames, pro, @auto_flags),
            auto_langs: from_arg_matches!(matches, AutoLangs, NoAutoLangs, pro, @auto_flags),
            auto_charsets: from_arg_matches!(matches, AutoCharsets, NoAutoCharsets, pro, @auto_flags),
        })
    }

    fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error> {
        let old_pro = self.pro;
        let new_pro = from_arg_matches!(matches, bool, Pro, || old_pro);

        self.pro = new_pro;

        self.t_flags[TFlagType::Default] = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.t_flags[TFlagType::Default],
            AutoDefaults,
            NoAutoDefaults
        );
        self.t_flags[TFlagType::Forced] = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.t_flags[TFlagType::Forced],
            AutoForceds,
            NoAutoForceds
        );
        self.t_flags[TFlagType::Enabled] = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.t_flags[TFlagType::Enabled],
            AutoEnableds,
            NoAutoEnableds
        );
        self.auto_names = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.auto_names,
            AutoNames,
            NoAutoNames
        );
        self.auto_langs = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.auto_langs,
            AutoLangs,
            NoAutoLangs
        );
        self.auto_charsets = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.auto_langs,
            AutoCharsets,
            NoAutoCharsets
        );
        /*
        self.sort_fonts = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.sort_fonts,
            SortFonts,
            NoSortFonts
        );
        */

        Ok(())
    }
}

macro_rules! push {
    ($args:ident, $pro:expr; $( $val:expr, $arg:ident, $no_arg:ident ),*) => {{
        $(
            if $pro && $val {
                $args.push(json_arg!($arg));
            } else if !$pro && !$val {
                $args.push(json_arg!($no_arg));
            }
        )*
    }};
}

impl ToJsonArgs for AutoFlags {
    fn to_json_args(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        if self.pro {
            args.push(json_arg!(Pro));
        }

        push!(
            args, self.pro;
            self.t_flags[TFlagType::Default], AutoDefaults, NoAutoDefaults,
            self.t_flags[TFlagType::Forced], AutoForceds, NoAutoForceds,
            self.t_flags[TFlagType::Enabled], AutoEnableds, NoAutoEnableds,
            self.auto_names, AutoNames, NoAutoNames,
            self.auto_langs, AutoLangs, NoAutoLangs,
            self.auto_charsets, AutoCharsets, NoAutoCharsets
            //self.sort_fonts, SortFonts, NoSortFonts
        );

        args
    }
}
