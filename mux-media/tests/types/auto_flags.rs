use crate::common::*;
use mux_media::markers::*;
use mux_media::*;

fn new(args: &[&str]) -> AutoFlags {
    *cfg::<_, &&str>(args).field::<MCAutoFlags>()
}

#[test]
fn test_empty() {
    let off = new(&[]);
    assert!(!off.pro);
    assert!(off.auto_t_flags(TFlagType::Default));
    assert!(off.auto_t_flags(TFlagType::Forced));
    assert!(off.auto_t_flags(TFlagType::Enabled));
    assert!(off.auto_names);
    assert!(off.auto_langs);
    assert!(off.auto_charsets);
}

#[test]
fn test_pro() {
    let off = new(&["--pro"]);
    assert!(off.pro);
    assert!(!off.auto_t_flags(TFlagType::Default));
    assert!(!off.auto_t_flags(TFlagType::Forced));
    assert!(!off.auto_t_flags(TFlagType::Enabled));
    assert!(!off.auto_names);
    assert!(!off.auto_langs);
    assert!(!off.auto_charsets);
}

#[test]
fn test_manual_on() {
    assert!(new(&["--auto-defaults"]).auto_t_flags(TFlagType::Default));
    assert!(new(&["--auto-forceds"]).auto_t_flags(TFlagType::Forced));
    assert!(new(&["--auto-enableds"]).auto_t_flags(TFlagType::Enabled));
    assert!(new(&["--auto-names"]).auto_names);
    assert!(new(&["--auto-langs"]).auto_langs);
    assert!(new(&["--auto-charsets"]).auto_charsets);
}

#[test]
fn test_manual_off() {
    assert!(!new(&["--no-auto-defaults"]).auto_t_flags(TFlagType::Default));
    assert!(!new(&["--no-auto-forceds"]).auto_t_flags(TFlagType::Forced));
    assert!(!new(&["--no-auto-enableds"]).auto_t_flags(TFlagType::Enabled));
    assert!(!new(&["--no-auto-names"]).auto_names);
    assert!(!new(&["--no-auto-langs"]).auto_langs);
    assert!(!new(&["--no-auto-charsets"]).auto_charsets);
}

#[test]
fn test_manual_on_with_pro() {
    assert!(new(&["--pro", "--auto-defaults"]).auto_t_flags(TFlagType::Default));
    assert!(new(&["--pro", "--auto-forceds"]).auto_t_flags(TFlagType::Forced));
    assert!(new(&["--pro", "--auto-enableds"]).auto_t_flags(TFlagType::Enabled));
    assert!(new(&["--pro", "--auto-names"]).auto_names);
    assert!(new(&["--pro", "--auto-langs"]).auto_langs);
    assert!(new(&["--pro", "--auto-charsets"]).auto_charsets);
}

crate::build_test_to_json_args!(
    test_to_json_args, MCAutoFlags, "auto_flags";
    vec![],
    vec!["--no-auto-defaults"],
    vec!["--no-auto-forceds"],
    vec!["--no-auto-enableds"],
    vec!["--no-auto-names"],
    vec!["--no-auto-langs"],
    vec!["--no-auto-charsets"],
    vec!["--pro", "--auto-defaults"],
    vec!["--pro", "--auto-forceds"],
    vec!["--pro", "--auto-enableds"],
    vec!["--pro", "--auto-names"],
    vec!["--pro", "--auto-langs"],
    vec!["--pro", "--auto-charsets"],
);
