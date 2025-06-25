use crate::common::*;
use crate::*;
use mux_media::*;

#[test]
fn test_cli_args() {
    test_cli_args!(DefaultTFlags; Defaults => "defaults", LimDefaults => "lim-defaults");
    test_cli_args!(ForcedTFlags; Forceds => "forceds", LimForceds => "lim-forceds");
    test_cli_args!(EnabledTFlags; Enableds => "enableds", LimEnableds => "lim-enableds");
}

#[test]
fn test_mkvmerge_args() {
    assert_eq!("--default-track-flag", DefaultTFlags::MKVMERGE_ARG);
    assert_eq!("--forced-display-flag", ForcedTFlags::MKVMERGE_ARG);
    assert_eq!("--track-enabled-flag", EnabledTFlags::MKVMERGE_ARG);

    assert_eq!("--default-track-flag", TFlagType::Default.to_mkvmerge_arg());
    assert_eq!("--forced-display-flag", TFlagType::Forced.to_mkvmerge_arg());
    assert_eq!("--track-enabled-flag", TFlagType::Enabled.to_mkvmerge_arg());
}

#[test]
fn test_is_default() {
    assert!(DefaultTFlags::default().is_default());
    assert!(ForcedTFlags::default().is_default());
    assert!(EnabledTFlags::default().is_default());

    assert!(from_cfg::<MCDefaultTFlags>(vec![]).is_default());
    assert!(from_cfg::<MCForcedTFlags>(vec![]).is_default());
    assert!(from_cfg::<MCEnabledTFlags>(vec![]).is_default());

    assert!(!from_cfg::<MCDefaultTFlags>(vec!["--lim-defaults", "1"]).is_default());
    assert!(!from_cfg::<MCForcedTFlags>(vec!["--lim-forceds", "0"]).is_default());
    assert!(!from_cfg::<MCEnabledTFlags>(vec!["--lim-enableds", MAX_U64_STR]).is_default());
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
    for ft in TFlagType::iter() {
        for tt in TrackType::iter() {
            assert_eq!(0, counts.get(ft, tt));
        }
    }
}

#[test]
fn test_counts_add() {
    let mut counts = TFlagsCounts::default();
    for ft in TFlagType::iter() {
        let mut current = 0;

        let mut add = |x| {
            (0..x).into_iter().for_each(|_| {
                for tt in TrackType::iter() {
                    counts.add(ft, tt);
                }
            });

            current += x;

            for tt in TrackType::iter() {
                assert_eq!(current, counts.get(ft, tt));
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
    let ftypes = [TFlagType::Default, TFlagType::Forced, TFlagType::Enabled];
    for ft in TFlagType::iter() {
        assert!(ftypes.contains(&ft));
    }
}

#[test]
fn test_flag_type_to_mkvmerge_arg() {
    assert_eq!("--default-track-flag", TFlagType::Default.to_mkvmerge_arg());
    assert_eq!("--forced-display-flag", TFlagType::Forced.to_mkvmerge_arg());
    assert_eq!("--track-enabled-flag", TFlagType::Enabled.to_mkvmerge_arg());
}

fn_variants_of_args!(
    "defaults" => vec!["--default-track-flags", "--default-tracks"],
    "forceds" => vec!["--forced-display-flags", "--forced-tracks"],
    "enableds" => vec!["track-enabled-flags"],
    "on" => vec!["1", "true"],
    "off" => vec!["0", "false"],
);

macro_rules! test_to_mvkmerge_args_fallback {
    ($fn_ident:ident, $mkvmerge_arg:expr, $arg:expr, $mc_field:ident) => {
        #[test]
        fn $fn_ident() {
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
    };
}

test_to_mvkmerge_args_fallback!(
    test_defaults_to_mvkmerge_args_fallback,
    "--default-track-flag",
    "--defaults",
    MCDefaultTFlags
);

test_to_mvkmerge_args_fallback!(
    test_forceds_to_mvkmerge_args_fallback,
    "--forced-display-flag",
    "--forceds",
    MCForcedTFlags
);

test_to_mvkmerge_args_fallback!(
    test_enableds_to_mvkmerge_args_fallback,
    "--track-enabled-flag",
    "--enableds",
    MCEnabledTFlags
);

/*
#[test]
fn test_to_mvkmerge_args_fallback() {
    let cases = [
        (vec![], vec![]),
        (vec![], vec!["--pro"]),
        (vec!["--default-track-flag", "0:0"], vec!["--defaults", "off"]),
    ];

    compare_arg_cases!(
        cases,
        variants_of_args,
        "srt.srt",
        MCDefaultTFlags,
        MITargets,
        MITracksInfo
    );
}
*/
