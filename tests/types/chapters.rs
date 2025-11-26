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

fn err_msg(file: &str) -> String {
    format!("{}", try_new(file).unwrap_err())
}

#[test]
fn test_err_messages() {
    assert_eq!("Is not a file", err_msg(""));
    assert!(try_new("missing").is_err());
}

#[test]
fn test_to_ffmpeg_args() {
    ["srt.srt", "audio_x1.mka"].iter().for_each(|file| {
        let path = data(file);
        let path = path.to_str().unwrap();

        [
            (vec!["-i", path, "-map", "0:0", "-c:0", "copy"], vec![]),
            (
                vec![
                    "-i",
                    path,
                    "-map",
                    "0:0",
                    "-map_chapters",
                    "-1",
                    "-c:0",
                    "copy",
                ],
                vec!["--no-chapters"],
            ),
            (
                vec![
                    "-i",
                    path,
                    "-i",
                    path,
                    "-map",
                    "0:0",
                    "-map_chapters",
                    "1",
                    "-c:0",
                    "copy",
                ],
                vec!["--chapters", path],
            ),
        ]
        .into_iter()
        .for_each(|(exp, cli)| {
            let cfg = cfg(cli);
            let mut mi = MediaInfo::new(&cfg, 0);
            mi.try_insert(path).unwrap();
            assert_eq!(
                to_os_args(exp),
                StreamsOrder::to_ffmpeg_args(&mut mi).unwrap()
            );
        })
    })
}

build_test_to_json_args!(
    test_to_json_args, chapters, "chapters";
    vec!["--no-chapters"],
    vec!["--chapters", data("srt.srt").to_str().unwrap()],
    vec!["--chapters", data("cp1251.srt").to_str().unwrap()],
    vec!["--chapters", data("audio_x1.mka").to_str().unwrap()],
    vec!["--chapters", data("video_x1.mkv").to_str().unwrap()]
);
