use super::common::data_file;
use crate::{compare_arg_cases, fn_variants_of_args, test_cli_args};
use mux_media::*;

fn new(file: &str) -> Chapters {
    let file = data_file(file);
    Chapters::try_from_path(&file).expect(&format!("Fail Chapters from '{}'", file.display()))
}

fn try_new(file: &str) -> Result<Chapters, MuxError> {
    let file = data_file(file);
    Chapters::try_from_path(file)
}

#[test]
fn test_is_default() {
    assert!(Chapters::default().is_default());
    assert!(!new("srt.srt").is_default());
}

#[test]
fn test_try_from_path() {
    assert!(try_new("srt.srt").is_ok());
    assert!(try_new("missing").is_err());
}

fn err_msg(file: &str) -> String {
    try_new(file).unwrap_err().message.unwrap()
}

#[test]
fn test_err_messages() {
    assert_eq!("Is not a file", err_msg(""));
    assert!(err_msg("missing").contains("No such file or directory"));
}

#[test]
fn test_cli_args() {
    test_cli_args!(Chapters; Chapters => "chapters", "--chapters",
                   NoChapters => "no-chapters", "--no-chapters");
}

fn_variants_of_args!("-c" => vec!["--chapters"], "-C" => vec!["--no-chapters"]);

#[test]
fn test_to_mvkmerge_args() {
    for file in ["srt.srt", "audio_x1.mka", "sub_x8.mks"] {
        let path = data_file(file);
        let path = path.to_str().unwrap();

        let cases = [
            (vec![], vec![]),
            (vec!["--no-chapters"], vec!["-C"]),
            (vec!["--no-chapters"], vec!["--no-chapters"]),
            (vec!["--chapters", path], vec!["-c", path]),
            (vec!["--chapters", path], vec!["--chapters", path]),
        ];

        compare_arg_cases!(cases, variants_of_args, file, MCChapters,);
    }
}
