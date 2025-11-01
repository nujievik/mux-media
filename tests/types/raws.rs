use crate::{common::*, *};
use mux_media::*;
use std::path::Path;

fn new(args: &[&str]) -> Raws {
    cfg::<_, &&str>(args).raws
}

#[test]
fn test_is_default() {
    assert!(Raws::default().is_default());
    assert!(new(&[]).is_default());
    assert!(!new(&["--raws", "-A"]).is_default());
}

build_test_to_json_args!(
    test_to_json_args, raws, "raws";
    vec![],
    vec!["--raws", "--original-flag 0:1"],
    vec!["--raws", "--commentary-flag 0:1"],
    vec!["--raws", "--audio-tracks 0 --video-tracks 1 --subtitle-tracks 2"],
    vec!["--raws", "--subtitle-tracks 2 --audio-tracks 0 --video-tracks 1"]
);

#[test]
fn test_err_missing_arg() {
    use clap::*;
    MuxConfig::try_parse_from(["--raws", "--missing"]).unwrap_err();
}
