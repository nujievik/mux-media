use crate::{compare_arg_cases, fn_variants_of_args, test_from_str};
use mux_media::*;

const FROM_STR_CASES: [&'static str; 11] = [
    "1",
    "0",
    "on",
    "off",
    "true",
    "false",
    "0,1,6",
    "3-4",
    "rus",
    "eng",
    "0:1,4:0,3:true,7:off,eng:1",
];
const FROM_STR_ERR_CASES: [&'static str; 3] = ["x", "missing", "0--1,6"];

test_from_str!(
    DefaultTFlags,
    test_defaults_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);
test_from_str!(
    ForcedTFlags,
    test_forceds_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);
test_from_str!(
    EnabledTFlags,
    test_enableds_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);

#[test]
fn test_counts_default() {
    let counts = TFlagsCounts::default();
    for tt in TrackType::iter() {
        assert_eq!(0, counts.get_default(tt));
        assert_eq!(0, counts.get_forced(tt));
        assert_eq!(0, counts.get_enabled(tt));
    }
}

#[test]
fn test_counts_add() {
    let mut counts = TFlagsCounts::default();
    for tt in TrackType::iter() {
        let mut current = 0;

        let mut add = |x| {
            (0..x).into_iter().for_each(|_| {
                counts.add_default(tt);
                counts.add_forced(tt);
                counts.add_enabled(tt);
            });
            current += x;
            assert_eq!(current, counts.get_default(tt));
            assert_eq!(current, counts.get_forced(tt));
            assert_eq!(current, counts.get_enabled(tt));
        };

        add(1);
        add(3);
        add(11);
        add(2);
    }
}

fn_variants_of_args!(
    "defaults" => vec!["--default-track-flags", "--default-tracks"],
    "forceds" => vec!["--forced-display-flags", "--forced-tracks"],
    "enableds" => vec!["track-enabled-flags"],
);

#[test]
fn test_to_mvkmerge_args() {
    let cases = [(vec!["--default-track-flag", "0:0"], vec![])];
    compare_arg_cases!(
        cases,
        variants_of_args,
        "srt.srt",
        MCDefaultTFlags,
        MITargets,
        MITracksInfo
    );
}
