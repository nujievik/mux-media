use super::id;
use crate::common::{MAX_U64_STR, from_cfg};
use crate::*;
use mux_media::*;

#[test]
fn test_cli_args() {
    test_cli_args!(TrackNames; Names => "names");
}

#[test]
fn test_is_default() {
    assert!(TrackNames::default().is_default());
    assert!(from_cfg::<MCTrackNames>(vec![]).is_default());
    assert!(!from_cfg::<MCTrackNames>(vec!["--names", "name"]).is_default());
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

/*
fn build_test_to_mkvmerge_args(file: &str) {
    let cases = [
        ("name", vec!["0", "8", "eng", "rus", MAX_U64_STR]),
        ("0:a,1:b,8:c", vec!["0", "1", "8"]),
        ("eng:a,rus:b,jpn:c", vec!["eng", "rus", "jpn"]),
        ("0-8:a", vec!["0", "1", "8"]),
        ("0-8:a,eng:b", vec!["0", "eng"]),
    ];
}
*/
