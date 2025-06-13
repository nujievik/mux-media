#[macro_export]
macro_rules! test_cli_args {
    ($type:ident;
    $( $arg:ident => $long:expr ),* $(,)?) => {{
        $(
            assert_eq!($long, <$type as mux_media::CLIArgs>::Arg::$arg.as_long());
            assert_eq!(None, <$type as mux_media::CLIArgs>::Arg::$arg.to_mkvmerge());
        )*
    }};

    ($type:ident;
    $( $arg:ident => $long:expr, $mkvmerge:expr ),* $(,)?) => {{
        $(
            assert_eq!($long, <$type as mux_media::CLIArgs>::Arg::$arg.as_long());
            assert_eq!(Some($mkvmerge), <$type as mux_media::CLIArgs>::Arg::$arg.to_mkvmerge());
        )*
    }};
}

#[macro_export]
macro_rules! take_mi_cache {
    ($path:expr, $( $field:ident ),* $(,)?) => {{
        let mc = $crate::common::cfg::<[&str; 0], &str>([]);
        let mut mi = mux_media::MediaInfo::from(&mc);
        $( mi.get::<$field>($path).unwrap(); )*
        mi.take_cache()
    }};
}

#[macro_export]
macro_rules! compare_arg_cases {
    ($cases:expr, $to_long:ident, $file:expr, $field:ident, $( $cache_field:ident ),* $(,)?) => {{
        let path = data_file($file);
        let cache = $crate::take_mi_cache!(&path, $( $cache_field, )*);

        for (exp, args) in $cases {
            let exp = $crate::common::to_args(exp);
            let alt_args_raw = $to_long(&args);
            let alt_args = alt_args_raw.iter().map(|x| x.as_str()).collect();

            assert_eq!(exp, $crate::common::cfg_args::<$field>(args, &path, cache.clone()));
            assert_eq!(exp, $crate::common::cfg_args::<$field>(alt_args, &path, cache.clone()));
        }
    }}
}
