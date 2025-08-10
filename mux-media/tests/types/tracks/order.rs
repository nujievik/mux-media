use crate::common::*;
use mux_media::{markers::*, *};
use once_cell::sync::Lazy;
use std::{ffi::OsString, path::Path};

static CACHE_LANGS_MKV: Lazy<CacheMI> = Lazy::new(|| {
    let file = data("order/langs.mkv");
    let mux_config = cfg::<_, &str>([]);
    let mut mi = MediaInfo::from(&mux_config);
    mi.try_init::<MIMatroska>(&file).unwrap();
    mi.take_cache()
});

fn body_test_order(args: &[&str], expected: [u64; 3]) {
    let mux_config = cfg(args);
    let mut mi = MediaInfo::from(&mux_config);
    mi.set_cache(CACHE_LANGS_MKV.clone());

    let order = mi.try_take_cmn::<MICmnTrackOrder>().unwrap();
    let itt = order.i_track_type();

    expected.iter().enumerate().for_each(|(i, exp)| {
        assert_eq!(*exp, itt[i].1);
    });

    let mut exp_mkvmerge: Vec<OsString> = Vec::with_capacity(2);
    exp_mkvmerge.push("--track-order".into());
    exp_mkvmerge.push(format!("0:{},0:{},0:{}", expected[0], expected[1], expected[2]).into());

    assert_eq!(exp_mkvmerge, order.to_mkvmerge_args(&mut mi, Path::new("")));

    let mut exp_ffmpeg: Vec<OsString> = Vec::with_capacity(8);
    exp_ffmpeg.push("-i".into());
    exp_ffmpeg.push(data("order/langs.mkv").into());

    expected.iter().for_each(|exp| {
        exp_ffmpeg.push("-map".into());
        exp_ffmpeg.push(format!("0:{}", exp).into());
    });

    expected.iter().enumerate().for_each(|(i, _)| {
        exp_ffmpeg.push(format!("-c:{}", i).into());
        exp_ffmpeg.push("copy".into());
    });

    mi.set_cmn::<MICmnTrackOrder>(order);
    assert_eq!(exp_ffmpeg, TrackOrder::to_ffmpeg_args(&mut mi));
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
fn test_enableds_order() {
    body_test_any_flags_order("--enableds");
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
    let file = data("order/reverse_track_types.mkv");
    let mux_config = cfg::<_, &str>([]);
    let mut mi = MediaInfo::from(&mux_config);
    mi.try_init::<MIMatroska>(&file).unwrap();

    let order = mi.try_take_cmn::<MICmnTrackOrder>().unwrap();
    let itt = order.i_track_type();

    [2, 1, 0].into_iter().enumerate().for_each(|(i, exp)| {
        assert_eq!(exp, itt[i].1);
    })
}

#[test]
fn test_path_name_order() {
    let dir = data("order/path_name/");
    let mux_config = cfg::<_, &str>([]);
    let mut mi = MediaInfo::from(&mux_config);

    let files = [
        "First/srt.srt",
        "Second/srt.srt",
        "Third/srt.srt",
        "srt.srt",
    ];

    files
        .iter()
        .for_each(|f| mi.try_insert(dir.join(f)).unwrap());

    let order = mi.try_take_cmn::<MICmnTrackOrder>().unwrap();
    let media = order.media();

    files.iter().enumerate().for_each(|(i, f)| {
        assert_eq!(&dir.join(f), media[i].as_path());
    })
}
