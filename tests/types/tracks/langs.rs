use super::id;
use crate::common::*;
use crate::*;
use mux_media::*;

#[test]
fn test_cli_args() {
    test_cli_args!(TrackLangs; Langs => "langs");
}

#[test]
fn test_is_default() {
    assert!(TrackLangs::default().is_default());
    assert!(from_cfg::<MCTrackLangs>(vec![]).is_default());
    assert!(!from_cfg::<MCTrackLangs>(vec!["--langs", "eng"]).is_default());
    assert!(!from_cfg::<MCTrackLangs>(vec!["--langs", "0:en,1:ru"]).is_default());
    assert!(!from_cfg::<MCTrackLangs>(vec!["--langs", "0-8:en"]).is_default());
}

test_from_str!(
    TrackLangs,
    test_from_str,
    ["eng", "rus", "0-8:eng", "1:jpn,2:en,3:ru", "ru:ru,en:jpn"],
    ["missing", "1:miss,2:miss", "miss:en,amiss:ru"]
);

fn from_str(s: &str) -> TrackLangs {
    s.parse::<TrackLangs>().unwrap()
}

#[test]
fn test_get() {
    let cases = [
        ("eng", vec!["0", "8", "eng", "rus", MAX_U64_STR]),
        ("0:en,1:ru,8:jpn", vec!["0", "1", "8"]),
        ("en:en,ru:en,jpn:en", vec!["eng", "rus", "jpn"]),
        ("0-8:en", vec!["0", "1", "8"]),
        ("0-8:en,eng:ru", vec!["0", "eng"]),
    ];

    for (s_names, s_tids) in cases {
        let names = from_str(s_names);
        for s_tid in s_tids {
            let tid = id(s_tid);
            assert!(
                names.get(&tid).is_some(),
                "Fail get '{}' from names '{}'",
                s_tid,
                s_names
            );
        }
    }

    let bad_cases = [
        ("0:en,1:ru,8:jpn", vec!["2", "7", "9", MAX_U64_STR, "eng"]),
        ("en:en,ru:en,jpn:en", vec!["und", "fr", "0", "8"]),
        ("0-8:en", vec!["9", "rus", MAX_U64_STR]),
        ("0-8:ru,eng:en", vec!["9", "rus", "jpn"]),
    ];

    for (s_names, s_tids) in bad_cases {
        let names = from_str(s_names);
        for s_tid in s_tids {
            let tid = id(s_tid);
            assert!(
                names.get(&tid).is_none(),
                "Fail None get() '{}' from names '{}'",
                s_tid,
                s_names
            );
        }
    }
}

fn_variants_of_args!("--langs" => vec!["--languages"]);

fn build_test_x1_to_mkvmerge_args(file: &str) {
    let cases = [
        (vec![], vec!["--pro"]),
        (vec![], vec!["--pro", "--langs", "eng:en"]),
        (vec![], vec!["--pro", "--langs", "2:en"]),
        (
            vec!["--language", "0:eng"],
            vec!["--pro", "--langs", "0:eng"],
        ),
        (vec!["--language", "0:eng"], vec!["--langs", "en"]),
        (vec!["--language", "0:eng"], vec!["--langs", "0:en"]),
        (vec!["--language", "0:eng"], vec!["--langs", "und:en"]),
        (vec!["--language", "0:eng"], vec!["--langs", "0-8:en"]),
        (
            vec!["--language", "0:eng"],
            vec!["--langs", "0:en,1:ru,2:en"],
        ),
        (vec!["--language", "0:rus"], vec!["--langs", "0:rus"]),
    ];

    compare_arg_cases!(cases, variants_of_args, file, MCTrackLangs, MITracksInfo);
}

#[test]
fn test_x1_audio_to_mkvmerge_args() {
    build_test_x1_to_mkvmerge_args("audio_x1.mka");
}

#[test]
fn test_x1_sub_to_mkvmerge_args() {
    build_test_x1_to_mkvmerge_args("sub_x1.mks");
}

#[test]
fn test_x1_video_to_mkvmerge_args() {
    build_test_x1_to_mkvmerge_args("video_x1.mkv");
}

fn build_test_x8_to_mkvmerge_args(file: &str) {
    let cases = [
        (vec![], vec!["--pro"]),
        (vec![], vec!["--pro", "--langs", "eng:en"]),
        (
            to_args(["--language", "0:eng"]),
            vec!["--pro", "--langs", "0:en"],
        ),
        (
            to_args(["--language", "2:eng", "--language", "4:rus"]),
            vec!["--pro", "--langs", "2:en,4:ru"],
        ),
        (
            repeat_track_arg("--language", ":eng", "0-7"),
            vec!["--langs", "eng"],
        ),
        (
            repeat_track_arg("--language", ":rus", "0-7"),
            vec!["--langs", "rus"],
        ),
        (
            append_str_vecs([
                repeat_track_arg("--language", ":eng", "0-2"),
                repeat_track_arg("--language", ":rus", "3-7"),
            ]),
            vec!["--langs", "0-2:en,und:rus"],
        ),
    ];

    compare_arg_cases!(cases, variants_of_args, file, MCTrackLangs, MITracksInfo);
}

#[test]
fn test_x8_audio_to_mkvmerge_args() {
    build_test_x8_to_mkvmerge_args("audio_x8.mka");
}

#[test]
fn test_x8_sub_to_mkvmerge_args() {
    build_test_x8_to_mkvmerge_args("sub_x8.mks");
}

#[test]
fn test_x8_video_to_mkvmerge_args() {
    build_test_x8_to_mkvmerge_args("video_x8.mkv");
}
