#[macro_export]
macro_rules! test_from_str {
    ($type:ty, $err_cases:expr, @err) => {{
        for s in $err_cases {
            assert!(s.parse::<$type>().is_err(), "Fail is_err() parse '{}'", s);
        }
    }};

    ($type:ty, $test_fn:ident, $cases:expr, $err_cases:expr) => {
        #[test]
        fn $test_fn() {
            for s in $cases {
                assert!(s.parse::<$type>().is_ok(), "Fail is_ok() parse '{}'", s);
            }

            $crate::test_from_str!($type, $err_cases, @err);
        }
    };

    ($type:ty, $test_fn:ident, $cases:expr, $err_cases:expr, @ok_compare) => {
        #[test]
        fn $test_fn() {
            for (exp, s) in $cases {
                assert!(exp == s.parse::<$type>().unwrap(), "Fail == parse '{}'", s);
            }

            $crate::test_from_str!($type, $err_cases, @err);
        }
    };
}

#[macro_export]
macro_rules! build_test_to_args {
    (@body, $field:ident, $txt_dir:expr; $( $left:expr, $right:expr ),* ) => {{
        let dir = std::path::Path::new("to_args").join($txt_dir);
        let dir = $crate::common::temp(&dir);

        let in_dir = dir.to_str().unwrap();
        let mut out_dir = dir.join("muxed").to_str().unwrap().to_string();
        out_dir.push_str(",.mkv");

        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::create_dir_all(&dir);

        let add_args = vec!["--locale", "eng", "--input", in_dir, "--output", &out_dir, "--save-config"];
        let txt = dir.clone().join("mux-media-config.txt");

        $(
            let mc_args = $crate::common::append_str_vecs([add_args.clone(), $right]);
            let mc = $crate::common::cfg(mc_args);

            let left = $crate::common::to_args::<Vec<&str>, _>($left.clone());
            let right = mc.$field.to_args();
            assert_eq!(left, right);

            let left = $crate::common::append_str_vecs([add_args.clone(), $left]);
            mc.try_save_config().unwrap();
            let right = $crate::common::read_txt_args(&txt);

            assert_eq!(left, right, "from txt err");
        )*
    }};

    ( $fn:ident, $field:ident, $txt_dir:expr; $( $args:expr ),* $(,)? ) => {
        #[test]
        fn $fn() {
            $crate::build_test_to_args!(@body, $field, $txt_dir; $( $args.clone(), $args ),* );
        }
    };

    ( $fn:ident, $field:ident, $txt_dir:expr, @diff_in_out; $( $left:expr, $right:expr ),* $(,)? ) => {
        #[test]
        fn $fn() {
            $crate::build_test_to_args!(@body, $field, $txt_dir; $( $left, $right ),* );
        }
    };
}
