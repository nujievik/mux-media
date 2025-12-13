use crate::common::*;
use clap::{error::ErrorKind, *};
use mux_media::{markers::*, *};
use std::{fs, sync::LazyLock};

static EMPTY_ARGS: LazyLock<Config> = LazyLock::new(|| cfg::<_, &str>([]));

#[test]
fn parse_empty_args_input() {
    let i = &EMPTY_ARGS.input;
    assert_eq!(&i.dir, &fs::canonicalize(".").unwrap());
    assert!(i.range.is_none());
    assert!(i.skip.is_none());
    assert_eq!(i.depth, 16);
    assert!(!i.solo);
    assert!(!i.need_num);
    assert!(!i.out_need_num);
    assert!(i.dirs.values().all(|xs| xs.is_empty()));
}

#[test]
fn parse_empty_args_output() {
    let o = &EMPTY_ARGS.output;
    assert_eq!(&o.dir, &fs::canonicalize(".").unwrap().join("muxed"));
    assert_eq!(&o.temp_dir, p(""));
    assert!(o.name_begin.is_empty());
    assert!(o.name_tail.is_empty());
    assert_eq!("mkv", o.ext);
    assert!(o.created_dirs.is_empty());
}

#[test]
fn parse_empty_args() {
    let e = &EMPTY_ARGS;
    assert_eq!(e.log_level, Default::default());
    assert!(!e.exit_on_err);
    assert!(!e.save_config);
    assert!(!e.reencode);
    assert_eq!(1, e.jobs);
    assert_eq!(&e.auto_flags, &Default::default());
    assert_eq!(&e.streams, &Default::default());
    assert_eq!(&e.defaults, &Default::default());
    assert_eq!(&e.forceds, &Default::default());
    assert_eq!(&e.names, &Default::default());
    assert_eq!(&e.langs, &Default::default());
    assert_eq!(&e.retiming_options, &Default::default());
    assert_eq!(&e.targets, &Default::default());
    assert_eq!(&e.tool_paths, &Default::default());
    assert_eq!(&e.muxer, &Default::default());
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
    assert_eq!(err.code, 0);
    assert_eq!(err.kind, MuxErrorKind::Ok);
}

#[test]
fn test_ok_exit() {
    [
        "-h",
        "-V",
        "--list-targets",
        "--list-containers",
        "--list-langs",
    ]
    .iter()
    .for_each(|arg| {
        assert_ok_exit(&[arg]);
    })
}

#[test]
fn test_ok_exit_ffmpeg_help() {
    assert_ok_exit(&["--ffmpeg", "-h"]);
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
    let i = data("");
    let o = data("muxed/");

    test_parse!(
        [p("-i"), &i],
        input.dir,
        i.clone(),
        output.dir,
        o.clone(),
        is_output_constructed_from_input,
        true
    );
    test_parse!(
        [p("-o"), &o],
        output.dir,
        o.clone(),
        is_output_constructed_from_input,
        false
    );
    test_parse!(
        ["-r", "1-1"],
        input.range,
        Some(RangeUsize::try_from((1, 1)).unwrap()),
        input.need_num,
        true
    );

    let x_globset = Some("x".parse::<GlobSetPattern>().unwrap());

    test_parse!(["--skip", "x"], input.skip, x_globset.clone());
    test_parse!(["--depth", "1"], input.depth, 1);
    test_parse!(["--solo"], input.solo, true);
}

#[test]
fn parse_global() {
    use log::LevelFilter;
    test_parse!(["-v"], log_level, LogLevel(LevelFilter::Debug));
    test_parse!(["-vv"], log_level, LogLevel(LevelFilter::Trace));
    test_parse!(["-q"], log_level, LogLevel(LevelFilter::Error));
    test_parse!(["-e"], exit_on_err, true);
    test_parse!(["--save-config"], save_config, true);
    test_parse!(["--reencode"], reencode, true);
    test_parse!(["--jobs", "8"], jobs, 8);
}

#[test]
fn parse_no_streams() {
    let xs = Streams {
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
        let val = ConfigTarget {
            streams: Some(xs.clone()),
            ..Default::default()
        };

        let mut exp = (*EMPTY_ARGS).clone();
        exp.targets.get_or_insert_default().insert(trg, val);
        assert_eq_wo_locale(cfg([arg]), &exp);
    })
}

#[test]
fn parse_streams() {
    let xs = Streams {
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
        let val = ConfigTarget {
            streams: Some(xs.clone()),
            ..Default::default()
        };

        let mut exp = (*EMPTY_ARGS).clone();
        exp.targets.get_or_insert_default().insert(trg, val);
        assert_eq_wo_locale(cfg([arg]), &exp);
    })
}

#[test]
fn parse_chapters() {
    test_parse!(["-C"], chapters.no_flag, true);
    test_parse!(
        [p("-c"), &data("srt.srt")],
        chapters.file,
        Some(data("srt.srt"))
    );
}

#[test]
fn parse_dispositions() {
    let xs = Dispositions {
        single_val: Some(true),
        ..Default::default()
    };
    test_parse!(
        ["--defaults", "true"],
        defaults,
        DefaultDispositions(xs.clone())
    );
    test_parse!(["--forceds", "true"], forceds, ForcedDispositions(xs));

    let xs = Dispositions {
        max_in_auto: Some(1),
        ..Default::default()
    };
    test_parse!(
        ["--max-defaults", "1"],
        defaults,
        DefaultDispositions(xs.clone())
    );
    test_parse!(["--max-forceds", "1"], forceds, ForcedDispositions(xs));
}

#[test]
fn parse_names() {
    let xs = Metadata {
        single_val: Some(String::from("x")),
        ..Default::default()
    };
    test_parse!(["--names", "x"], names, NameMetadata(xs));
}

#[test]
fn parse_langs() {
    let xs = Metadata {
        single_val: Some(lang!(Eng)),
        ..Default::default()
    };
    test_parse!(["--langs", "eng"], langs, LangMetadata(xs));
}

#[test]
fn parse_retiming_options() {
    let x_globset = Some("x".parse::<GlobSetPattern>().unwrap());
    test_parse!(["--parts", "x"], retiming_options.parts, x_globset.clone());
    test_parse!(["--no-linked"], retiming_options.no_linked, true);
}

#[test]
fn test_aliases_of_args() {
    [
        vec!["-v", "--verbose"],
        vec!["-vv", "-vvv", "-vvvvvvv"],
        vec!["-q", "--quiet"],
        vec!["-e", "--exit-on-err", "--exit-on-error"],
        vec!["--reencode", "--re-encode"],
        vec!["-p", "--pro"],
        vec!["-A", "--no-audio"],
        vec!["-S", "--no-subs"],
        vec!["-D", "--no-video"],
        vec!["-C", "--no-chapters"],
        vec!["-F", "--no-fonts"],
        vec!["-M", "--no-attachs"],
        vec!["--sys", "--system"],
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
        (vec!["-a", "--audio"], "1"),
        (vec!["-s", "--subs"], "1"),
        (vec!["-d", "--video"], "1"),
        (vec!["-c", "--chapters"], data("srt.srt").to_str().unwrap()),
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
        "--reencode",
        "--target",
        "subs",
        "--defaults",
        "true",
    ]);

    assert!(cfg.exit_on_err);
    assert!(cfg.target(CfgDefaults, "video").single_val.unwrap());
    assert!(cfg.target(CfgDefaults, "audio").single_val.unwrap());
    assert!(cfg.reencode);
    assert!(cfg.target(CfgDefaults, "sub").single_val.unwrap());

    assert!(cfg.defaults.single_val.is_none());
}
