mod common;

#[path = "config/auto_flags.rs"]
mod auto_flags;
#[path = "config/chapters.rs"]
mod chapters;
#[path = "config/dispositions.rs"]
mod dispositions;
#[path = "config/input.rs"]
mod input;
#[path = "config/locales.rs"]
mod locales;
#[path = "config/log_level.rs"]
mod log_level;
#[path = "config/metadata.rs"]
mod metadata;
#[path = "config/output.rs"]
mod output;
#[path = "config/retiming.rs"]
mod retiming;
#[path = "config/streams.rs"]
mod streams;
#[path = "config/streams_order.rs"]
mod streams_order;
#[path = "config/subs_encodings.rs"]
mod subs_encodings;
#[path = "config/ts_offsets.rs"]
mod ts_offsets;

mod range {
    use mux_media::RangeUsizeInclusive;

    pub fn new(s: &str) -> RangeUsizeInclusive {
        s.parse::<RangeUsizeInclusive>()
            .expect(&format!("Fail range from '{}'", s))
    }
}

use clap::{error::ErrorKind, *};
use common::*;
use is_default::IsDefault;
use mux_media::{config::*, *};
use std::fs;

#[test]
fn parse_empty_args_input() {
    let i = &CONFIG.input;
    assert_eq!(i.dir(), &fs::canonicalize(".").unwrap());
    assert!(i.range.is_none());
    assert!(i.skip.is_none());
    assert_eq!(i.depth, 16);
    assert!(!i.orphans_too);
}

#[test]
fn parse_empty_args_output() {
    let o = &CONFIG.output;
    let dir = fs::canonicalize(".").unwrap().join("muxed");
    assert_eq!(o.dir(), &dir);
    assert_eq!(o.temp_dir(), dir.join(".temp-mux-media"));
}

#[test]
fn parse_empty_args() {
    let e = &CONFIG;
    assert_eq!(e.log_level, Default::default());
    assert!(!e.input.overwrite);
    assert_eq!(1, e.jobs);
    assert!(!e.exit_on_err);
    assert!(!e.save_config);

    assert_eq!(&e.auto_flags, &Default::default());
    assert!(!e.clean_flags);
    assert!(!e.clean_meta);

    assert_eq!(&e.streams, &Default::default());
    assert_eq!(&e.defaults, &Default::default());
    assert_eq!(&e.forceds, &Default::default());
    assert_eq!(&e.titles, &Default::default());
    assert_eq!(&e.langs, &Default::default());
    assert_eq!(&e.subs_encodings, &Default::default());
    assert_eq!(&e.ts_offsets, &Default::default());
    assert_eq!(&e.retiming, &Default::default());

    assert_eq!(&e.per_stream_type, &Default::default());
    assert!(e.by_path.is_empty());
}

fn assert_eq_wo_locale(mut left: Config, right: &Config) {
    left.locales = right.locales.clone();
    assert_eq!(&left, right);
}

fn assert_ok_exit(args: &[&str]) {
    let err = Config::try_parse_from(args).unwrap_err();
    assert_eq!(err.exit_code(), 0);
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);

    let err = MuxError::from(err);
    assert_eq!(err.code(), 0);
}

#[test]
fn test_ok_exit() {
    ["-h", "-V", "--list-targets", "--list-langs"]
        .iter()
        .for_each(|arg| {
            assert_ok_exit(&[arg]);
        })
}

macro_rules! test_parse {
    ($args:expr, $( $field:ident $( .$sub_field:ident )? , $exp:expr ),* ) => {{
        let mut exp = (*CONFIG).clone();
        $( exp.$field $( .$sub_field )? = $exp; )*

        assert_eq_wo_locale(cfg($args), &exp);
    }};
}

#[test]
fn parse_input_output() {
    let i = data("input/1/");
    let o = i.join("muxed");

    let c = cfg([p("-i"), &i]);
    assert_eq!(c.input.dir(), &i);
    assert_eq!(c.output.dir(), &o);

    let c = cfg([p("-o"), &o]);
    assert_eq!(c.output.dir(), &o);

    test_parse!(["--overwrite"], input.overwrite, true);

    test_parse!(
        ["-r", "1-1"],
        input.range,
        Some(RangeUsizeInclusive::try_new(1, 1).unwrap())
    );

    let x_globset = Some("x".parse::<GlobSetPattern>().unwrap());
    test_parse!(["--skip", "x"], input.skip, x_globset);

    test_parse!(["--depth", "1"], input.depth, 1);
    test_parse!(["--orphans-too"], input.orphans_too, true);
}

#[test]
fn parse_global() {
    use log::LevelFilter;
    test_parse!(["-v"], log_level, ConfigLogLevel(LevelFilter::Debug));
    test_parse!(["-vv"], log_level, ConfigLogLevel(LevelFilter::Trace));
    test_parse!(["-q"], log_level, ConfigLogLevel(LevelFilter::Error));
    test_parse!(["-e"], exit_on_err, true);
    test_parse!(["--save-config"], save_config, true);
    test_parse!(["--jobs", "8"], jobs, 8);
}

#[test]
fn parse_clean_flags() {
    test_parse!(["--clean-flags"], clean_flags, true);
}

#[test]
fn parse_clean_meta() {
    test_parse!(["--clean-meta"], clean_meta, true);
}

#[test]
fn parse_chapters() {
    test_parse!(["-C"], chapters.no_flag, true);
}

#[test]
fn parse_dispositions() {
    let xs = ConfigDispositions {
        single_val: Some(Bool(true)),
        ..Default::default()
    };
    test_parse!(["--defaults", "true"], defaults, xs.clone());
    test_parse!(["--forceds", "true"], forceds, xs);

    let xs = ConfigDispositions {
        max_in_auto: Some(1),
        ..Default::default()
    };
    test_parse!(["--max-defaults", "1"], defaults, xs.clone());
    test_parse!(["--max-forceds", "1"], forceds, xs);
}

#[test]
fn parse_titles() {
    let xs = ConfigMetadata {
        single_val: Some(String::from("x")),
        ..Default::default()
    };
    test_parse!(["--titles", "x"], titles, ConfigTitleMetadata(xs));
}

#[test]
fn parse_langs() {
    let xs = ConfigMetadata {
        single_val: Some(lang!(Eng)),
        ..Default::default()
    };
    test_parse!(["--langs", "eng"], langs, ConfigLangMetadata(xs));
}

#[test]
fn parse_retiming() {
    let mut parts = ConfigRetimingParts::default();
    parts.pattern = Some("x".parse::<GlobSetPattern>().unwrap());
    test_parse!(["--parts", "x"], retiming.parts, parts.clone());

    parts.inverse = true;
    test_parse!(["--parts", "!x"], retiming.parts, parts);

    test_parse!(["--no-linked"], retiming.no_linked, true);
}

#[test]
fn test_aliases_of_args() {
    [
        vec!["-w", "--overwrite"],
        vec!["-v", "--verbose"],
        vec!["-vv", "-vvv", "-vvvvvvv"],
        vec!["-q", "--quiet"],
        vec!["-e", "--exit-on-err", "--exit-on-error"],
        vec!["-A", "--no-audio"],
        vec!["-S", "--no-subs"],
        vec!["-D", "--no-video"],
        vec!["-C", "--no-chapters"],
        vec!["-F", "--no-fonts"],
        vec!["-P", "--no-pics"],
        vec!["--clean-meta", "--clean-metadata"],
    ]
    .iter()
    .for_each(|args| {
        let first = cfg([args[0]]);
        for arg in &args[1..] {
            assert_eq_wo_locale(cfg([arg]), &first)
        }
    });

    [
        (vec!["-i", "--input"], data("").to_str().unwrap()),
        (vec!["-o", "--output"], data("").to_str().unwrap()),
        (vec!["-r", "--range"], "1-1"),
        (vec!["-j", "--jobs"], "8"),
        (vec!["-a", "--audio"], "1"),
        (vec!["-s", "--subs"], "1"),
        (vec!["-d", "--video"], "1"),
        (vec!["-f", "--fonts"], "1"),
        (vec!["-p", "--pics"], "1"),
        (vec!["-c", "--chapters"], "0:0:0.0-0:5:0.0"),
    ]
    .iter()
    .for_each(|(args, val)| {
        let first = cfg([args[0], val]);
        for arg in &args[1..] {
            assert_eq_wo_locale(cfg([arg, val]), &first)
        }
    });

    [["on", "1", "true"], ["off", "0", "false"]]
        .iter()
        .for_each(|args| {
            let first = cfg(["--defaults", args[0]]);
            for arg in &args[1..] {
                assert_eq_wo_locale(cfg(["--defaults", arg]), &first)
            }
        });
}

#[test]
fn test_target_switching() {
    let args = &[
        "--exit-on-err",
        "--target",
        "video",
        "--defaults",
        "true",
        "--target",
        "audio",
        "--defaults",
        "true",
        "--target",
        "global",
        "--jobs",
        "8",
        "--target",
        "subs",
        "--defaults",
        "true",
    ];

    let xs = ConfigDispositions {
        single_val: Some(Bool(true)),
        ..Default::default()
    };

    let mut exp = CONFIG.clone();
    exp.exit_on_err = true;
    exp.per_stream_type[StreamType::Video].defaults = Some(xs.clone());
    exp.per_stream_type[StreamType::Audio].defaults = Some(xs.clone());

    exp.jobs = 8;
    exp.per_stream_type[StreamType::Subtitles].defaults = Some(xs);

    assert_eq_wo_locale(exp, &cfg(args));
}

const TEST_TARGETS: &[&[&str]] = &[
    &["audio_x1.mka", "audio_x8.mka", "ogg.ogg"],
    &["srt.srt", "sub_x1.mks", "sub_x8.mks", "cp1251.srt"],
    &["video_x1.mkv", "video_x8.mkv"],
];

fn trg_with_no_chapters() -> ConfigTarget {
    let mut chapters = ConfigChapters::default();
    chapters.no_flag = true;

    let mut trg = ConfigTarget::default();
    trg.chapters = Some(chapters);
    trg
}

#[test]
fn parse_file_targets() {
    let trg = trg_with_no_chapters();

    TEST_TARGETS.iter().for_each(|files| {
        files.iter().for_each(|f| {
            let f = data(f);
            let mut exp = CONFIG.clone();
            exp.by_path.insert(f.clone().into(), trg.clone());

            assert_eq_wo_locale(exp, &cfg([p("--target"), &f, p("-C")]));
        })
    })
}

#[test]
fn parse_parent_target() {
    let parent = data("");
    let mut exp = CONFIG.clone();
    exp.by_path
        .insert(parent.clone().into(), trg_with_no_chapters());

    assert_eq_wo_locale(exp, &cfg([p("--target"), &parent, p("-C")]));
}

#[test]
fn parse_file_and_dir_targets() {
    let parent = data(""); //common for all files
    let trg = trg_with_no_chapters();

    TEST_TARGETS.iter().for_each(|files| {
        files.iter().for_each(|f| {
            let f = data(f);
            let args = [p("--target"), &f, p("-C"), p("--target"), &parent, p("-C")];

            let mut exp = CONFIG.clone();
            exp.by_path.insert(f.clone().into(), trg.clone());
            exp.by_path
                .insert(parent.clone().into(), trg_with_no_chapters());

            assert_eq_wo_locale(exp, &cfg(args));
        })
    })
}

build_test_to_args!(to_args_exit_on_err, "config_exit_on_err"; vec!["--exit-on-err"]);
build_test_to_args!(to_args_clean_flags, "config_clean_flags"; vec!["--clean-flags"]);
build_test_to_args!(to_args_clean_meta, "config_clean_meta"; vec!["--clean-meta"]);
