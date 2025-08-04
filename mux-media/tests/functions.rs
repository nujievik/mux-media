#![allow(dead_code)]

mod common;

use crate::common::*;
use mux_media::{markers::*, *};
use std::{fs, path::Path};

macro_rules! test_mux_any {
    ($fn:ident, $in_arg:expr, $out_arg:expr) => {
        #[test]
        fn $fn() {
            let in_arg = data($in_arg);
            let out_arg = data($out_arg);

            let p: fn(&str) -> &Path = Path::new;
            let mut mc = cfg([p("-i"), &in_arg, p("-o"), &out_arg, p("-e")]);
            mc.try_finalize_init().unwrap();

            let expected = mc.field::<MCOutput>().build_out("x1_set");
            let _ = fs::remove_file(&expected);

            assert!(
                !expected.exists(),
                "Should not exists '{}'",
                expected.display()
            );
            mux(&mc).unwrap();
            assert!(expected.exists(), "Should exists '{}'", expected.display());
        }
    };
}

test_mux_any!(test_mux_matroska, "x1_set/", "output/mux/matroska/,.mkv");
test_mux_any!(test_mux_avi, "x1_set/", "output/mux/avi/,.avi");
test_mux_any!(test_mux_mp4, "x1_set/", "output/mux/mp4/,.mp4");
test_mux_any!(test_mux_webm, "x1_set/", "output/mux/webm/,.webm");
