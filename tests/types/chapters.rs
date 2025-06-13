use super::common::{cfg_args, data_file, to_args};
use crate::test_cli_args;
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

#[test]
fn test_to_mkvmerge_args() {
    let cache = std::collections::HashMap::new();

    for file in ["srt.srt", "audio_x1.mka", "sub_x8.mks"] {
        let path = data_file(file);
        let path_str = path.to_str().unwrap();

        let cases = [
            (vec![], vec![]),
            (vec!["--no-chapters"], vec!["-C"]),
            (vec!["--no-chapters"], vec!["--no-chapters"]),
            (vec!["--chapters", path_str], vec!["-c", path_str]),
            (vec!["--chapters", path_str], vec!["--chapters", path_str]),
        ];

        for (expected, args) in cases {
            assert_eq!(
                to_args(expected),
                cfg_args::<MCChapters>(args, &path, cache.clone())
            );
        }
    }
}
