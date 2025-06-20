use crate::{TFlagType, cli_args, from_arg_matches};
use enum_map::{EnumMap, enum_map};

#[derive(Copy, Clone)]
pub struct OffOnPro {
    pub pro: bool,
    t_flags: EnumMap<TFlagType, bool>,
    pub add_names: bool,
    pub add_langs: bool,
    pub sort_fonts: bool,
}

impl OffOnPro {
    pub fn add_t_flags(&self, ft: TFlagType) -> bool {
        self.t_flags[ft]
    }
}

cli_args!(
    OffOnPro, OffOnProArg;
    Pro => "pro", HelpAddDefaults => "add-defaults / --no-add-defaults",
    AddDefaults => "add-defaults", NoAddDefaults => "no-add-defaults",
    HelpAddForceds => "add-forceds / --no-add-forceds", AddForceds => "add-forceds",
    NoAddForceds => "no-add-forceds", HelpAddEnableds => "add-enableds / --no-add-enableds",
    AddEnableds => "add-enableds", NoAddEnableds => "no-add-enableds",
    HelpAddNames => "add-names / --no-add-names", AddNames => "add-names",
    NoAddNames => "no-add-names", HelpAddLangs => "add-langs / --no-add-langs",
    AddLangs => "add-langs", NoAddLangs => "no-add-langs",
    HelpSortFonts => "sort-fonts / --no-sort-fonts", SortFonts => "sort-fonts",
    NoSortFonts => "no-sort-fonts"
);

impl clap::FromArgMatches for OffOnPro {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        let pro = from_arg_matches!(matches, bool, Pro, || false);

        let t_flags: EnumMap<TFlagType, bool> = enum_map! {
            TFlagType::Default => from_arg_matches!(matches, AddDefaults, NoAddDefaults, pro, @off_on_pro),
            TFlagType::Forced => from_arg_matches!(matches, AddForceds, NoAddForceds, pro, @off_on_pro),
            TFlagType::Enabled => from_arg_matches!(matches, AddEnableds, NoAddEnableds, pro, @off_on_pro),
        };

        let add_names = from_arg_matches!(matches, AddNames, NoAddNames, pro, @off_on_pro);
        let add_langs = from_arg_matches!(matches, AddLangs, NoAddLangs, pro, @off_on_pro);
        let sort_fonts = from_arg_matches!(matches, SortFonts, NoSortFonts, pro, @off_on_pro);

        Ok(Self {
            pro,
            t_flags,
            add_names,
            add_langs,
            sort_fonts,
        })
    }
}
