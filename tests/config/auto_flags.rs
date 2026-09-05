use super::*;

fn new(args: &[&str]) -> ConfigAutoFlags {
    cfg::<_, &&str>(args).auto_flags
}

#[test]
fn test_empty() {
    let f = new(&[]);
    assert_eq!(false, f.no_auto);
    assert_eq!(Value::Auto(true), f.defaults);
    assert_eq!(Value::Auto(true), f.forceds);
    assert_eq!(Value::Auto(true), f.titles);
    assert_eq!(Value::Auto(true), f.langs);
    assert_eq!(Value::Auto(true), f.encs);
}

#[test]
fn test_no_auto() {
    let f = new(&["--no-auto"]);
    assert_eq!(true, f.no_auto);
    assert_eq!(Value::Auto(false), f.defaults);
    assert_eq!(Value::Auto(false), f.forceds);
    assert_eq!(Value::Auto(false), f.titles);
    assert_eq!(Value::Auto(false), f.langs);
    assert_eq!(Value::Auto(false), f.encs);
}

#[test]
fn test_manual_on() {
    let v = Value::User(true);
    assert_eq!(v, new(&["--auto-defaults"]).defaults);
    assert_eq!(v, new(&["--auto-forceds"]).forceds);
    assert_eq!(v, new(&["--auto-titles"]).titles);
    assert_eq!(v, new(&["--auto-langs"]).langs);
    assert_eq!(v, new(&["--auto-encs"]).encs);
}

#[test]
fn test_manual_off() {
    let v = Value::User(false);
    assert_eq!(v, new(&["--no-auto-defaults"]).defaults);
    assert_eq!(v, new(&["--no-auto-forceds"]).forceds);
    assert_eq!(v, new(&["--no-auto-titles"]).titles);
    assert_eq!(v, new(&["--no-auto-langs"]).langs);
    assert_eq!(v, new(&["--no-auto-encs"]).encs);
}

#[test]
fn test_manual_on_with_no_auto() {
    let v = Value::User(true);
    assert_eq!(v, new(&["--no-auto", "--auto-defaults"]).defaults);
    assert_eq!(v, new(&["--no-auto", "--auto-forceds"]).forceds);
    assert_eq!(v, new(&["--no-auto", "--auto-titles"]).titles);
    assert_eq!(v, new(&["--no-auto", "--auto-langs"]).langs);
    assert_eq!(v, new(&["--no-auto", "--auto-encs"]).encs);
}

crate::build_test_to_args!(
    test_to_args, "auto_flags";
    vec![],
    vec!["--no-auto-defaults"],
    vec!["--no-auto-forceds"],
    vec!["--no-auto-titles"],
    vec!["--no-auto-langs"],
    vec!["--no-auto-encs"],
    vec!["--no-auto", "--auto-defaults"],
    vec!["--no-auto", "--auto-forceds"],
    vec!["--no-auto", "--auto-titles"],
    vec!["--no-auto", "--auto-langs"],
    vec!["--no-auto", "--auto-encs"],
);
