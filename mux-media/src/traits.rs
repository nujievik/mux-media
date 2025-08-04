pub(crate) mod is_default;
pub(crate) mod lazy_fields;

use crate::{MediaInfo, MuxError};
use std::{ffi::OsString, path::Path};

/// Provides a fallible default initialization.
pub trait TryInit {
    fn try_init() -> Result<Self, MuxError>
    where
        Self: Sized;
}

/// Provides a delayed initialization for expensive operations.
pub trait TryFinalizeInit {
    /// Finalizes initialization.
    fn try_finalize_init(&mut self) -> Result<(), MuxError>;
}

/// Defines a type maximum value.
pub trait MaxValue {
    const MAX: Self;
}

/// Associates a type with its parseable arguments.
pub trait ParseableArgs {
    type Arg: ParseableArg;
}

/// Represents a parseable argument.
pub trait ParseableArg: Copy {
    /// Returns argument with leading dashes (e.g. `"--output"`).
    fn dashed(self) -> &'static str;

    /// Returns argument without leading dashes (e.g. `"output"`).
    fn undashed(self) -> &'static str;
}

/// Converts a [`MediaInfo`] value to ffmpeg-compatible arguments.
pub trait ToFfmpegArgs {
    /// Attempts append arguments to the given `args` vector.
    fn try_append_ffmpeg_args(args: &mut Vec<OsString>, mi: &mut MediaInfo)
    -> Result<(), MuxError>;

    /// Appends arguments to the given `args` vector.
    fn append_ffmpeg_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) {
        let _ = Self::try_append_ffmpeg_args(args, mi);
    }

    /// Returns arguments.
    fn to_ffmpeg_args(mi: &mut MediaInfo) -> Vec<OsString> {
        let mut args = Vec::new();
        Self::append_ffmpeg_args(&mut args, mi);
        args
    }

    /// Appends arguments to the given `args` for the given `media`, `track` and `out_stream` only.
    fn append_stream(
        _args: &mut Vec<OsString>,
        _mi: &mut MediaInfo,
        _media: &Path,
        _track: u64,
        _out_stream: usize,
    ) {
        eprintln!("Unimplemented ToFfmpegArgs::append_stream");
    }
}

/// Converts a value to JSON-compatible arguments.
pub trait ToJsonArgs {
    /// Appends arguments to the given `args` vector.
    fn append_json_args(&self, args: &mut Vec<String>);

    /// Returns arguments.
    fn to_json_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        self.append_json_args(&mut args);
        args
    }
}

/// Converts a value to mkvmerge-compatible arguments.
pub trait ToMkvmergeArgs {
    /// Attempts appends arguments to the given `args` vector.
    fn try_append_mkvmerge_args(
        &self,
        args: &mut Vec<OsString>,
        mi: &mut MediaInfo,
        media: &Path,
    ) -> Result<(), MuxError>;

    /// Appends arguments to the given `args` vector.
    fn append_mkvmerge_args(&self, args: &mut Vec<OsString>, mi: &mut MediaInfo, media: &Path) {
        let _ = self.try_append_mkvmerge_args(args, mi, media);
    }

    /// Returns arguments.
    fn to_mkvmerge_args(&self, mi: &mut MediaInfo, media: &Path) -> Vec<OsString> {
        let mut args = Vec::new();
        self.append_mkvmerge_args(&mut args, mi, media);
        args
    }
}

/// Associates a field with the marker type `F`.
pub trait Field<F> {
    type FieldType;

    /// Returns a reference to the field value.
    fn field(&self) -> &Self::FieldType;
}

// Doc hidden traits:

/// Converts a value to mkvmerge recognizable arg.
pub trait ToMkvmergeArg {
    fn to_mkvmerge_arg(&self) -> String;
}
