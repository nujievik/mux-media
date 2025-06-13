use crate::common::cfg;
use crate::test_cli_args;
use mux_media::{CLIArg, IsDefault, MCSpecials, MediaInfo, Specials, ToMkvmergeArgs};
use std::path::Path;

fn new(args: &[&str]) -> Specials {
    cfg::<_, &&str>(args).get::<MCSpecials>().clone()
}

#[test]
fn test_cli_args() {
    test_cli_args!(Specials; Specials => "specials");
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
