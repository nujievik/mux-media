use super::common::cfg;
use crate::test_cli_args;
use mux_media::{CLIArg, IsDefault, MCRetiming, Retiming};

#[test]
fn test_cli_args() {
    test_cli_args!(Retiming; RmSegments => "rm-segments", NoLinked => "no-linked",
                   LessRetiming => "less-retiming");
}

fn new(args: &[&str]) -> Retiming {
    cfg::<_, &&str>(args).get::<MCRetiming>().clone()
}

#[test]
fn test_is_default() {
    assert!(new(&[]).is_default());
    assert!(!new(&["--no-linked"]).is_default());
}

#[test]
fn test_empty() {
    let rtm = new(&[]);
    assert!(rtm.rm_segments.is_none());
    assert!(!rtm.no_linked);
    assert!(!rtm.less);
}

#[test]
fn test_args() {
    assert!(new(&["--rm-segments", "*.srt"]).rm_segments.is_some());
    assert!(new(&["--no-linked"]).no_linked);
    assert!(new(&["--less-retiming"]).less);
}
