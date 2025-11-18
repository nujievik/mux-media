use crate::common::*;
use mux_media::*;

fn new(args: &[&str]) -> AutoFlags {
    cfg::<_, &&str>(args).auto_flags
}

#[test]
fn test_empty() {
    let f = new(&[]);
    assert_eq!(Value::Auto(false), f.pro);
    assert_eq!(Value::Auto(true), f.defaults);
    assert_eq!(Value::Auto(true), f.forceds);
    assert_eq!(Value::Auto(true), f.names);
    assert_eq!(Value::Auto(true), f.langs);
    assert_eq!(Value::Auto(true), f.charsets);
}

#[test]
fn test_pro() {
    let f = new(&["--pro"]);
    assert_eq!(Value::User(true), f.pro);
    assert_eq!(Value::Auto(false), f.defaults);
    assert_eq!(Value::Auto(false), f.forceds);
    assert_eq!(Value::Auto(false), f.names);
    assert_eq!(Value::Auto(false), f.langs);
    assert_eq!(Value::Auto(false), f.charsets);
}

#[test]
fn test_manual_on() {
    let v = Value::User(true);
    assert_eq!(v, new(&["--auto-defaults"]).defaults);
    assert_eq!(v, new(&["--auto-forceds"]).forceds);
    assert_eq!(v, new(&["--auto-names"]).names);
    assert_eq!(v, new(&["--auto-langs"]).langs);
    assert_eq!(v, new(&["--auto-charsets"]).charsets);
}

#[test]
fn test_manual_off() {
    let v = Value::User(false);
    assert_eq!(v, new(&["--no-auto-defaults"]).defaults);
    assert_eq!(v, new(&["--no-auto-forceds"]).forceds);
    assert_eq!(v, new(&["--no-auto-names"]).names);
    assert_eq!(v, new(&["--no-auto-langs"]).langs);
    assert_eq!(v, new(&["--no-auto-charsets"]).charsets);
}

#[test]
fn test_manual_on_with_pro() {
    let v = Value::User(true);
    assert_eq!(v, new(&["--pro", "--auto-defaults"]).defaults);
    assert_eq!(v, new(&["--pro", "--auto-forceds"]).forceds);
    assert_eq!(v, new(&["--pro", "--auto-names"]).names);
    assert_eq!(v, new(&["--pro", "--auto-langs"]).langs);
    assert_eq!(v, new(&["--pro", "--auto-charsets"]).charsets);
}

crate::build_test_to_json_args!(
    test_to_json_args, auto_flags, "auto_flags";
    vec![],
    vec!["--no-auto-defaults"],
    vec!["--no-auto-forceds"],
    vec!["--no-auto-names"],
    vec!["--no-auto-langs"],
    vec!["--no-auto-charsets"],
    vec!["--pro", "--auto-defaults"],
    vec!["--pro", "--auto-forceds"],
    vec!["--pro", "--auto-names"],
    vec!["--pro", "--auto-langs"],
    vec!["--pro", "--auto-charsets"],
);
