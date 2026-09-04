use super::*;
use log::LevelFilter;

fn new(args: &[&str]) -> ConfigLogLevel {
    cfg::<_, &&str>(args).log_level
}

#[test]
fn test_is_default() {
    use is_default::IsDefault;
    assert!(ConfigLogLevel::default().is_default());
    assert!(!ConfigLogLevel(LevelFilter::Error).is_default());
}

#[test]
fn parse_empty() {
    assert_eq!(ConfigLogLevel::default(), new(&[]).clone());
}

#[test]
fn parse_quiet() {
    assert_eq!(ConfigLogLevel(LevelFilter::Error), new(&["--quiet"]));
}

#[test]
fn parse_verbose() {
    assert_eq!(ConfigLogLevel(LevelFilter::Debug), new(&["-v"]));
    assert_eq!(ConfigLogLevel(LevelFilter::Trace), new(&["-vv"]));
    assert_eq!(
        ConfigLogLevel(LevelFilter::Trace),
        new(&["-vvvvvvvvvvvvvvvvvvv"])
    );
}
