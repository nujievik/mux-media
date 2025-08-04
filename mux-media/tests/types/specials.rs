use crate::{common::*, *};
use mux_media::{markers::*, *};
use std::path::Path;

fn new(args: &[&str]) -> Specials {
    cfg::<_, &&str>(args).field::<MCSpecials>().clone()
}

#[test]
fn test_is_default() {
    assert!(Specials::default().is_default());
    assert!(new(&[]).is_default());
    assert!(!new(&["--specials", "-A"]).is_default());
}

#[test]
fn test_to_mkvmerge_args() {
    let cfg = cfg::<_, &&str>(&[]);
    let mut mi = MediaInfo::from(&cfg);
    let path = Path::new("1");

    assert!(
        Specials::default()
            .to_mkvmerge_args(&mut mi, path)
            .is_empty()
    );
    assert!(new(&[]).to_mkvmerge_args(&mut mi, path).is_empty());
    assert!(
        !new(&["--specials", "-A"])
            .to_mkvmerge_args(&mut mi, path)
            .is_empty()
    );
}

build_test_to_json_args!(
    test_to_json_args, MCSpecials, "specials";
    vec![],
    vec!["--specials", "--original-flag 0:1"],
    vec!["--specials", "--commentary-flag 0:1"],
    vec!["--specials", "--audio-tracks 0 --video-tracks 1 --subtitle-tracks 2"],
    vec!["--specials", "--subtitle-tracks 2 --audio-tracks 0 --video-tracks 1"]
);
