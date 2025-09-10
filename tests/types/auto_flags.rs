use crate::common::*;
use mux_media::*;

fn new(args: &[&str]) -> AutoFlags {
    cfg::<_, &&str>(args).auto_flags
}

#[test]
fn test_empty() {
    let f = new(&[]);
    assert!(!f.pro);
    assert!(f.track[TrackFlagType::Default]);
    assert!(f.track[TrackFlagType::Forced]);
    assert!(f.track[TrackFlagType::Enabled]);
    assert!(f.names);
    assert!(f.langs);
    assert!(f.charsets);
}

#[test]
fn test_pro() {
    let f = new(&["--pro"]);
    assert!(f.pro);
    assert!(!f.track[TrackFlagType::Default]);
    assert!(!f.track[TrackFlagType::Forced]);
    assert!(!f.track[TrackFlagType::Enabled]);
    assert!(!f.names);
    assert!(!f.langs);
    assert!(!f.charsets);
}

#[test]
fn test_manual_on() {
    assert!(new(&["--auto-defaults"]).track[TrackFlagType::Default]);
    assert!(new(&["--auto-forceds"]).track[TrackFlagType::Forced]);
    assert!(new(&["--auto-enableds"]).track[TrackFlagType::Enabled]);
    assert!(new(&["--auto-names"]).names);
    assert!(new(&["--auto-langs"]).langs);
    assert!(new(&["--auto-charsets"]).charsets);
}

#[test]
fn test_manual_off() {
    assert!(!new(&["--no-auto-defaults"]).track[TrackFlagType::Default]);
    assert!(!new(&["--no-auto-forceds"]).track[TrackFlagType::Forced]);
    assert!(!new(&["--no-auto-enableds"]).track[TrackFlagType::Enabled]);
    assert!(!new(&["--no-auto-names"]).names);
    assert!(!new(&["--no-auto-langs"]).langs);
    assert!(!new(&["--no-auto-charsets"]).charsets);
}

#[test]
fn test_manual_on_with_pro() {
    assert!(new(&["--pro", "--auto-defaults"]).track[TrackFlagType::Default]);
    assert!(new(&["--pro", "--auto-forceds"]).track[TrackFlagType::Forced]);
    assert!(new(&["--pro", "--auto-enableds"]).track[TrackFlagType::Enabled]);
    assert!(new(&["--pro", "--auto-names"]).names);
    assert!(new(&["--pro", "--auto-langs"]).langs);
    assert!(new(&["--pro", "--auto-charsets"]).charsets);
}

crate::build_test_to_json_args!(
    test_to_json_args, auto_flags, "auto_flags";
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
