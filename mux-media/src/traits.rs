pub(crate) mod is_default;

use crate::{MediaInfo, MuxError};
use std::{ffi::OsString, path::Path};

/// Associates a type with its parseable arguments.
pub trait ParseableArgs {
    type Arg: ParseableArg;
}

/// Represents a parseable argument.
pub trait ParseableArg {
    /// Returns argument with leading dashes (e.g. `"--output"`).
    fn dashed(self) -> &'static str;

    /// Returns argument without leading dashes (e.g. `"output"`).
    fn undashed(self) -> &'static str;
}

/// Converts a value to JSON args.
pub trait ToJsonArgs {
    fn to_json_args(&self) -> Vec<String>;
}

/// Converts a value to mkvmerge recognizable args.
pub trait ToMkvmergeArgs {
    fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String>;
    fn to_os_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<OsString>;
}

/// Associates a field with the marker type `F`.
pub trait Field<F> {
    type FieldType;

    /// Returns the associated field value.
    fn field(&self) -> &Self::FieldType;
}

/// Associates a mutable field with the marker type `F`.
pub trait MutField<F> {
    type FieldType;

    fn try_set(&mut self) -> Result<(), MuxError>;
    fn try_get(&mut self) -> Result<&Self::FieldType, MuxError>;
    fn get(&mut self) -> Option<&Self::FieldType>;
    fn unmut(&self) -> Option<&Self::FieldType>;
}

/// Associates a mutable field with the marker type `F` and [`Path`].
pub trait MutPathField<F> {
    type FieldType;

    fn try_set(&mut self, path: &Path) -> Result<(), MuxError>;
    fn try_get(&mut self, path: &Path) -> Result<&Self::FieldType, MuxError>;
    fn get(&mut self, path: &Path) -> Option<&Self::FieldType>;
    fn unmut(&self, path: &Path) -> Option<&Self::FieldType>;
}

/// Associates a mutable field with the marker type `F`, [`Path`] and `num`.
pub trait MutPathNumField<F> {
    type FieldType;

    fn try_set(&mut self, path: &Path, num: u64) -> Result<(), MuxError>;
    fn try_get(&mut self, path: &Path, num: u64) -> Result<&Self::FieldType, MuxError>;
    fn get(&mut self, path: &Path, num: u64) -> Option<&Self::FieldType>;
    fn unmut(&self, path: &Path, num: u64) -> Option<&Self::FieldType>;
}

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

// Doc hidden traits:

/// Defines a mkvmerge recognizable arg.
pub trait MkvmergeArg {
    const MKVMERGE_ARG: &'static str;
}

/// Defines a mkvmerge recognizable negative arg.
pub trait MkvmergeNoArg {
    const MKVMERGE_NO_ARG: &'static str;
}

/// Converts a value to mkvmerge recognizable arg.
pub trait ToMkvmergeArg {
    fn to_mkvmerge_arg(&self) -> String;
}
