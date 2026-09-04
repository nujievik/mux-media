use super::*;

fn new(args: &[&str]) -> ConfigRetiming {
    cfg::<_, &&str>(args).retiming
}

#[test]
fn test_is_default() {
    assert!(new(&[]).is_default());
    assert!(!new(&["--no-linked"]).is_default());
}

#[test]
fn test_empty() {
    let rtm = new(&[]);
    assert!(!rtm.no_linked);
    assert!(!rtm.parts.inverse);
    assert!(rtm.parts.pattern.is_none());
}

#[test]
fn test_args() {
    assert!(new(&["--no-linked"]).no_linked);

    let parts = Some("*.srt".parse::<GlobSetPattern>().unwrap());
    assert_eq!(parts, new(&["--parts", "*.srt"]).parts.pattern);
}

crate::build_test_to_args!(
    test_to_args, retiming, "retiming";
    vec![],
    vec!["--no-linked"],
    vec!["--parts", "*.srt"],
    vec!["--parts", "*.srt", "--no-linked"],
);
