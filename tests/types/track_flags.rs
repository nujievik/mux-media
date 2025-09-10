use crate::{common::*, *};
use mux_media::{markers::*, *};

#[test]
fn test_is_default() {
    assert!(DefaultTrackFlags::default().is_default());
    assert!(ForcedTrackFlags::default().is_default());
    assert!(EnabledTrackFlags::default().is_default());

    assert!(from_cfg::<MCDefaultTrackFlags>(vec![]).is_default());
    assert!(from_cfg::<MCForcedTrackFlags>(vec![]).is_default());
    assert!(from_cfg::<MCEnabledTrackFlags>(vec![]).is_default());

    assert!(!from_cfg::<MCDefaultTrackFlags>(vec!["--max-defaults", "1"]).is_default());
    assert!(!from_cfg::<MCForcedTrackFlags>(vec!["--max-forceds", "0"]).is_default());
    assert!(!from_cfg::<MCEnabledTrackFlags>(vec!["--max-enableds", MAX_U64_STR]).is_default());
}

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
    DefaultTrackFlags,
    test_defaults_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);
test_from_str!(
    ForcedTrackFlags,
    test_forceds_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);
test_from_str!(
    EnabledTrackFlags,
    test_enableds_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);

#[test]
fn test_counts_default() {
    let counts = TrackFlagsCounts::default();
    for ft in TrackFlagType::iter() {
        for tt in TrackType::iter() {
            assert_eq!(0, counts.val(ft, tt));
        }
    }
}

#[test]
fn test_counts_add() {
    let mut counts = TrackFlagsCounts::default();
    for ft in TrackFlagType::iter() {
        let mut current = 0;

        let mut add = |x| {
            (0..x).into_iter().for_each(|_| {
                for tt in TrackType::iter() {
                    counts.add(ft, tt);
                }
            });

            current += x;

            for tt in TrackType::iter() {
                assert_eq!(current, counts.val(ft, tt));
            }
        };

        add(1);
        add(3);
        add(11);
        add(2);
    }
}

#[test]
fn test_flag_type_iter() {
    let ftypes = [
        TrackFlagType::Default,
        TrackFlagType::Forced,
        TrackFlagType::Enabled,
    ];
    for ft in TrackFlagType::iter() {
        assert!(ftypes.contains(&ft));
    }
}

fn_variants_of_args!(
    "defaults" => vec!["--default-track-flags", "--default-tracks"],
    "forceds" => vec!["--forced-display-flags", "--forced-tracks"],
    "enableds" => vec!["track-enabled-flags"],
    "on" => vec!["1", "true"],
    "off" => vec!["0", "false"],
);

macro_rules! build_test_to_mvkmerge_args_fallback {
    ( $( $fn:ident, $mkvmerge_arg:expr, $arg:expr, $mc_field:ident );* ) => {
        $(
        #[test]
        fn $fn() {
            let cases = [
                (vec![], vec![]),
                (vec![], vec!["--pro"]),
                (repeat_track_arg($mkvmerge_arg, "", "0-7"), vec![$arg, "on"]),
                (
                    repeat_track_arg($mkvmerge_arg, ":0", "0-7"),
                    vec![$arg, "off"],
                ),
                (to_args([$mkvmerge_arg, "1"]), vec![$arg, "1:on"]),
                (to_args([$mkvmerge_arg, "1:0"]), vec![$arg, "1:off"]),
                (
                    append_str_vecs([vec![$mkvmerge_arg, "0:0"], vec![$mkvmerge_arg, "1"]]),
                    vec![$arg, "1:on,0:off"],
                ),
            ];

            compare_arg_cases!(
                cases,
                variants_of_args,
                "sub_x8.mks",
                $mc_field,
                MITargets,
                MITracksInfo
            );
        }
        )*
    };
}

build_test_to_mvkmerge_args_fallback!(
    test_defaults_to_mvkmerge_args_fallback, "--default-track-flag", "--defaults", MCDefaultTrackFlags;
    test_forceds_to_mvkmerge_args_fallback, "--forced-display-flag", "--forceds", MCForcedTrackFlags;
    test_enableds_to_mvkmerge_args_fallback, "--track-enabled-flag", "--enableds", MCEnabledTrackFlags
);

macro_rules! build_test_flags_to_json_args {
    ( $( $fn:ident, $field:ident, $json_dir:expr, $arg:expr, $lim_arg:expr );* ) => {
        $(
            build_test_to_json_args!(
                $fn, $field, $json_dir, @diff_in_out;
                vec![], vec![],
                vec![$lim_arg, "0"], vec![$lim_arg, "0"],
                vec![$lim_arg, "8"], vec![$lim_arg, "8"],
                vec![$lim_arg, MAX_U64_STR], vec![$lim_arg, MAX_U64_STR],
                vec![$arg, "true"], vec![$arg, "true"],
                vec![$arg, "true"], vec![$arg, "1"],
                vec![$arg, "true"], vec![$arg, "on"],
                vec![$arg, "false"], vec![$arg, "false"],
                vec![$arg, "false"], vec![$arg, "0"],
                vec![$arg, "false"], vec![$arg, "off"],
                vec![$arg, "0:true,1:false"], vec![$arg, "0:true,1:false"],
                vec![$arg, "0:true,1:false"], vec![$arg, "1:0,0:1"],
                vec![$arg, "0:false,eng:true"], vec![$arg, "0:false,eng:true"],
                vec![$arg, "0:true,1-8:false"], vec![$arg, "0:true,1-8:false"],
                vec![$arg, "false"], vec![$lim_arg, "8", $arg, "false"],
                vec![$lim_arg, "8", $arg, "0:false,1:true"], vec![$lim_arg, "8", $arg, "0:0,1:1"],
            );
        )*
    };
}

build_test_flags_to_json_args!(
    test_defaults_to_json_args, default_track_flags, "default_track_flags", "--defaults", "--max-defaults";
    test_forceds_to_json_args, forced_track_flags, "forced_track_flags", "--forceds", "--max-forceds";
    test_enableds_to_json_args, enabled_track_flags, "enabled_track_flags", "--enableds", "--max-enableds"
);
