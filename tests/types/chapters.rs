use crate::{common::*, *};
use mux_media::*;

fn new(file: &str) -> Chapters {
    let file = data(file);
    Chapters::try_from_file(&file).expect(&format!("Fail Chapters from '{}'", file.display()))
}

fn try_new(file: &str) -> Result<Chapters> {
    let file = data(file);
    Chapters::try_from_file(file)
}

#[test]
fn test_is_default() {
    use is_default::IsDefault;
    assert!(Chapters::default().is_default());
    assert!(!new("srt.srt").is_default());
}

#[test]
fn test_try_from_file() {
    assert!(try_new("srt.srt").is_ok());
    assert!(try_new("missing").is_err());
}

build_test_to_json_args!(
    test_to_json_args, chapters, "chapters";
    vec!["--no-chapters"],
    vec!["--chapters", data("srt.srt").to_str().unwrap()],
    vec!["--chapters", data("cp1251.srt").to_str().unwrap()],
    vec!["--chapters", data("audio_x1.mka").to_str().unwrap()],
    vec!["--chapters", data("video_x1.mkv").to_str().unwrap()]
);
