use crate::common::*;
use clap::{error::ErrorKind, *};
use mux_media::{markers::*, *};
use std::sync::LazyLock;

static EMPTY_ARGS: LazyLock<MuxConfig> = LazyLock::new(|| cfg::<_, &str>([]));

fn assert_eq_wo_locale(mut left: MuxConfig, right: &MuxConfig) {
    left.locale = right.locale;
    assert_eq!(&left, right);
}

#[test]
fn test_empty_args() {
    let exp = MuxConfig {
        input: Input {
            dir: EMPTY_ARGS.input.dir.clone(),
            ..Default::default()
        },
        output: Output {
            dir: EMPTY_ARGS.output.dir.clone(),
            ..Default::default()
        },
        is_output_constructed_from_input: true,
        ..Default::default()
    };
    assert_eq_wo_locale(exp, &*EMPTY_ARGS);
}

fn assert_ok_exit(args: &[&str]) {
    let err = MuxConfig::try_parse_from(args).unwrap_err();
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
        "--list-langs-full",
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
fn test_parse() {
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
        Some(RangeU64::try_from((1, 1)).unwrap()),
        input.need_num,
        true
    );

    let x_globset = Some("x".parse::<GlobSetPattern>().unwrap());

    test_parse!(["--skip", "x"], input.skip, x_globset.clone());
    test_parse!(["--depth", "1"], input.depth, 1);
    test_parse!(["--solo"], input.solo, true);

    test_parse!(["-v"], verbosity, Verbosity::Verbose);
    test_parse!(["-vv"], verbosity, Verbosity::Debug);
    test_parse!(["-q"], verbosity, Verbosity::Quiet);
    test_parse!(["-e"], exit_on_err, true);
    test_parse!(["--save-config"], save_config, true);
    test_parse!(["--reencode"], reencode, true);
    test_parse!(["--threads", "1"], threads, 1);

    let t = Tracks {
        no_flag: true,
        ..Default::default()
    };
    test_parse!(["-A"], audio_tracks, AudioTracks(t.clone()));
    test_parse!(["-S"], sub_tracks, SubTracks(t.clone()));
    test_parse!(["-D"], video_tracks, VideoTracks(t.clone()));

    let id = TrackID::Num(1);
    let t = Tracks {
        ids_hashed: Some([id.clone()].into()),
        ..Default::default()
    };
    test_parse!(["-a", "1"], audio_tracks, AudioTracks(t.clone()));
    test_parse!(["-s", "1"], sub_tracks, SubTracks(t.clone()));
    test_parse!(["-d", "1"], video_tracks, VideoTracks(t.clone()));

    test_parse!(["-C"], chapters.no_flag, true);
    test_parse!(
        [p("-c"), &data("srt.srt")],
        chapters.file,
        Some(data("srt.srt"))
    );

    let a = Attachs {
        no_flag: true,
        ..Default::default()
    };
    test_parse!(["-F"], font_attachs, FontAttachs(a.clone()));
    test_parse!(["-M"], other_attachs, OtherAttachs(a.clone()));

    let id = AttachID::Num(1);
    let a = Attachs {
        ids_hashed: Some([id.clone()].into()),
        ..Default::default()
    };
    test_parse!(["-f", "1"], font_attachs, FontAttachs(a.clone()));
    test_parse!(["-m", "1"], other_attachs, OtherAttachs(a.clone()));

    let f = TrackFlags {
        unmapped: Some(true),
        ..Default::default()
    };
    test_parse!(
        ["--defaults", "true"],
        default_track_flags,
        DefaultTrackFlags(f.clone())
    );
    test_parse!(
        ["--forceds", "true"],
        forced_track_flags,
        ForcedTrackFlags(f.clone())
    );
    test_parse!(
        ["--enableds", "true"],
        enabled_track_flags,
        EnabledTrackFlags(f.clone())
    );

    let f = TrackFlags {
        lim_for_unset: Some(1),
        ..Default::default()
    };
    test_parse!(
        ["--max-defaults", "1"],
        default_track_flags,
        DefaultTrackFlags(f.clone())
    );
    test_parse!(
        ["--max-forceds", "1"],
        forced_track_flags,
        ForcedTrackFlags(f.clone())
    );
    test_parse!(
        ["--max-enableds", "1"],
        enabled_track_flags,
        EnabledTrackFlags(f.clone())
    );

    test_parse!(
        ["--names", "x"],
        track_names.unmapped,
        Some(String::from("x"))
    );
    test_parse!(
        ["--langs", "eng"],
        track_langs.unmapped,
        Some(LangCode::Eng)
    );

    test_parse!(
        ["--rm-segments", "x"],
        retiming.rm_segments,
        x_globset.clone()
    );
    test_parse!(["--no-linked"], retiming.no_linked, true);
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
        vec!["-S", "--no-subs", "--no-subtitles"],
        vec!["-D", "--no-video"],
        vec!["-C", "--no-chapters"],
        vec!["-F", "--no-fonts"],
        vec!["-M", "--no-attachs", "--no-attachments"],
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
        (vec!["-a", "--audio", "--audio-tracks"], "1"),
        (vec!["-s", "--subs", "--subtitle-tracks"], "1"),
        (vec!["-d", "--video", "--video-tracks"], "1"),
        (vec!["-c", "--chapters"], data("srt.srt").to_str().unwrap()),
        (vec!["-f", "--fonts"], "1"),
        (vec!["-m", "--attachs", "--attachments"], "1"),
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
        "--no-audio",
        "--target",
        "audio",
        "--no-audio",
        "--target",
        "global",
        "--reencode",
        "--target",
        "subs",
        "--no-audio",
        "--target",
        "global",
        "--no-video",
    ]);

    assert!(cfg.exit_on_err);
    assert!(cfg.target(MCAudioTracks, ["video"]).0.no_flag);
    assert!(cfg.target(MCAudioTracks, ["audio"]).0.no_flag);
    assert!(cfg.reencode);
    assert!(cfg.target(MCAudioTracks, ["subs"]).0.no_flag);
    assert!(cfg.video_tracks.0.no_flag);

    // Sure that global audio_tracks has not true no_flag.
    assert!(!cfg.audio_tracks.0.no_flag);
}
