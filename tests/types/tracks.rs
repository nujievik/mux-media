#[path = "tracks/flags.rs"]
mod flags;
#[path = "tracks/langs.rs"]
mod langs;
#[path = "tracks/names.rs"]
mod names;

use crate::common::*;
use crate::*;
use mux_media::*;

#[test]
fn test_mkvmerge_args() {
    assert_eq!("-a", AudioTracks::MKVMERGE_ARG);
    assert_eq!("-A", AudioTracks::MKVMERGE_NO_ARG);
    assert_eq!("-s", SubTracks::MKVMERGE_ARG);
    assert_eq!("-S", SubTracks::MKVMERGE_NO_ARG);
    assert_eq!("-d", VideoTracks::MKVMERGE_ARG);
    assert_eq!("-D", VideoTracks::MKVMERGE_NO_ARG);
    assert_eq!("-b", ButtonTracks::MKVMERGE_ARG);
    assert_eq!("-B", ButtonTracks::MKVMERGE_NO_ARG);
}

#[test]
fn test_is_default() {
    assert!(AudioTracks::default().is_default());
    assert!(SubTracks::default().is_default());
    assert!(VideoTracks::default().is_default());
    assert!(ButtonTracks::default().is_default());

    assert!(from_cfg::<MCAudioTracks>(vec![]).is_default());
    assert!(from_cfg::<MCSubTracks>(vec![]).is_default());
    assert!(from_cfg::<MCVideoTracks>(vec![]).is_default());
    assert!(from_cfg::<MCButtonTracks>(vec![]).is_default());

    assert!(!from_cfg::<MCAudioTracks>(vec!["-A"]).is_default());
    assert!(!from_cfg::<MCAudioTracks>(vec!["-a", "0"]).is_default());
    assert!(!from_cfg::<MCAudioTracks>(vec!["-a", "0-8"]).is_default());
    assert!(!from_cfg::<MCSubTracks>(vec!["-S"]).is_default());
    assert!(!from_cfg::<MCSubTracks>(vec!["-s", "0"]).is_default());
    assert!(!from_cfg::<MCSubTracks>(vec!["-s", "0-8"]).is_default());
    assert!(!from_cfg::<MCVideoTracks>(vec!["-D"]).is_default());
    assert!(!from_cfg::<MCVideoTracks>(vec!["-d", "0"]).is_default());
    assert!(!from_cfg::<MCVideoTracks>(vec!["-d", "0-8"]).is_default());
    assert!(!from_cfg::<MCButtonTracks>(vec!["-B"]).is_default());
    assert!(!from_cfg::<MCButtonTracks>(vec!["-b", "0"]).is_default());
    assert!(!from_cfg::<MCButtonTracks>(vec!["-b", "0-8"]).is_default());
}

fn id(s: &str) -> TrackID {
    s.parse::<TrackID>().unwrap()
}

test_from_str!(
    TrackID, test_id_from_str,
    [
        (TrackID::Num(0), "0"),
        (TrackID::Num(8), "8"),
        (TrackID::Lang(LangCode::Eng), "eng"),
        (TrackID::Lang(LangCode::Rus), "rus"),
        (TrackID::Range(range::new("0-0")), "0-0"),
        (TrackID::Range(range::new("1-8")), "1-8"),
        (TrackID::Range(range::new("0-")), "0-"),
    ],
    ["missing", "x", "1--2", "1;2"],
    @ok_compare
);

#[test]
fn test_id_is_range() {
    let cases = ["0-", "1-1", "-8"];

    for s in cases {
        assert!(id(s).is_range());
    }

    let bad_cases = ["0", "8", "eng", "rus"];

    for s in bad_cases {
        assert!(!id(s).is_range());
    }
}

#[test]
fn test_id_contains() {
    let cases = [
        ("0", "0"),
        ("8", "8"),
        ("eng", "eng"),
        ("rus", "rus"),
        ("0-", "0"),
        ("0-", "8"),
        ("0-", "65535"),
        ("0-", "0-"),
        ("0-", "1-8"),
        ("1-8", "1"),
        ("1-8", "8"),
    ];

    for (s_exp, s) in cases {
        assert!(id(s_exp).contains(&id(s)));
    }

    let bad_cases = [
        ("0", "1"),
        ("8", "0"),
        ("eng", "rus"),
        ("rus", "eng"),
        ("1-8", "1-"),
        ("1-8", "0"),
        ("1-8", "9"),
    ];

    for (s_exp, s) in bad_cases {
        assert!(!id(s_exp).contains(&id(s)));
    }
}

const FROM_STR_CASES: [&'static str; 7] = ["0", "8", "eng", "rus", "1-8", "0,8,eng,1-8", "!0"];
const FROM_STR_ERR_CASES: [&'static str; 0] = [];

test_from_str!(
    AudioTracks,
    test_audio_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);
test_from_str!(
    SubTracks,
    test_sub_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);
test_from_str!(
    VideoTracks,
    test_video_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);
test_from_str!(
    ButtonTracks,
    test_button_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);

macro_rules! test_save_track {
    ($type:ident, $test_fn:ident, $field:ident, $no_arg:expr) => {
        #[test]
        fn $test_fn() {
            let no_tracks = from_cfg::<$field>(vec![$no_arg]);

            let save = |left: bool, tracks: &$type, s_ids: &[&str]| {
                s_ids.into_iter().for_each(|s_id| {
                    let tid = TrackID::Num(s_id.parse::<u64>().unwrap_or(99));
                    let l_tid = TrackID::Lang(s_id.parse::<LangCode>().unwrap_or(LangCode::Und));

                    assert_eq!(left, tracks.save_track(&tid, &l_tid), "err on '{}'", s_id);
                })
            };

            let cases = [
                (vec!["0"], "0"),
                (vec!["8"], "8"),
                (vec!["eng"], "eng"),
                (vec!["rus"], "rus"),
                (vec!["1", "2", "3", "4", "5", "6", "7", "8"], "1-8"),
                (vec!["0", "12", "eng", "4"], "0,12,eng,1-8"),
                (vec!["1", "8", MAX_U64_STR], "!0"),
                (vec!["eng", "rus", "0", "4"], "!jpn"),
                (vec!["eng", "rus", "0", "4"], "!1-3"),
            ];

            cases.iter().for_each(|(s_ids, s)| {
                let tracks = s.parse::<$type>().unwrap();
                save(true, &tracks, s_ids);
                save(false, &no_tracks, s_ids);
            });

            let bad_cases = [
                (vec!["1", "8", "eng"], "0"),
                (vec!["0", MAX_U64_STR, "rus"], "8"),
                (vec!["rus", "8", "jpn"], "eng"),
                (vec!["eng", "0", "jpn"], "rus"),
                (vec!["0", "9", "eng", "rus"], "1-8"),
                (vec!["9", "rus", "jpn"], "0,12,eng,1-8"),
                (vec!["jpn"], "!jpn"),
                (vec!["1", "2", "3"], "!1-3"),
            ];

            bad_cases.iter().for_each(|(s_ids, s)| {
                let tracks = s.parse::<$type>().unwrap();
                save(false, &tracks, s_ids);
                save(false, &no_tracks, s_ids);
            });
        }
    };
}

test_save_track!(AudioTracks, test_audio_save_track, MCAudioTracks, "-A");
test_save_track!(SubTracks, test_subs_save_track, MCSubTracks, "-S");
test_save_track!(VideoTracks, test_video_save_track, MCVideoTracks, "-D");
test_save_track!(ButtonTracks, test_buttons_save_track, MCButtonTracks, "-B");

#[test]
fn test_id_to_mkvmerge_arg() {
    let cases = [
        ("0", "0"),
        ("8", "8"),
        (MAX_U64_STR, MAX_U64_STR),
        ("eng", "eng"),
        ("rus", "rus"),
        ("0,1,2,3,4", "-4"),
        ("1,2,3,4", "1-4"),
    ];

    for (exp, s) in cases {
        assert_eq!(exp, id(s).to_mkvmerge_arg());
    }
}

fn_variants_of_args!(
    "-a" => vec!["--audio", "--audio-tracks", "--atracks"],
    "-A" => vec!["--no-audio", "--noaudio"],
    "-s" => vec!["--subs", "--subtitle-tracks", "--subtitles", "--sub-tracks", "--stracks"],
    "-S" => vec!["--no-subs", "--no-subtitles", "--nosubtitles", "--nosubs"],
    "-d" => vec!["--video", "--video-tracks", "--vtracks"],
    "-D" => vec!["--no-video", "--novideo"],
);

macro_rules! test_x8_file_to_mkvmerge_arg {
    ( $( $test_fn:ident, $file:expr, $field:ident, $arg:expr, $no_arg:expr );* ) => {
        $(
        #[test]
        fn $test_fn() {
            let cases = [
                (vec![], vec![]),
                (vec![$no_arg], vec![$no_arg]),
                (vec![$arg, "0"], vec![$arg, "0"]),
                (vec![$arg, "7"], vec![$arg, "7"]),
                (vec![$no_arg], vec![$no_arg]),
                (vec![$no_arg], vec![$arg, "9"]),
                (vec![$no_arg], vec![$arg, "eng"]),
                (vec![$no_arg], vec![$arg, "rus"]),
                (vec![$arg, "0,1,2,3,4,5,6,7"], vec![$arg, "0-"]),
                (vec![$arg, "0,1,2,3,4,5,6,7"], vec![$arg, "!eng"]),
                (vec![$arg, "1,2,3,4,5,6,7"], vec![$arg, "!0"]),
                (vec![$arg, "3,4"], vec![$arg, "3,4"]),
                (vec![$arg, "1,2,3,4,7"], vec![$arg, "!0,5-6"]),
            ];

            compare_arg_cases!(cases, variants_of_args, $file, $field, MITracksInfo);
        }
        )*
    };
}

test_x8_file_to_mkvmerge_arg!(
    test_x8_audio_to_mkvmerge_args, "audio_x8.mka", MCAudioTracks, "-a", "-A";
    test_x8_sub_to_mkvmerge_args, "sub_x8.mks", MCSubTracks, "-s", "-S";
    test_x8_video_to_mkvmerge_args, "video_x8.mkv", MCVideoTracks, "-d", "-D"
);

macro_rules! test_x1_file_to_mkvmerge_arg {
    ( $( $test_fn:ident, $file:expr, $field:ident, $arg:expr, $no_arg:expr );* ) => {
        $(
        #[test]
        fn $test_fn() {
            let cases = [
                (vec![], vec![]),
                (vec![$no_arg], vec![$no_arg]),
                (vec![$arg, "0"], vec![$arg, "0"]),
                (vec![$no_arg], vec![$arg, "7"]),
                (vec![$no_arg], vec![$no_arg]),
                (vec![$no_arg], vec![$arg, "9"]),
                (vec![$no_arg], vec![$arg, "eng"]),
                (vec![$no_arg], vec![$arg, "rus"]),
                (vec![$arg, "0"], vec![$arg, "0-"]),
                (vec![$arg, "0"], vec![$arg, "!eng"]),
                (vec![$no_arg], vec![$arg, "!0"]),
                (vec![$no_arg], vec![$arg, "3,4"]),
                (vec![$no_arg], vec![$arg, "!0,5-6"]),
            ];

            compare_arg_cases!(cases, variants_of_args, $file, $field, MITracksInfo);
        }
        )*
    };
}

test_x1_file_to_mkvmerge_arg!(
    test_x1_audio_to_mkvmerge_args, "audio_x1.mka", MCAudioTracks, "-a", "-A";
    test_x1_sub_to_mkvmerge_args, "sub_x1.mks", MCSubTracks, "-s", "-S";
    test_x1_video_to_mkvmerge_args, "video_x1.mkv", MCVideoTracks, "-d", "-D"
);

macro_rules! build_test_tracks_to_json_args {
    ( $( $fn:ident, $field:ident, $json_dir:expr, $arg:expr, $no_arg:expr );* ) => {
        $(
            build_test_to_json_args!(
                $fn, $field, $json_dir, @diff_in_out;
                vec![], vec![],
                vec![$no_arg], vec![$no_arg],
                vec![$arg, "0"], vec![$arg, "0"],
                vec![$arg, "0,1,2,3"], vec![$arg, "0,1,2,3"],
                vec![$arg, "eng,rus"], vec![$arg, "eng,rus"],
                vec![$arg, "1,eng,rus"], vec![$arg, "1,eng,rus"],
                vec![$arg, "1..=5,eng,rus"], vec![$arg, "1..=5,eng,rus"],
                vec![$arg, "0,1,2,3"], vec![$arg, "3,0,2,1"],
                vec![$arg, "eng,rus"], vec![$arg, "rus,eng"],
                vec![$arg, "1,eng,rus"], vec![$arg, "eng,1,rus"],
                vec![$arg, "1..=5,eng,rus"], vec![$arg, "rus,1..=5,eng"],
                vec![$arg, "1..=5"], vec![$arg, "1-5"],
            );
        )*
    };
}

build_test_tracks_to_json_args!(
    test_audio_to_json_args, MCAudioTracks, "audio_tracks", "--audio", "--no-audio";
    test_subs_to_json_args, MCSubTracks, "sub_tracks", "--subs", "--no-subs";
    test_video_to_json_args, MCVideoTracks, "video_tracks", "--video", "--no-video";
    test_buttons_to_json_args, MCButtonTracks, "button_tracks", "--buttons", "--no-buttons"
);
