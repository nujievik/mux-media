use crate::{common::*, tracks::id, *};
use mux_media::{markers::*, *};

#[test]
fn test_is_default() {
    assert!(TrackNames::default().is_default());
    assert!(from_cfg::<MCTrackNames>(vec![]).is_default());
    assert!(!from_cfg::<MCTrackNames>(vec!["--names", "name"]).is_default());
    assert!(!from_cfg::<MCTrackNames>(vec!["--names", "0:a,1:b"]).is_default());
    assert!(!from_cfg::<MCTrackNames>(vec!["--names", "0-8:a"]).is_default());
}

test_from_str!(
    TrackNames,
    test_from_str,
    ["name", "123", "0-8:name", "eng", "1:a,2:b,3:c", "ru:a,3:b"],
    ["1:,2,3", "1:name,bad_lang:name"]
);

fn from_str(s: &str) -> TrackNames {
    s.parse::<TrackNames>().unwrap()
}

#[test]
fn test_get() {
    let cases = [
        ("name", vec!["0", "8", "eng", "rus", MAX_U64_STR]),
        ("0:a,1:b,8:c", vec!["0", "1", "8"]),
        ("eng:a,rus:b,jpn:c", vec!["eng", "rus", "jpn"]),
        ("0-8:a", vec!["0", "1", "8"]),
        ("0-8:a,eng:b", vec!["0", "eng"]),
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
        ("0:a,1:b,8:c", vec!["2", "7", "9", MAX_U64_STR, "eng"]),
        ("eng:a,rus:b,jpn:c", vec!["und", "fr", "0", "8"]),
        ("0-8:a", vec!["9", "eng", MAX_U64_STR]),
        ("0-8:a,eng:b", vec!["9", "rus", "jpn"]),
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

fn_variants_of_args!("--names" => vec!["--track-names"]);

fn build_test_x1_to_mkvmerge_args(file: &str) {
    let cases = [
        (vec![], vec!["--pro"]),
        (vec![], vec!["--pro", "--names", "eng:a"]),
        (vec![], vec!["--pro", "--names", "2:a"]),
        (vec!["--track-name", "0:a"], vec!["--pro", "--names", "0:a"]),
        (vec!["--track-name", "0:a"], vec!["--names", "a"]),
        (vec!["--track-name", "0:a"], vec!["--names", "0:a"]),
        (vec!["--track-name", "0:a"], vec!["--names", "und:a"]),
        (vec!["--track-name", "0:a"], vec!["--names", "0-8:a"]),
        (vec!["--track-name", "0:a"], vec!["--names", "0:a,1:b,2:c"]),
        (vec!["--track-name", "0:bc"], vec!["--names", "0:bc"]),
    ];

    compare_arg_cases!(cases, variants_of_args, file, MCTrackNames, MITracksInfo);
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
        (vec![], vec!["--pro", "--names", "eng:a"]),
        (
            to_args(["--track-name", "0:a"]),
            vec!["--pro", "--names", "0:a"],
        ),
        (
            to_args(["--track-name", "2:a", "--track-name", "4:b"]),
            vec!["--pro", "--names", "2:a,4:b"],
        ),
        (
            repeat_track_arg("--track-name", ":a", "0-7"),
            vec!["--names", "a"],
        ),
        (
            repeat_track_arg("--track-name", ":bc", "0-7"),
            vec!["--names", "bc"],
        ),
        (
            append_str_vecs([
                repeat_track_arg("--track-name", ":a", "0-2"),
                repeat_track_arg("--track-name", ":b", "3-7"),
            ]),
            vec!["--names", "0-2:a,und:b"],
        ),
    ];

    compare_arg_cases!(cases, variants_of_args, file, MCTrackNames, MITracksInfo);
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

build_test_to_json_args!(
    test_names_to_json_args, track_names, "track_names", @diff_in_out;
    vec![], vec![],
    vec!["--names", "a"], vec!["--names", "a"],
    vec!["--names", "bc"], vec!["--names", "bc"],
    vec!["--names", "0:a,1:bc"], vec!["--names", "0:a,1:bc"],
    vec!["--names", "0:a,1:bc"], vec!["--names", "1:bc,0:a"],
    vec!["--names", "0:bc,1-5:de,eng:a"], vec!["--names", "eng:a,0:bc,1-5:de"],
);
