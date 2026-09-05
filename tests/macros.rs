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
    ( $fn:ident, $txt_dir:expr; $( $args:expr ),* $(,)? ) => {
        #[test]
        fn $fn() {
            let dir = std::path::Path::new("to_args").join($txt_dir);
            let dir = $crate::common::temp(&dir);

            let in_dir = dir.to_str().unwrap();
            let mut out_dir = dir.join("muxed").to_str().unwrap().to_string();
            out_dir.push_str(",.mkv");

            let _ = std::fs::remove_dir_all(&dir);
            let _ = std::fs::create_dir_all(&dir);

            let add_args = vec!["--locale", "eng", "--input", in_dir, "--output", &out_dir, "--save-config"];
            let txt = dir.clone().join(".mux-media").join("config.txt");

            $(
                let cfg_args = $crate::common::append_str_vecs([add_args.clone(), $args.clone()]);
                let cfg = $crate::common::cfg(cfg_args);
                let left = $crate::common::append_str_vecs([&add_args[..add_args.len() - 1], $args.as_slice()]);

                assert_eq!(&left, &cfg.to_args(), "from config struct err");

                cfg.try_save_config().unwrap();
                assert_eq!(left, $crate::common::read_txt_args(&txt), "from txt err");
            )*
        }
    };
}
