use super::common::cfg;
use crate::test_cli_args;
use mux_media::{CLIArg, MCOffOnPro, OffOnPro, TFlagType};

#[test]
fn test_cli_args() {
    test_cli_args!(
        OffOnPro;
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
}

fn new(args: &[&str]) -> OffOnPro {
    *cfg::<_, &&str>(args).get::<MCOffOnPro>()
}

#[test]
fn test_empty() {
    let off = new(&[]);
    assert!(!off.pro);
    assert!(off.add_t_flags(TFlagType::Default));
    assert!(off.add_t_flags(TFlagType::Forced));
    assert!(off.add_t_flags(TFlagType::Enabled));
    assert!(off.add_names);
    assert!(off.add_langs);
    assert!(off.sort_fonts);
}

#[test]
fn test_pro() {
    let off = new(&["--pro"]);
    assert!(off.pro);
    assert!(!off.add_t_flags(TFlagType::Default));
    assert!(!off.add_t_flags(TFlagType::Forced));
    assert!(!off.add_t_flags(TFlagType::Enabled));
    assert!(!off.add_names);
    assert!(!off.add_langs);
    assert!(!off.sort_fonts);
}

#[test]
fn test_manual_on() {
    assert!(new(&["--add-defaults"]).add_t_flags(TFlagType::Default));
    assert!(new(&["--add-forceds"]).add_t_flags(TFlagType::Forced));
    assert!(new(&["--add-enableds"]).add_t_flags(TFlagType::Enabled));
    assert!(new(&["--add-names"]).add_names);
    assert!(new(&["--add-langs"]).add_langs);
    assert!(new(&["--sort-fonts"]).sort_fonts);
}

#[test]
fn test_manual_off() {
    assert!(!new(&["--no-add-defaults"]).add_t_flags(TFlagType::Default));
    assert!(!new(&["--no-add-forceds"]).add_t_flags(TFlagType::Forced));
    assert!(!new(&["--no-add-enableds"]).add_t_flags(TFlagType::Enabled));
    assert!(!new(&["--no-add-names"]).add_names);
    assert!(!new(&["--no-add-langs"]).add_langs);
    assert!(!new(&["--no-sort-fonts"]).sort_fonts);
}

#[test]
fn test_manual_on_with_pro() {
    assert!(new(&["--pro", "--add-defaults"]).add_t_flags(TFlagType::Default));
    assert!(new(&["--pro", "--add-forceds"]).add_t_flags(TFlagType::Forced));
    assert!(new(&["--pro", "--add-enableds"]).add_t_flags(TFlagType::Enabled));
    assert!(new(&["--pro", "--add-names"]).add_names);
    assert!(new(&["--pro", "--add-langs"]).add_langs);
    assert!(new(&["--pro", "--sort-fonts"]).sort_fonts);
}
