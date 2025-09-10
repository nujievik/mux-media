#[macro_export]
macro_rules! take_mi_cache {
    ($path:expr, $( $field:ident ),* $(,)?) => {{
        let mc = $crate::common::cfg::<[&str; 0], &str>([]);
        #[allow(unused_mut)]
        let mut mi = mux_media::MediaInfo::from(&mc);
        $( mi.get::<$field>($path).unwrap(); )*
        mi.cache
    }};
}

#[macro_export]
macro_rules! fn_variants_of_args {
    ( $( $arg:expr => $vars:expr ),* $(,)?) => {
        fn variants_of_args<I, S>(args: I) -> Vec<Vec<std::ffi::OsString>>
        where
            I: IntoIterator<Item = S>,
            S: Into<std::ffi::OsString>,
        {
            let mut variants: Vec<Vec<std::ffi::OsString>> = Vec::new();

            let args: Vec<std::ffi::OsString> = args
                .into_iter()
                .map(|arg| arg.into())
                .collect();

            for (i, arg) in args.iter().enumerate() {
                let alts: Option<Vec<&str>> = match arg.to_str().unwrap() {
                    $( $arg => Some($vars), )*
                    _ => None,
                };

                if let Some(alts) = alts {
                    for alt in alts {
                        let mut new_args = args.clone();
                        new_args[i] = alt.into();
                        variants.push(new_args);
                    }
                }
            }

            variants.push(args);

            variants
        }
    };
}

#[macro_export]
macro_rules! compare_arg_cases {
    ($cases:expr, $var_args:ident, $file:expr, $field:ident, $( $cache_field:ident ),* $(,)?) => {{
        let path = $crate::common::data($file);
        let cache = $crate::take_mi_cache!(&path, $( $cache_field, )*);

        for (exp, args) in $cases {
            let exp = $crate::common::to_os_args(exp);

            $var_args(args).into_iter().for_each(|args| {
                assert_eq!(exp, $crate::common::cfg_args::<$field, _, _>(args, &path, cache.clone()));
            });
        }
    }};
}

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
macro_rules! build_test_parseable_args {
    ($fn:ident, $ty:ty; $( $arg:ident => $s:expr ),* $(,)?) => {
        #[test]
        fn $fn() {
            $(
                let arg = <$ty as ParseableArgs>::Arg::$arg;
                assert_eq!(&format!("--{}", $s), arg.dashed());
                assert_eq!($s, arg.undashed());
            )*
        }
    };
}

#[macro_export]
macro_rules! build_test_to_json_args {
    (@body, $field:ident, $json_dir:expr; $( $left:expr, $right:expr ),* ) => {{
        let dir = std::path::Path::new("output").join("to_json_args").join($json_dir);
        let dir = $crate::common::data(&dir);

        let in_dir = dir.to_str().unwrap();
        let mut out_dir = dir.join("muxed").to_str().unwrap().to_string();
        out_dir.push_str(",.mkv");

        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let add_args = vec!["--locale", "eng", "--input", in_dir, "--output", &out_dir, "--save-config"];
        let json = dir.clone().join(mux_media::MuxConfig::JSON_NAME);

        $(
            let mc_args = $crate::common::append_str_vecs([add_args.clone(), $right]);
            let mc = $crate::common::cfg(mc_args);

            let left = $crate::common::to_args::<Vec<&str>, _>($left.clone());
            let right = mc.$field.to_json_args();
            assert_eq!(left, right);

            let left = $crate::common::append_str_vecs([add_args.clone(), $left]);
            mc.try_save_config().unwrap();
            let right = $crate::common::read_json_args(&json);

            assert_eq!(left, right, "from json err");
        )*

        let _ = std::fs::remove_dir_all(&dir);
    }};

    ( $fn:ident, $field:ident, $json_dir:expr; $( $args:expr ),* $(,)? ) => {
        #[test]
        fn $fn() {
            $crate::build_test_to_json_args!(@body, $field, $json_dir; $( $args.clone(), $args ),* );
        }
    };

    ( $fn:ident, $field:ident, $json_dir:expr, @diff_in_out; $( $left:expr, $right:expr ),* $(,)? ) => {
        #[test]
        fn $fn() {
            $crate::build_test_to_json_args!(@body, $field, $json_dir; $( $left, $right ),* );
        }
    };
}
