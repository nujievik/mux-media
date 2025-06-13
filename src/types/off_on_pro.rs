use crate::{CLIArg, cli_args, from_arg_matches};

#[derive(Clone, Copy)]
pub struct OffOnPro {
    pub pro: bool,
    pub add_defaults: bool,
    pub add_forceds: bool,
    pub add_enableds: bool,
    pub add_names: bool,
    pub add_langs: bool,
    pub sort_fonts: bool,
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

        let add_defaults = from_arg_matches!(matches, AddDefaults, NoAddDefaults, pro, @off_on_pro);
        let add_forceds = from_arg_matches!(matches, AddForceds, NoAddForceds, pro, @off_on_pro);
        let add_enableds = from_arg_matches!(matches, AddEnableds, NoAddEnableds, pro, @off_on_pro);
        let add_names = from_arg_matches!(matches, AddNames, NoAddNames, pro, @off_on_pro);
        let add_langs = from_arg_matches!(matches, AddLangs, NoAddLangs, pro, @off_on_pro);
        let sort_fonts = from_arg_matches!(matches, SortFonts, NoSortFonts, pro, @off_on_pro);

        Ok(Self {
            pro,
            add_defaults,
            add_forceds,
            add_enableds,
            add_names,
            add_langs,
            sort_fonts,
        })
    }
}
