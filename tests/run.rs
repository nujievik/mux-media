#[allow(dead_code)]
mod common;

use crate::common::*;
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

            let expected = c.output.build_out("x1_set");
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

test_mux_any!(test_mux_matroska, "x1_set/", "mux/matroska/,.mkv");
