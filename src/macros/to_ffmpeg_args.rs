#[doc(hidden)]
#[macro_export]
macro_rules! to_ffmpeg_args {
    (@names_or_langs, $ty:ty, $arg:ident, $auto:ident, $marker:ident) => {
        impl $crate::ToFfmpegArgs for $ty {
            fn try_append_ffmpeg_args(
                args: &mut Vec<std::ffi::OsString>,
                mi: &mut $crate::MediaInfo,
            ) -> $crate::Result<()> {
                use $crate::{MuxConfigArg, ParseableArg, markers::MICmnTrackOrder};

                let order = mi.try_take_cmn::<MICmnTrackOrder>()?;

                let auto = mi.auto_flags.$auto;
                let metadata = MuxConfigArg::Metadata.undashed();
                let mtd_marker = MuxConfigArg::$arg.undashed();

                order.iter().enumerate().for_each(|(i, m)| {
                    let val = $crate::unwrap_or_return!(
                        mi.get_ti::<$crate::markers::$marker>(&m.media, m.track)
                    );

                    if auto || val.is_user() {
                        args.push(format!("-{}:s:{}", metadata, i).into());
                        args.push(format!("{}={}", mtd_marker, val).into());
                    }
                });

                mi.set_cmn::<MICmnTrackOrder>(order);

                Ok(())
            }

            fn append_stream(
                args: &mut Vec<std::ffi::OsString>,
                mi: &mut $crate::MediaInfo,
                media: &std::path::Path,
                track: u64,
                out_stream: usize,
            ) {
                use $crate::{MuxConfigArg, ParseableArg};

                let auto = mi.auto_flags.$auto;
                let metadata = MuxConfigArg::Metadata.undashed();
                let mtd_marker = MuxConfigArg::$arg.undashed();

                let val =
                    $crate::unwrap_or_return!(mi.get_ti::<$crate::markers::$marker>(media, track));

                if auto || val.is_user() {
                    args.push(format!("-{}:s:{}", metadata, out_stream).into());
                    args.push(format!("{}={}", mtd_marker, val).into());
                }
            }
        }
    };
}
