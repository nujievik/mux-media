#[macro_export]
macro_rules! mkvmerge_arg {
    ($type:ident, $arg:expr) => {
        impl $crate::MkvmergeArg for $type {
            const MKVMERGE_ARG: &'static str = $arg;
        }
    };
}

#[macro_export]
macro_rules! mkvmerge_no_arg {
    ($type:ident, $no_arg:expr) => {
        impl $crate::MkvmergeNoArg for $type {
            const MKVMERGE_NO_ARG: &'static str = $no_arg;
        }
    };
}

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
macro_rules! unwrap_or_return_vec {
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
        $crate::unwrap_or_return_vec!(<Self as $crate::CLIArgs>::Arg::$arg.to_mkvmerge())
            .to_string()
    };

    (@names_or_langs, $typ:ident, $arg:ident, $add_marker:ident, $tic_marker:ident) => {
        impl $crate::ToMkvmergeArgs for $typ {
            fn to_mkvmerge_args(&self, mi: &mut $crate::MediaInfo, path: &std::path::Path) -> Vec<String> {
                use $crate::{MISavedTracks, MkvmergeArg};

                let add = mi.off_on_pro.$add_marker;
                let nums: Vec<u64> = $crate::unwrap_or_return_vec!(mi.get::<MISavedTracks>(path))
                    .values()
                    .flat_map(|nums| nums.iter().copied())
                    .collect();

                let val_args: Vec<String> = nums
                    .into_iter()
                    .filter_map(|num| {
                        let val = self.get(&TrackID::Num(num)).or_else(|| {
                            mi.get_ti::<$crate::MITILang>(path, num)
                                .and_then(|lang| self.get(&TrackID::Lang(*lang)))
                        });

                        val.map(|v| format!("{}:{}", num, v))
                            .or_else(|| {
                                add.then(|| {
                                    mi.get_ti::<$crate::$tic_marker>(path, num)
                                        .map(|x| format!("{}:{}", num, x))
                                }).flatten()
                            })
                    })
                    .collect();

                if val_args.is_empty() {
                    return Vec::new();
                }

                let mut args: Vec<String> = Vec::with_capacity(val_args.len() * 2);
                for val in val_args {
                    args.push(Self::MKVMERGE_ARG.into());
                    args.push(val);
                }

                args
            }

            to_mkvmerge_args!(@fn_os);
        }
    };
}
