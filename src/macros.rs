mod deref;
mod immut;
mod msg;
mod parseable_args;
mod to_ffmpeg_args;
mod to_json_args;
mod to_mkvmerge_args;

#[doc(hidden)]
#[macro_export]
macro_rules! mux_err {
    ( $($arg:tt)* ) => {
        $crate::MuxError::from(format!($($arg)*))
    };
}
