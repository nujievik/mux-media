#[doc(hidden)]
#[macro_export]
macro_rules! mkvmerge_arg {
    ($ty:ty, $arg:expr) => {
        impl $crate::MkvmergeArg for $ty {
            const MKVMERGE_ARG: &'static str = $arg;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! mkvmerge_no_arg {
    ($ty:ty, $no_arg:expr) => {
        impl $crate::MkvmergeNoArg for $ty {
            const MKVMERGE_NO_ARG: &'static str = $no_arg;
        }
    };
}

#[doc(hidden)]
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

#[doc(hidden)]
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
                use $crate::MkvmergeArg;
                use $crate::markers::{MISavedTracks, MITILang};

                let add = mi.pro_flags.$add_marker;
                let nums: Vec<u64> = $crate::unwrap_or_return_vec!(mi.get::<MISavedTracks>(path))
                    .values()
                    .flat_map(|nums| nums.iter().copied())
                    .collect();

                let val_args: Vec<String> = nums
                    .into_iter()
                    .filter_map(|num| {
                        let val = self.get(&TrackID::Num(num)).or_else(|| {
                            mi.get_ti::<MITILang>(path, num)
                                .and_then(|lang| self.get(&TrackID::Lang(*lang)))
                        });

                        val.map(|v| format!("{}:{}", num, v))
                            .or_else(|| {
                                add.then(|| {
                                    mi.get_ti::<$crate::markers::$tic_marker>(path, num)
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
