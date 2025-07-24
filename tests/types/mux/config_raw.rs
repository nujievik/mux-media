use mux_media::*;
use std::{ffi::OsString, path::Path};

pub fn new(args: &[&str]) -> RawMuxConfig {
    RawMuxConfig::try_from_args(args).expect("Error try_from_args()")
}

fn oss(args: &[&str]) -> Vec<OsString> {
    args.iter().map(|s| OsString::from(s)).collect()
}

#[test]
fn test_basic_split() {
    let raw = new(&[
        "--locale",
        "eng",
        "--list-langs",
        "--list-targets",
        "arg1",
        "arg2",
        "--target",
        "video",
        "--opt1",
        "--opt2",
        "--target",
        "audio",
        "--opt3",
        "--target",
        "global",
        "--g1",
        "--g2",
        "--mkvmerge",
        "-o",
        "out.mkv",
    ]);

    assert_eq!(Some(LangCode::Eng), raw.locale);
    assert_eq!(true, raw.list_langs);
    assert_eq!(true, raw.list_targets);
    assert_eq!(
        Some((Tool::Mkvmerge, oss(&["-o", "out.mkv"]))),
        raw.run_command
    );
    assert_eq!(
        raw.args,
        oss(&["--locale", "eng", "arg1", "arg2", "--g1", "--g2"])
    );

    let map = raw.trg_args.unwrap();
    assert_eq!(
        map.get(&Target::Group(TargetGroup::Video)).unwrap(),
        &oss(&["--opt1", "--opt2"])
    );
    assert_eq!(
        map.get(&Target::Group(TargetGroup::Audio)).unwrap(),
        &oss(&["--opt3"])
    );
}

#[test]
fn test_path_target() {
    let current_dir = Path::new(file!())
        .parent()
        .expect("Should get parent directory")
        .to_path_buf();

    let dir_str = current_dir.to_str().unwrap();

    let raw = new(&["--target", dir_str, "--x", "--y", "--mkvmerge"]);

    match raw.run_command {
        Some((Tool::Mkvmerge, _)) => {}
        _ => panic!("Expected mvkmerge tool"),
    }

    let canonical_dir = current_dir.canonicalize().unwrap();
    let target_key = Target::Path(canonical_dir.into());

    let map = raw.trg_args.unwrap();
    assert!(map.contains_key(&target_key));
    assert_eq!(map.get(&target_key).unwrap(), &oss(&["--x", "--y"]));
}

#[test]
fn test_subs_alias() {
    let raw = new(&[
        "--target",
        "subtitles",
        "--opt_sub",
        "--target",
        "subs",
        "--opt_s",
    ]);

    let map = raw.trg_args.unwrap();
    assert_eq!(
        map.get(&Target::Group(TargetGroup::Subs)).unwrap(),
        &oss(&["--opt_sub", "--opt_s"])
    );
}

#[test]
fn test_locale() {
    [
        (LangCode::Eng, "eng"),
        (LangCode::Rus, "rus"),
        (LangCode::Jpn, "jpn"),
    ]
    .into_iter()
    .for_each(|(lang, lng)| {
        let raw = new(&["--locale", lng]);
        assert_eq!(Some(lang), raw.locale);
        assert_eq!(false, raw.list_langs);
        assert_eq!(false, raw.list_targets);
        assert_eq!(None, raw.run_command);
        assert_eq!(oss(&["--locale", lng]), raw.args);
        assert_eq!(None, raw.trg_args);
    })
}

#[test]
fn test_only_tool() {
    let raw = new(&["--mkvmerge", "-h"]);
    assert_eq!(None, raw.locale);
    assert_eq!(false, raw.list_langs);
    assert_eq!(false, raw.list_targets);
    assert_eq!(Some((Tool::Mkvmerge, oss(&["-h"]))), raw.run_command);
    assert_eq!(oss(&[]), raw.args);
    assert_eq!(None, raw.trg_args);
}

#[test]
fn test_list_langs_flags() {
    [["--list-langs"], ["--list-languages"]]
        .iter()
        .for_each(|args| {
            let raw = new(args);
            assert_eq!(None, raw.locale);
            assert_eq!(true, raw.list_langs);
            assert_eq!(false, raw.list_targets);
            assert_eq!(None, raw.run_command);
            assert_eq!(oss(&[]), raw.args);
            assert_eq!(None, raw.trg_args);
        })
}

#[test]
fn test_list_targets_flag() {
    let raw = new(&["--list-targets"]);
    assert_eq!(None, raw.locale);
    assert_eq!(false, raw.list_langs);
    assert_eq!(true, raw.list_targets);
    assert_eq!(None, raw.run_command);
    assert_eq!(oss(&[]), raw.args);
    assert_eq!(None, raw.trg_args);
}

#[test]
fn test_fail_missing_path() {
    let result = RawMuxConfig::try_from_args(&["--target", "missing_path", "--opt"]);
    assert!(result.is_err());

    if let Err(err) = result {
        assert!(
            err.to_string().contains("Incorrect path target"),
            "Unexpected error message: {}",
            err
        );
    }
}

#[test]
fn test_args_before_target() {
    let raw = new(&["--arg1", "--target", "audio", "--opt"]);
    let map = raw.trg_args.unwrap();

    assert_eq!(raw.args, oss(&["--arg1"]));
    assert_eq!(
        map.get(&Target::Group(TargetGroup::Audio)).unwrap(),
        &oss(&["--opt"])
    );
}

#[test]
fn test_multiple_target_switching() {
    let raw = new(&[
        "init_arg", "--target", "audio", "--a1", "--target", "video", "--v1", "--target", "global",
        "--g1", "--target", "audio", "--a2", "--target", "video", "--v2",
    ]);

    let map = raw.trg_args.unwrap();

    assert_eq!(raw.args, oss(&["init_arg", "--g1"]));

    assert_eq!(
        map.get(&Target::Group(TargetGroup::Audio)).unwrap(),
        &oss(&["--a1", "--a2"])
    );
    assert_eq!(
        map.get(&Target::Group(TargetGroup::Video)).unwrap(),
        &oss(&["--v1", "--v2"])
    );
}

#[test]
fn test_empty_input() {
    let raw = new(&[]);
    assert_eq!(None, raw.locale);
    assert_eq!(false, raw.list_langs);
    assert_eq!(false, raw.list_targets);
    assert_eq!(None, raw.run_command);
    assert_eq!(oss(&[]), raw.args);
    assert_eq!(None, raw.trg_args);
}
