use crate::common::*;
use mux_media::*;
use std::{ffi::OsString, fs};

#[test]
fn test_from_output() {
    [
        (Muxer::AVI, "avi"),
        (Muxer::MP4, "mp4"),
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
        assert_eq!(expected, Muxer::from(&out));
    })
}

macro_rules! test_mux_current_any {
    ($fn:ident, $in_arg:expr, $out_arg:expr, $muxer:ident) => {
        #[test]
        fn $fn() {
            let in_arg = data($in_arg);
            let out_arg = data($out_arg);

            let mut mc = cfg([p("-i"), &in_arg, p("-o"), &out_arg, p("-e")]);
            mc.try_finalize_init().unwrap();

            let mut media_info = MediaInfo::from(&mc);
            let media = mc.input.iter_media_grouped_by_stem().next().unwrap();
            media_info.try_insert_many(media.files, true).unwrap();

            let mut oss = OsString::from("other_name.");
            oss.push(Muxer::$muxer.as_ext());
            let expected = out_arg.parent().unwrap().join(oss);

            let _ = fs::remove_file(&expected);
            assert!(
                !expected.exists(),
                "Should not exists '{}'",
                expected.display()
            );

            match Muxer::$muxer.mux_current(&mut media_info, &expected) {
                MuxCurrent::Err(e) => panic!("{}", e),
                MuxCurrent::Continue => panic!("Unexpected MuxCurrent::Continue"),
                _ => assert!(expected.exists(), "Should exists '{}'", expected.display()),
            }
        }
    };
}

test_mux_current_any!(
    test_mux_current_matroska,
    "x1_set/",
    "output/mux/matroska/,.mkv",
    Matroska
);
test_mux_current_any!(test_mux_current_avi, "x1_set/", "output/mux/avi/,.avi", AVI);
test_mux_current_any!(test_mux_current_mp4, "x1_set/", "output/mux/mp4/,.mp4", MP4);
test_mux_current_any!(
    test_mux_current_webm,
    "x1_set/",
    "output/mux/webm/,.webm",
    Webm
);
