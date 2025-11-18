use crate::common::*;
use mux_media::{markers::*, *};
use std::{ffi::OsString, path::Path, sync::LazyLock};

static CACHE_LANGS_MKV: LazyLock<CacheMI> = LazyLock::new(|| {
    let file = data("streams_order/langs.mkv");
    let cfg = cfg::<_, &str>([]);
    let mut mi = MediaInfo::new(&cfg, 0);
    mi.try_init(MIStreams, &file).unwrap();
    mi.cache
});

fn body_test_order(args: &[&str], expected: [usize; 3]) {
    let cfg = cfg(args);
    let mut mi = MediaInfo::new(&cfg, 0);
    mi.cache = CACHE_LANGS_MKV.clone();
    mi.try_finalize_init_streams().unwrap();

    let order = mi.try_take_cmn(MICmnStreamsOrder).unwrap();

    expected.iter().enumerate().for_each(|(i, exp)| {
        assert_eq!(exp, &order[i].i_stream);
    });

    let mut exp_ffmpeg = Vec::<OsString>::new();
    exp_ffmpeg.push("-i".into());
    exp_ffmpeg.push(data("streams_order/langs.mkv").into());

    expected.iter().for_each(|exp| {
        exp_ffmpeg.push("-map".into());
        exp_ffmpeg.push(format!("0:{}", exp).into());
    });

    expected.iter().enumerate().for_each(|(i, _)| {
        exp_ffmpeg.push(format!("-c:{}", i).into());
        exp_ffmpeg.push("copy".into());
    });

    mi.set_cmn(MICmnStreamsOrder, order);
    assert_eq!(exp_ffmpeg, StreamsOrder::to_ffmpeg_args(&mut mi).unwrap());
}

#[test]
fn test_lang_order() {
    body_test_order(&["--locale", "jpn"], [0, 1, 2]);
    body_test_order(&["--locale", "eng"], [1, 2, 0]);
    body_test_order(&["--locale", "rus"], [2, 1, 0]);
}

fn body_test_any_flags_order(flags_arg: &str) {
    body_test_order(&[flags_arg, "0:true", "--locale", "eng"], [0, 1, 2]);
    body_test_order(&[flags_arg, "1:true", "--locale", "eng"], [1, 2, 0]);
    body_test_order(&[flags_arg, "2:true", "--locale", "eng"], [2, 1, 0]);

    body_test_order(&[flags_arg, "1-2:false", "--locale", "eng"], [0, 1, 2]);
    body_test_order(
        &[flags_arg, "0:false,2:false", "--locale", "eng"],
        [1, 2, 0],
    );
    body_test_order(&[flags_arg, "0-1:false", "--locale", "eng"], [2, 1, 0]);
}

#[test]
fn test_defaults_order() {
    body_test_any_flags_order("--defaults");
}

#[test]
fn test_forceds_order() {
    body_test_any_flags_order("--forceds");
}

#[test]
fn test_it_signs_order() {
    body_test_order(&["--names", "0:signs", "--locale", "eng"], [0, 1, 2]);
    body_test_order(&["--names", "1:signs", "--locale", "eng"], [1, 2, 0]);
    body_test_order(&["--names", "2:signs", "--locale", "eng"], [2, 1, 0]);

    body_test_order(&["--names", "0:x надписи", "--locale", "eng"], [0, 1, 2]);
    body_test_order(&["--names", "1:ab надписи", "--locale", "eng"], [1, 2, 0]);
    body_test_order(&["--names", "2:cde_надписи", "--locale", "eng"], [2, 1, 0]);

    body_test_order(&["--names", "0:SIGns", "--locale", "eng"], [0, 1, 2]);
    body_test_order(&["--names", "0:НаДпИси", "--locale", "eng"], [0, 1, 2]);
}

#[test]
fn test_track_type_order() {
    let file = data("streams_order/reverse_stream_types.mkv");
    let cfg = cfg::<_, &str>([]);
    let mut mi = MediaInfo::new(&cfg, 0);
    mi.try_init(MIStreams, &file).unwrap();
    let order = mi.try_take_cmn(MICmnStreamsOrder).unwrap();

    [2, 1, 0].into_iter().enumerate().for_each(|(i, exp)| {
        assert_eq!(exp, order[i].i_stream);
    })
}

#[test]
fn test_path_name_order() {
    let dir = data("streams_order/path_name/");
    let cfg = cfg::<_, &str>([]);
    let mut mi = MediaInfo::new(&cfg, 0);

    let files = [
        "First/srt.srt",
        "Second/srt.srt",
        "Third/srt.srt",
        "srt.srt",
    ];

    files
        .iter()
        .for_each(|f| mi.try_insert(dir.join(f)).unwrap());

    let order = mi.try_take_cmn(MICmnStreamsOrder).unwrap();
    files.iter().enumerate().for_each(|(i, f)| {
        assert_eq!(&dir.join(f), order[i].src());
    })
}
