use crate::{MediaInfo, MuxError};
use std::{ffi::OsString, path::Path};

/// Associates a type with its corresponding CLI arguments.
pub trait CLIArgs {
    type Arg: CLIArg;
}

/// Enum representing long-form CLI argument names.
pub trait CLIArg {
    /// Returns the argument name without leading dashes (e.g. `"output"`).
    fn as_long(self) -> &'static str;
}

/// Converts a value to json args.
pub trait ToJsonArgs {
    fn to_json_args(&self) -> Vec<String>;
}

/// Defines a constant `MKVMERGE_ARG` containing the mkvmerge recognizable arg.
pub trait MkvmergeArg {
    const MKVMERGE_ARG: &'static str;
}

/// Defines a constant `MKVMERGE_NO_ARG` containing the mkvmerge recognizable arg.
pub trait MkvmergeNoArg {
    const MKVMERGE_NO_ARG: &'static str;
}

/// Converts a value to mkvmerge recognizable arg.
pub trait ToMkvmergeArg {
    fn to_mkvmerge_arg(&self) -> String;
}

/// Converts a value to mkvmerge recognizable args.
pub trait ToMkvmergeArgs {
    fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String>;
    fn to_os_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<OsString>;
}

/// Gets a value reference from a field of `Self`.
pub trait GetField<F> {
    type FieldType;
    fn get(&self) -> &Self::FieldType;
}

/// Gets an `Option` value reference from a field of `Self`.
pub trait GetOptField<F> {
    type FieldType;
    fn get(&self) -> Option<&Self::FieldType>;
}

/// Sets value and gets a value reference from a field of `Self`.
pub trait SetGetField<F> {
    type FieldType;
    fn try_set(&mut self) -> Result<(), MuxError>;
    fn try_get(&mut self) -> Result<&Self::FieldType, MuxError>;
    fn get(&mut self) -> Option<&Self::FieldType>;
    fn unmut_get(&self) -> Option<&Self::FieldType>;
}

/// Sets value and gets a value reference from a field of `Self` identified by `Path`.
pub trait SetGetPathField<F> {
    type FieldType;
    fn try_set(&mut self, path: &Path) -> Result<(), MuxError>;
    fn try_get(&mut self, path: &Path) -> Result<&Self::FieldType, MuxError>;
    fn get(&mut self, path: &Path) -> Option<&Self::FieldType>;
    fn unmut_get(&self, path: &Path) -> Option<&Self::FieldType>;
}

/// Sets value and gets a value reference from a field of `Self` identified by `Path` and `num`.
pub trait SetGetPathTrackField<F> {
    type FieldType;
    fn try_set(&mut self, path: &Path, num: u64) -> Result<(), MuxError>;
    fn try_get(&mut self, path: &Path, num: u64) -> Result<&Self::FieldType, MuxError>;
    fn get(&mut self, path: &Path, num: u64) -> Option<&Self::FieldType>;
}

/// Provides a fallible default initialization.
pub trait TryInit {
    fn try_init() -> Result<Self, MuxError>
    where
        Self: Sized;
}

/// Performs delayed initialization for expensive operations.
pub trait TryFinalizeInit {
    fn try_finalize_init(&mut self) -> Result<(), MuxError>;
}

/// Defines a constant `MAX` containing the maximum value of `Self`.
pub trait MaxValue {
    const MAX: Self;
}

/// Checks whether a value is equal to its type's default.
pub trait IsDefault {
    /// Returns `true` if `self` is equal to the default value for its type.
    fn is_default(&self) -> bool;
}

impl<T> IsDefault for T
where
    T: Default + PartialEq,
{
    fn is_default(&self) -> bool {
        self == &T::default()
    }
}
