mod common;

use crate::common::*;
use ffmpeg_next as ffmpeg;
use mux_media::*;
use std::fs;

macro_rules! test_mux_any {
    ($fn:ident, $in_arg:expr, $out_arg:expr) => {
        #[test]
        fn $fn() {
            let in_arg = data($in_arg);
            let out_arg = temp($out_arg);

            let mut c = cfg([p("-i"), &in_arg, p("-o"), &out_arg, p("-e")]);
            c.try_finalize_init().unwrap();

            let expected = c.output.dir().join("x1_set.mkv");
            let _ = fs::remove_file(&expected);

            assert!(
                !expected.exists(),
                "Should not exists '{}'",
                expected.display()
            );
            c.mux().unwrap();
            assert!(expected.exists(), "Should exists '{}'", expected.display());
        }
    };
}

test_mux_any!(test_mux_matroska, "x1_set/", "mux/matroska");

#[test]
fn mux_attached_pic() {
    let i = data("cover.mkv");
    let o = temp("remuxed-cover.mkv");
    let _ = fs::remove_file(&o);

    let cfg = cfg(["--orphans-too"]);
    let mut mi = MediaInfo::new(&cfg, 0);
    mi.try_insert(&i).unwrap();
    mi.mux_files(&o).unwrap();

    let ctx = ffmpeg::format::input(&o).unwrap();
    let st = ctx.stream(1).unwrap();
    let av_st = unsafe { &*st.as_ptr() };

    if (av_st.disposition & ffmpeg::ffi::AV_DISPOSITION_ATTACHED_PIC) == 0 {
        panic!("not attached pic disposition");
    }

    assert_eq!(182, av_st.attached_pic.size);
}
