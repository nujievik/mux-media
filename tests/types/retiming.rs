use crate::common::*;
use mux_media::markers::MCRetiming;
use mux_media::*;

fn new(args: &[&str]) -> Retiming {
    cfg::<_, &&str>(args).field::<MCRetiming>().clone()
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
}

#[test]
fn test_args() {
    assert!(new(&["--rm-segments", "*.srt"]).rm_segments.is_some());
    assert!(new(&["--no-linked"]).no_linked);
}

crate::build_test_to_json_args!(
    test_to_json_args, retiming, "retiming";
    vec!["--no-linked"],
    vec!["--rm-segments", "*.srt"],
    vec!["--rm-segments", "*.srt", "--no-linked"],
);
