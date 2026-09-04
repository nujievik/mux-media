mod common;
mod macros;

#[path = "config/auto_flags.rs"]
mod auto_flags;
#[path = "config/chapters.rs"]
mod chapters;
#[path = "config/dispositions.rs"]
mod dispositions;
#[path = "config/input.rs"]
mod input;
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

mod range {
    use mux_media::RangeUsize;

    pub fn new(s: &str) -> RangeUsize {
        s.parse::<RangeUsize>()
            .expect(&format!("Fail range from '{}'", s))
    }
}

use clap::{error::ErrorKind, *};
use common::*;
use is_default::IsDefault;
use mux_media::{config::*, *};
use std::{fs, sync::LazyLock};

static EMPTY_ARGS: LazyLock<Config> = LazyLock::new(|| cfg::<_, &str>([]));

#[test]
fn parse_empty_args_input() {
    let i = &EMPTY_ARGS.input;
    assert_eq!(i.dir(), &fs::canonicalize(".").unwrap());
    assert!(i.range.is_none());
    assert!(i.skip.is_none());
    assert_eq!(i.depth, 16);
    assert!(!i.solo);
}

#[test]
fn parse_empty_args_output() {
    let o = &EMPTY_ARGS.output;
    let dir = fs::canonicalize(".").unwrap().join("muxed");
    assert_eq!(o.dir(), &dir);
    assert_eq!(o.temp_dir(), dir.join(".temp-mux-media"));
}

#[test]
fn parse_empty_args() {
    let e = &EMPTY_ARGS;
    assert_eq!(e.log_level, Default::default());
    assert!(!e.exit_on_err);
    assert!(!e.save_config);
    assert_eq!(1, e.jobs);
    assert_eq!(&e.auto_flags, &Default::default());
    assert_eq!(&e.streams, &Default::default());
    assert_eq!(&e.defaults, &Default::default());
    assert_eq!(&e.forceds, &Default::default());
    assert_eq!(&e.names, &Default::default());
    assert_eq!(&e.langs, &Default::default());
    assert_eq!(&e.retiming, &Default::default());
    assert_eq!(&e.targets, &Default::default());
    assert!(e.is_output_constructed_from_input);
}

fn assert_eq_wo_locale(mut left: Config, right: &Config) {
    left.locale = right.locale;
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
        let mut exp = (*EMPTY_ARGS).clone();
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
    assert!(c.is_output_constructed_from_input);

    let c = cfg([p("-o"), &o]);
    assert_eq!(c.output.dir(), &o);
    assert!(!c.is_output_constructed_from_input);

    test_parse!(
        ["-r", "1-1"],
        input.range,
        Some(RangeUsize::try_from((1, 1)).unwrap())
    );

    let x_globset = Some("x".parse::<GlobSetPattern>().unwrap());
    test_parse!(["--skip", "x"], input.skip, x_globset);

    test_parse!(["--depth", "1"], input.depth, 1);
    test_parse!(["--solo"], input.solo, true);
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
fn parse_no_streams() {
    let xs = ConfigStreams {
        no_flag: true,
        ..Default::default()
    };
    test_parse!(["--no-streams"], streams, xs.clone());

    [
        ("audio", "-A"),
        ("subs", "-S"),
        ("video", "-D"),
        ("fonts", "-F"),
        ("attachs", "-M"),
    ]
    .iter()
    .for_each(|(trg, arg)| {
        let trg = Target::Stream(trg.parse::<StreamType>().unwrap());
        let mut val = ConfigTarget::default();
        val.streams = Some(xs.clone());

        let mut exp = (*EMPTY_ARGS).clone();
        exp.targets.get_or_insert_default().insert(trg, val);
        assert_eq_wo_locale(cfg([arg]), &exp);
    })
}

#[test]
fn parse_streams() {
    let xs = ConfigStreams {
        idxs: Some([0].into()),
        ..Default::default()
    };
    test_parse!(["--streams", "0"], streams, xs.clone());

    [
        ("audio", "-a0"),
        ("subs", "-s0"),
        ("video", "-d0"),
        ("fonts", "-f0"),
        ("attachs", "-m0"),
    ]
    .iter()
    .for_each(|(trg, arg)| {
        let trg = Target::Stream(trg.parse::<StreamType>().unwrap());
        let mut val = ConfigTarget::default();
        val.streams = Some(xs.clone());

        let mut exp = (*EMPTY_ARGS).clone();
        exp.targets.get_or_insert_default().insert(trg, val);
        assert_eq_wo_locale(cfg([arg]), &exp);
    })
}

#[test]
fn parse_chapters() {
    test_parse!(["-C"], chapters.no_flag, true);
}

#[test]
fn parse_dispositions() {
    let xs = ConfigDispositions {
        single_val: Some(true),
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
fn parse_names() {
    let xs = ConfigMetadata {
        single_val: Some(String::from("x")),
        ..Default::default()
    };
    test_parse!(["--names", "x"], names, ConfigNameMetadata(xs));
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
        vec!["-v", "--verbose"],
        vec!["-vv", "-vvv", "-vvvvvvv"],
        vec!["-q", "--quiet"],
        vec!["-e", "--exit-on-err", "--exit-on-error"],
        vec!["-A", "--no-audio"],
        vec!["-S", "--no-subs"],
        vec!["-D", "--no-video"],
        vec!["-C", "--no-chapters"],
        vec!["-F", "--no-fonts"],
        vec!["-M", "--no-attachs"],
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
        (vec!["-m", "--attachs"], "1"),
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
    let cfg = cfg([
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
    ]);

    assert!(cfg.exit_on_err);
    assert!(cfg.target(MarkConfigDefaults, "video").single_val.unwrap());
    assert!(cfg.target(MarkConfigDefaults, "audio").single_val.unwrap());
    assert_eq!(cfg.jobs, 8);
    assert!(cfg.target(MarkConfigDefaults, "sub").single_val.unwrap());

    assert!(cfg.defaults.single_val.is_none());
}
