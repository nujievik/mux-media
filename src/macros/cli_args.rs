#[macro_export]
macro_rules! cli_args {
    ($type:ident, $enum_arg:ident; $( $arg:ident ),* $(,)?) => {
        impl $crate::CLIArgs for $type {
            type Arg = $enum_arg;
        }

        #[derive(Clone, Copy)]
        pub enum $enum_arg {
            $( $arg ),*
        }
    };

    ($type:ident, $enum_arg:ident;
    $( $arg:ident => $long:expr, $mkvmerge:expr ),* $(,)?) => {
        $crate::cli_args!($type, $enum_arg; $( $arg ),*);

        impl $crate::CLIArg for $enum_arg {
            fn as_long(self) -> &'static str {
                match self {
                    $( Self::$arg => $long ),*
                }
            }

            fn to_mkvmerge(self) -> Option<&'static str> {
                match self {
                    $( Self::$arg => Some($mkvmerge) ),*
                }
            }
        }
    };

    ($type:ident, $enum_arg:ident; $( $arg:ident => $long:expr ),* $(,)?) => {
        $crate::cli_args!($type, $enum_arg; $( $arg ),*);

        impl $crate::CLIArg for $enum_arg {
            fn as_long(self) -> &'static str {
                match self {
                    $( Self::$arg => $long ),*
                }
            }

            fn to_mkvmerge(self) -> Option<&'static str> {
                None
            }
        }
    };
}

#[macro_export]
macro_rules! ok_or_return_vec_new {
    ($x:expr) => {
        match $x {
            Some(x) => x,
            None => {
                eprintln!("Unexpected None. Return empty");
                return Vec::new();
            }
        }
    };
}

#[macro_export]
macro_rules! to_mkvmerge_args {
    (@fn) => {
        fn to_mkvmerge_args(
            &self,
            mi: &mut $crate::MediaInfo,
            path: &std::path::Path,
        ) -> Vec<String> {
            let args = self
                .to_os_mkvmerge_args(mi, path)
                .into_iter()
                .map(|arg| arg.into_string())
                .collect();

            match args {
                Ok(vec) => vec,
                Err(bad_arg) => {
                    eprintln!(
                        "Err convert OsString arg to String: {:?}. Return empty",
                        bad_arg
                    );
                    Vec::new()
                }
            }
        }
    };

    (@fn_os) => {
        fn to_os_mkvmerge_args(
            &self,
            mi: &mut $crate::MediaInfo,
            path: &std::path::Path,
        ) -> Vec<std::ffi::OsString> {
            self.to_mkvmerge_args(mi, path)
                .into_iter()
                .map(std::ffi::OsString::from)
                .collect()
        }
    };

    (@cli_arg, $arg:ident) => {
        $crate::ok_or_return_vec_new!(<Self as $crate::CLIArgs>::Arg::$arg.to_mkvmerge())
            .to_string()
    };
}
