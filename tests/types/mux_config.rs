use crate::common::*;
use clap::{error::ErrorKind, *};
use mux_media::{markers::*, *};

#[test]
fn test_default() {
    let cfg = MuxConfig::default();
    assert!(cfg.is_default());
    assert!(cfg.input.is_default());
    assert!(cfg.output.is_default());
    assert!(cfg.locale.is_default());
    assert!(cfg.exit_on_err.is_default());
    assert!(cfg.save_config.is_default());
    assert!(cfg.reencode.is_default());
    assert_eq!(cfg.threads, 4);
    assert!(cfg.auto_flags.is_default());
    assert!(cfg.audio_tracks.is_default());
    assert!(cfg.sub_tracks.is_default());
    assert!(cfg.video_tracks.is_default());
    assert!(cfg.chapters.is_default());
    assert!(cfg.font_attachs.is_default());
    assert!(cfg.other_attachs.is_default());
    assert!(cfg.default_track_flags.is_default());
    assert!(cfg.forced_track_flags.is_default());
    assert!(cfg.enabled_track_flags.is_default());
    assert!(cfg.track_names.is_default());
    assert!(cfg.track_langs.is_default());
    assert!(cfg.specials.is_default());
    assert!(cfg.targets.is_default());
    assert!(cfg.tool_paths.is_default());
    assert!(cfg.muxer.is_default());
    assert!(cfg.is_output_constructed_from_input.is_default());
}

#[test]
fn test_empty_input() {
    let parsed = cfg::<_, &str>([]);
    let mut exp = MuxConfig::default();
    exp.input.dir = parsed.input.dir.clone();
    exp.output.dir = parsed.output.dir.clone();
    exp.locale = parsed.locale;
    exp.is_output_constructed_from_input = true;
    assert_eq!(parsed, exp);
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

#[test]
fn test_ok_exit_ffprobe_help() {
    assert_ok_exit(&["--ffprobe", "-h"]);
}

#[test]
fn test_ok_exit_mkvmerge_help() {
    assert_ok_exit(&["--mkvmerge", "-h"]);
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

#[test]
fn test_locale() {
    [
        ("rus", LangCode::Rus),
        ("jpn", LangCode::Jpn),
        ("eng", LangCode::Eng),
    ]
    .into_iter()
    .for_each(|(arg, lang)| assert_eq!(cfg(["--locale", arg]).locale, lang))
}

crate::build_test_parseable_args!(
    test_parseable_args, MuxConfig;
    Input => "input",
    Output => "output",
    Range => "range",
    Skip => "skip",
    Depth => "depth",
    Solo => "solo",
    Locale => "locale",
    Verbose => "verbose",
    Quiet => "quiet",
    ExitOnErr => "exit-on-err",
    Json => "json",
    SaveConfig => "save-config",
    Reencode => "reencode",
    Threads => "threads",
    RmSegments => "rm-segments",
    NoLinked => "no-linked",
    LessRetiming => "less-retiming",
    Pro => "pro",
    HelpAutoDefaults => "auto-defaults / --no-auto-defaults",
    AutoDefaults => "auto-defaults",
    NoAutoDefaults => "no-auto-defaults",
    HelpAutoForceds => "auto-forceds / --no-auto-forceds",
    AutoForceds => "auto-forceds",
    NoAutoForceds => "no-auto-forceds",
    HelpAutoEnableds => "auto-enableds / --no-auto-enableds",
    AutoEnableds => "auto-enableds",
    NoAutoEnableds => "no-auto-enableds",
    HelpAutoNames => "auto-names / --no-auto-names",
    AutoNames => "auto-names",
    NoAutoNames => "no-auto-names",
    HelpAutoLangs => "auto-langs / --no-auto-langs",
    AutoLangs => "auto-langs",
    NoAutoLangs => "no-auto-langs",
    HelpAutoCharsets => "auto-charsets / --no-auto-charsets",
    AutoCharsets => "auto-charsets",
    NoAutoCharsets => "no-auto-charsets",
    Target => "target",
    TargetHelp => "target <trg> [options]",
    ListTargets => "list-targets",
    Audio => "audio",
    AudioTracks => "audio-tracks",
    NoAudio => "no-audio",
    Subs => "subs",
    SubtitleTracks => "subtitle-tracks",
    NoSubs => "no-subs",
    NoSubtitles => "no-subtitles",
    Video => "video",
    VideoTracks => "video-tracks",
    NoVideo => "no-video",
    Chapters => "chapters",
    NoChapters => "no-chapters",
    Attachments => "attachments",
    NoAttachments => "no-attachments",
    Fonts => "fonts",
    NoFonts => "no-fonts",
    Attachs => "attachs",
    NoAttachs => "no-attachs",
    DefaultTrackFlag => "default-track-flag",
    Defaults => "defaults",
    MaxDefaults => "max-defaults",
    ForcedDisplayFlag => "forced-display-flag",
    Forceds => "forceds",
    MaxForceds => "max-forceds",
    Enableds => "enableds",
    MaxEnableds => "max-enableds",
    TrackEnabledFlag => "track-enabled-flag",
    Metadata => "metadata",
    Names => "names",
    Title => "title",
    TrackName => "track-name",
    Langs => "langs",
    Language => "language",
    Specials => "specials",
    ListContainers => "list-containers",
    ListLangs => "list-langs",
    UserTools => "user-tools",
    Ffmpeg => "ffmpeg",
    FfmpegHelp => "ffmpeg [options]",
    Ffprobe => "ffprobe",
    FfprobeHelp => "ffprobe [options]",
    Mkvmerge => "mkvmerge",
    MkvmergeHelp => "mkvmerge [options]",
    Version => "version",
    Help => "help",
    SubCharset => "sub-charset",
    TrackOrder => "track-order",
);
