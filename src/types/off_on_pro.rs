use crate::{TFlagType, ToJsonArgs, from_arg_matches, json_arg};
use clap::{ArgMatches, Error, FromArgMatches};
use enum_map::{EnumMap, enum_map};

#[derive(Copy, Clone)]
pub struct OffOnPro {
    pub pro: bool,
    t_flags: EnumMap<TFlagType, bool>,
    pub add_names: bool,
    pub add_langs: bool,
    //pub sort_fonts: bool,
}

impl OffOnPro {
    pub fn add_t_flags(&self, ft: TFlagType) -> bool {
        self.t_flags[ft]
    }
}

macro_rules! upd_val {
    ($matches:ident, $old_pro:expr, $new_pro:expr, $old:expr, $add:ident, $no_add:ident) => {{
        if let Some(true) = from_arg_matches!($matches, bool, $add, @no_default) {
            true
        } else if let Some(true) = from_arg_matches!($matches, bool, $no_add, @no_default) {
            false
        } // Is manual value
        else if $old != $old_pro {
            $old
        } else {
            !$new_pro
        }
    }};
}

impl FromArgMatches for OffOnPro {
    from_arg_matches!(@fn);
    from_arg_matches!(@fn_update);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        let pro = from_arg_matches!(matches, bool, Pro, || false);

        let t_flags: EnumMap<TFlagType, bool> = enum_map! {
            TFlagType::Default => from_arg_matches!(matches, AddDefaults, NoAddDefaults, pro, @off_on_pro),
            TFlagType::Forced => from_arg_matches!(matches, AddForceds, NoAddForceds, pro, @off_on_pro),
            TFlagType::Enabled => from_arg_matches!(matches, AddEnableds, NoAddEnableds, pro, @off_on_pro),
        };

        let add_names = from_arg_matches!(matches, AddNames, NoAddNames, pro, @off_on_pro);
        let add_langs = from_arg_matches!(matches, AddLangs, NoAddLangs, pro, @off_on_pro);
        //let sort_fonts = from_arg_matches!(matches, SortFonts, NoSortFonts, pro, @off_on_pro);

        Ok(Self {
            pro,
            t_flags,
            add_names,
            add_langs,
            //sort_fonts,
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
            AddDefaults,
            NoAddDefaults
        );
        self.t_flags[TFlagType::Forced] = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.t_flags[TFlagType::Forced],
            AddForceds,
            NoAddForceds
        );
        self.t_flags[TFlagType::Enabled] = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.t_flags[TFlagType::Enabled],
            AddEnableds,
            NoAddEnableds
        );
        self.add_names = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.add_names,
            AddNames,
            NoAddNames
        );
        self.add_langs = upd_val!(
            matches,
            old_pro,
            new_pro,
            self.add_langs,
            AddLangs,
            NoAddLangs
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

impl ToJsonArgs for OffOnPro {
    fn to_json_args(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        if self.pro {
            args.push(json_arg!(Pro));
        }

        push!(
            args, self.pro;
            self.t_flags[TFlagType::Default], AddDefaults, NoAddDefaults,
            self.t_flags[TFlagType::Forced], AddForceds, NoAddForceds,
            self.t_flags[TFlagType::Enabled], AddEnableds, NoAddEnableds,
            self.add_names, AddNames, NoAddNames,
            self.add_langs, AddLangs, NoAddLangs
            //self.sort_fonts, SortFonts, NoSortFonts
        );

        args
    }
}
