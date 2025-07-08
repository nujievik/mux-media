use crate::common::*;
use mux_media::*;
use std::{
    env,
    path::{Path, PathBuf},
};

fn new(args: &[&str]) -> Output {
    cfg::<_, &&str>(args).get::<MCOutput>().clone()
}

fn dir_empty() -> PathBuf {
    let mut dir = env::current_dir().unwrap();
    dir.push("muxed/");
    dir
}

fn build_test_empty_args(out: &Output, dir: PathBuf) {
    assert_eq!(false, out.need_num());
    assert_eq!(Path::new(""), out.get_temp_dir());

    ["1", "2", "abc", "n.am.e."].iter().for_each(|name| {
        let builded = out.build_out(name);
        assert_eq!(&dir, builded.parent().unwrap());
        assert_eq!(dir.join(format!("{}.mkv", name)), builded);
    })
}

#[test]
fn test_default() {
    let out = Output::default();
    build_test_empty_args(&out, PathBuf::from("./muxed/"));
}

#[test]
fn test_empty() {
    let out = new(&[]);
    build_test_empty_args(&out, dir_empty());
}

#[test]
fn test_try_finalize_init() {
    let dir = data_file("output/");
    let mut out = new(&["-o", &dir.to_str().unwrap()]);
    out.try_finalize_init().unwrap();

    assert_eq!(false, out.need_num());
    assert_eq!(&dir.join(".temp-mux-media"), out.get_temp_dir());

    ["1", "2", "abc", "n.am.e."].iter().for_each(|name| {
        let builded = out.build_out(name);
        assert_eq!(&dir, builded.parent().unwrap());
        assert_eq!(dir.join(format!("{}.mkv", name)), builded);
    })
}

/*
#[test]
fn test_try_finalize_init() {
    let mut out = new(&[""]);
    out.try_finalize_init();
}
*/

/*
use super::common::{cfg_empty, cfg};
use mux_media::{MCOutput, Output};
use std::path::PathBuf;

#[test]
fn test_build_out() {
    let path = data_file("file_,_part.mp4");
    let output = Output::try_from_path(&path).unwrap();

    let result = output.build_out("01");
    let expected = data_file("file_01_part.mp4");

    assert_eq!(result, expected);
}

/*
#[test]
fn test_err_not_writable_path() {
    let result = Output::try_from_path("/");

    if let Err(err) = result {
        let err_msg = format!("{}", err);
        assert!(
            err_msg.contains("is not writable"),
            "Expected error message to contain 'is not writable', but got: {}",
            err_msg
        );
    } else {
        panic!("Expected Err, but got Ok");
    }
}

#[test]
fn test_build_out_with_empty_dir() {
    let output = Output::try_from_path("file_,_part.mp4").unwrap();

    let result = output.build_out("01");
    let mut expected = PathBuf::from(std::env::current_dir().unwrap());
    expected.push("muxed");
    expected.push("file_01_part.mp4");

    assert_eq!(result, expected);
}
*/

#[test]
fn test_build_out_with_empty_tail() {
    let path = data_file("file_.mp4");
    let output = Output::try_from_path(&path).unwrap();

    let result = output.build_out("01");
    let expected = data_file("file_01.mp4");

    assert_eq!(result, expected);
}

#[test]
fn test_build_out_with_empty_ext() {
    let path = data_file("file_,_part");
    let output = Output::try_from_path(&path).unwrap();

    let result = output.build_out("01");
    let expected = data_file("file_01_part.mkv");

    assert_eq!(result, expected);
}

#[test]
fn test_build_out_empty() {
    let output = Output::try_from_path("").unwrap();

    let result = output.build_out("01");
    let mut expected = PathBuf::from(std::env::current_dir().unwrap());
    expected.push("muxed");
    expected.push("01.mkv");

    assert_eq!(result, expected);
}

/*
#[test]
fn test_temp_dir_access() {
    let path = data_file("file_,_part");
    let output = Output::try_from_path(&path).unwrap();

    let mut dir = data_dir();
    dir.push(".temp-mux-media/");
    assert_eq!(*output.get_temp_dir(), dir);
}
*/
*/
