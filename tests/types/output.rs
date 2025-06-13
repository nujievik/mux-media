/*
use super::common::{cfg_empty, cfg};
use mux_media::{MCOutput, Output};
use std::path::PathBuf;

fn new(args: &[&str]) -> Output {
    cfg(args).get::<MCOutput>().clone()
}

fn new_empty() -> Output {
    cfg_empty().get::<MCOutput>().clone()
}

fn dir_empty() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap();
    dir.push("muxed/");
    dir
}

#[test]
fn test_empty() {
    let dir_empty = dir_empty();
    let out = new_empty();
    assert_eq!(&dir_empty, out.build_out("1").parent().unwrap());
}
*/

/*
#[test]
fn test_build_from_empty_cfg() {
    let cfg = cfg_empty();
    let output = cfg.get::<MCOutput>();
    let result = output.build_out("01");
    let expected = data_file("muxed/01.mkv");

    assert_eq!(result, expected);
}

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
