use super::common::cfg;
use mux_media::{IsDefault, MCVerbosity, Verbosity};

fn new(args: &[&str]) -> Verbosity {
    cfg::<_, &&str>(args).get::<MCVerbosity>().clone()
}

#[test]
fn test_is_default() {
    assert!(Verbosity::default().is_default());
    assert!(!Verbosity::Quiet.is_default());
}

#[test]
fn test_default() {
    assert_eq!(Verbosity::default(), new(&[]).clone());
}

#[test]
fn test_quiet_flag() {
    assert_eq!(Verbosity::Quiet, new(&["--quiet"]));
    assert_eq!(Verbosity::Quiet, new(&["-q"]));
}

#[test]
fn test_verbose_flag() {
    assert_eq!(Verbosity::Verbose, new(&["--verbose"]));
    assert_eq!(Verbosity::Verbose, new(&["-v"]));
    assert_eq!(Verbosity::Debug, new(&["-vv"]));
    assert_eq!(Verbosity::Debug, new(&["-vvvvvvvvvvvvvvvvvvv"]));
}

#[test]
fn test_to_level_filter() {
    use log::LevelFilter;

    let cases = [
        (&["--quiet"], LevelFilter::Error),
        (&["-v"], LevelFilter::Debug),
        (&["-vv"], LevelFilter::Trace),
    ];

    for (args, lvl) in cases {
        assert_eq!(lvl, new(args).to_level_filter());
    }
    assert_eq!(LevelFilter::Info, Verbosity::default().to_level_filter());
}
