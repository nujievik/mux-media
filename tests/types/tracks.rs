#[path = "tracks/flags.rs"]
mod flags;
#[path = "tracks/names.rs"]
mod names;

use crate::common::{MAX_U64_STR, from_cfg};
use crate::{compare_arg_cases, fn_variants_of_args, range, test_cli_args, test_from_str};
use mux_media::*;

#[test]
fn test_cli_args() {
    test_cli_args!(AudioTracks; Audio => "audio", NoAudio => "no-audio");
    test_cli_args!(SubTracks; Subs => "subs", NoSubs => "no-subs");
    test_cli_args!(VideoTracks; Video => "video", NoVideo => "no-video");
    test_cli_args!(ButtonTracks; Buttons => "buttons", NoButtons => "no-buttons");

    test_cli_args!(TrackLangs; Langs => "langs");
}

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
fn test_default_is_default() {
    assert!(AudioTracks::default().is_default());
    assert!(SubTracks::default().is_default());
    assert!(VideoTracks::default().is_default());
    assert!(ButtonTracks::default().is_default());
    assert!(TrackLangs::default().is_default());
}

#[test]
fn test_empty_args_is_default() {
    assert!(from_cfg::<MCAudioTracks>(vec![]).is_default());
    assert!(from_cfg::<MCSubTracks>(vec![]).is_default());
    assert!(from_cfg::<MCVideoTracks>(vec![]).is_default());
    assert!(from_cfg::<MCButtonTracks>(vec![]).is_default());
    assert!(from_cfg::<MCTrackLangs>(vec![]).is_default());
}

#[test]
fn test_any_args_is_not_default() {
    assert!(!from_cfg::<MCAudioTracks>(vec!["-A"]).is_default());
    assert!(!from_cfg::<MCSubTracks>(vec!["-S"]).is_default());
    assert!(!from_cfg::<MCVideoTracks>(vec!["-D"]).is_default());
    assert!(!from_cfg::<MCButtonTracks>(vec!["-B"]).is_default());
    assert!(!from_cfg::<MCTrackLangs>(vec!["--langs", "eng"]).is_default());
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
    ($type:ident, $test_fn:ident) => {
        #[test]
        fn $test_fn() {
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

            for (s_ids, s) in cases {
                let tracks = s.parse::<$type>().unwrap();
                for s_id in s_ids {
                    let tid = TrackID::Num(s_id.parse::<u64>().unwrap_or(99));
                    let l_tid = TrackID::Lang(s_id.parse::<LangCode>().unwrap_or(LangCode::Und));

                    assert!(
                        tracks.save_track(&tid, &l_tid),
                        "Tracks '{}' not save track '{}'",
                        s,
                        s_id,
                    );
                }
            }

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

            for (s_ids, s) in bad_cases {
                let tracks = s.parse::<$type>().unwrap();
                for s_id in s_ids {
                    let tid = TrackID::Num(s_id.parse::<u64>().unwrap_or(99));
                    let l_tid = TrackID::Lang(s_id.parse::<LangCode>().unwrap_or(LangCode::Und));

                    assert!(
                        !tracks.save_track(&tid, &l_tid),
                        "Tracks '{}' save track '{}'",
                        s,
                        s_id
                    );
                }
            }
        }
    };
}

test_save_track!(AudioTracks, test_audio_save_track);
test_save_track!(SubTracks, test_sub_save_track);
test_save_track!(VideoTracks, test_video_save_track);
test_save_track!(ButtonTracks, test_button_save_track);

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
    ( $( $type:ident, $test_fn:ident, $file:expr, $field:ident, $arg:expr, $no_arg:expr );* $(;)?) => {
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
    AudioTracks, test_x8_audio_to_mkvmerge_args, "audio_x8.mka", MCAudioTracks, "-a", "-A";
    SubTracks, test_x8_sub_to_mkvmerge_args, "sub_x8.mks", MCSubTracks, "-s", "-S";
    VideoTracks, test_x8_video_to_mkvmerge_args, "video_x8.mkv", MCVideoTracks, "-d", "-D";
);

macro_rules! test_x1_file_to_mkvmerge_arg {
    ( $( $type:ident, $test_fn:ident, $file:expr, $field:ident, $arg:expr, $no_arg:expr );* $(;)?) => {
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
    AudioTracks, test_x1_audio_to_mkvmerge_args, "audio_x1.mka", MCAudioTracks, "-a", "-A";
    SubTracks, test_x1_sub_to_mkvmerge_args, "sub_x1.mks", MCSubTracks, "-s", "-S";
    VideoTracks, test_x1_video_to_mkvmerge_args, "video_x1.mkv", MCVideoTracks, "-d", "-D";
);
