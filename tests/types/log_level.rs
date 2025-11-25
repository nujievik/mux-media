use crate::common::cfg;
use log::LevelFilter;
use mux_media::*;

fn new(args: &[&str]) -> LogLevel {
    cfg::<_, &&str>(args).log_level
}

#[test]
fn test_is_default() {
    use is_default::IsDefault;
    assert!(LogLevel::default().is_default());
    assert!(!LogLevel(LevelFilter::Error).is_default());
}

#[test]
fn parse_empty() {
    assert_eq!(LogLevel::default(), new(&[]).clone());
}

#[test]
fn parse_quiet() {
    assert_eq!(LogLevel(LevelFilter::Error), new(&["--quiet"]));
}

#[test]
fn parse_verbose() {
    assert_eq!(LogLevel(LevelFilter::Debug), new(&["-v"]));
    assert_eq!(LogLevel(LevelFilter::Trace), new(&["-vv"]));
    assert_eq!(LogLevel(LevelFilter::Trace), new(&["-vvvvvvvvvvvvvvvvvvv"]));
}
