#[doc(hidden)]
#[macro_export]
macro_rules! unwrap_or_return {
    ($x:expr) => {
        match $x {
            Some(x) => x,
            None => {
                return;
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! to_mkvmerge_args {
    (@names_or_langs, $ty:ty, $arg:ident, $add_marker:ident, $tic_marker:ident) => {
        impl $crate::ToMkvmergeArgs for $ty {
            fn try_append_mkvmerge_args(
                &self,
                args: &mut Vec<std::ffi::OsString>,
                mi: &mut $crate::MediaInfo,
                media: &std::path::Path,
            ) -> $crate::Result<()> {
                use $crate::markers::{MISavedTracks, MITITrackIDs};
                use $crate::{MuxConfigArg, ParseableArg};

                let tracks = mi.try_take::<MISavedTracks>(media)?;
                let add = mi.auto_flags.$add_marker;

                tracks
                    .values()
                    .flat_map(|tracks| tracks.iter())
                    .filter_map(|&track| {
                        let tids = $crate::immut!(mi, MITITrackIDs, media, track)?;

                        if let Some(val) = self.get(&tids[0]).or_else(|| self.get(&tids[1])) {
                            return Some(format!("{}:{}", track, val));
                        }

                        if !add {
                            return None;
                        }

                        if let Some(val) = mi.get_ti::<$crate::markers::$tic_marker>(media, track) {
                            return Some(format!("{}:{}", track, val));
                        }

                        None
                    })
                    .for_each(|val| {
                        args.push(MuxConfigArg::$arg.dashed().into());
                        args.push(val.into());
                    });

                mi.set::<MISavedTracks>(media, tracks);

                Ok(())
            }
        }
    };
}
