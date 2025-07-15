use crate::common::*;
use mux_media::*;

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
    assert!(off.add_charsets);
    //assert!(off.sort_fonts);
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
    assert!(!off.add_charsets);
    //assert!(!off.sort_fonts);
}

#[test]
fn test_manual_on() {
    assert!(new(&["--add-defaults"]).add_t_flags(TFlagType::Default));
    assert!(new(&["--add-forceds"]).add_t_flags(TFlagType::Forced));
    assert!(new(&["--add-enableds"]).add_t_flags(TFlagType::Enabled));
    assert!(new(&["--add-names"]).add_names);
    assert!(new(&["--add-langs"]).add_langs);
    assert!(new(&["--add-charsets"]).add_charsets);
    //assert!(new(&["--sort-fonts"]).sort_fonts);
}

#[test]
fn test_manual_off() {
    assert!(!new(&["--no-add-defaults"]).add_t_flags(TFlagType::Default));
    assert!(!new(&["--no-add-forceds"]).add_t_flags(TFlagType::Forced));
    assert!(!new(&["--no-add-enableds"]).add_t_flags(TFlagType::Enabled));
    assert!(!new(&["--no-add-names"]).add_names);
    assert!(!new(&["--no-add-langs"]).add_langs);
    assert!(!new(&["--no-add-charsets"]).add_charsets);
    //assert!(!new(&["--no-sort-fonts"]).sort_fonts);
}

#[test]
fn test_manual_on_with_pro() {
    assert!(new(&["--pro", "--add-defaults"]).add_t_flags(TFlagType::Default));
    assert!(new(&["--pro", "--add-forceds"]).add_t_flags(TFlagType::Forced));
    assert!(new(&["--pro", "--add-enableds"]).add_t_flags(TFlagType::Enabled));
    assert!(new(&["--pro", "--add-names"]).add_names);
    assert!(new(&["--pro", "--add-langs"]).add_langs);
    assert!(new(&["--pro", "--add-charsets"]).add_charsets);
    //assert!(new(&["--pro", "--sort-fonts"]).sort_fonts);
}

crate::build_test_to_json_args!(
    test_to_json_args, MCOffOnPro, "off_on_pro";
    vec![],
    vec!["--no-add-defaults"],
    vec!["--no-add-forceds"],
    vec!["--no-add-enableds"],
    vec!["--no-add-names"],
    vec!["--no-add-langs"],
    vec!["--no-add-charsets"],
    //vec!["--no-sort-fonts"],
    vec!["--pro", "--add-defaults"],
    vec!["--pro", "--add-forceds"],
    vec!["--pro", "--add-enableds"],
    vec!["--pro", "--add-names"],
    vec!["--pro", "--add-langs"],
    vec!["--pro", "--add-charsets"],
    //vec!["--pro", "--sort-fonts"]
);
