use crate::common::*;
use mux_media::*;

#[test]
fn test_from_output() {
    [
        (Muxer::Matroska, "avi"),
        (Muxer::Matroska, "mp4"),
        (Muxer::Matroska, "mkv"),
        (Muxer::Matroska, "webm"),
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
