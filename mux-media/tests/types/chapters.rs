use super::common::data;
use crate::*;
use mux_media::markers::*;
use mux_media::*;

fn new(file: &str) -> Chapters {
    let file = data(file);
    Chapters::try_from_path(&file).expect(&format!("Fail Chapters from '{}'", file.display()))
}

fn try_new(file: &str) -> Result<Chapters, MuxError> {
    let file = data(file);
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
    format!("{}", try_new(file).unwrap_err())
}

#[test]
fn test_err_messages() {
    assert_eq!("Is not a file", err_msg(""));
    assert!(try_new("missing").is_err());
}

fn_variants_of_args!();

#[test]
fn test_to_mvkmerge_args() {
    ["srt.srt", "audio_x1.mka", "sub_x8.mks"]
        .iter()
        .for_each(|file| {
            let path = data(file);
            let path = path.to_str().unwrap();

            let cases = [
                (vec![], vec![]),
                (vec!["--no-chapters"], vec!["--no-chapters"]),
                (vec!["--chapters", path], vec!["--chapters", path]),
            ];

            compare_arg_cases!(cases, variants_of_args, file, MCChapters,);
        })
}

build_test_to_json_args!(
    test_to_json_args, MCChapters, "chapters";
    vec!["--no-chapters"],
    vec!["--chapters", data("srt.srt").to_str().unwrap()],
    vec!["--chapters", data("cp1251.srt").to_str().unwrap()],
    vec!["--chapters", data("audio_x1.mka").to_str().unwrap()],
    vec!["--chapters", data("video_x1.mkv").to_str().unwrap()]
);
