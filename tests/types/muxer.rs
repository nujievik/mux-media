use crate::common::*;
use mux_media::*;
use std::{ffi::OsString, fs};

#[test]
fn test_from_output() {
    [
        (Muxer::Avi, "avi"),
        (Muxer::Mp4, "mp4"),
        (Muxer::Matroska, "mkv"),
        (Muxer::Webm, "webm"),
        (Muxer::Matroska, "x"),
        (Muxer::Matroska, "abc"),
        (Muxer::Matroska, "rand"),
    ]
    .into_iter()
    .for_each(|(expected, ext)| {
        let s = format!(",.{}", ext);
        let out = Output::try_from_path(data(s)).unwrap();
        assert_eq!(expected, Muxer::new(&out));
    })
}
