use crate::{MediaInfo, MuxError};
use std::{ffi::OsString, path::Path};

pub trait CLIArgs {
    type Arg: CLIArg;
}

pub trait CLIArg {
    fn as_long(self) -> &'static str;
}

pub trait ToJsonArgs {
    fn to_json_args(&self) -> Vec<String>;
}

pub trait MkvmergeArg {
    const MKVMERGE_ARG: &'static str;
}

pub trait MkvmergeNoArg {
    const MKVMERGE_NO_ARG: &'static str;
}

pub trait ToMkvmergeArg {
    fn to_mkvmerge_arg(&self) -> String;
}

pub trait ToMkvmergeArgs {
    fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String>;
    fn to_os_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<OsString>;
}

pub trait GetField<F> {
    type FieldType;
    fn get(&self) -> &Self::FieldType;
}

pub trait GetOptField<F> {
    type FieldType;
    fn get(&self) -> Option<&Self::FieldType>;
}

pub trait SetGetField<F> {
    type FieldType;
    fn try_set(&mut self) -> Result<(), MuxError>;
    fn try_get(&mut self) -> Result<&Self::FieldType, MuxError>;
    fn get(&mut self) -> Option<&Self::FieldType>;
    fn unmut_get(&self) -> Option<&Self::FieldType>;
}

pub trait SetGetPathField<F> {
    type FieldType;
    fn try_set(&mut self, path: &Path) -> Result<(), MuxError>;
    fn try_get(&mut self, path: &Path) -> Result<&Self::FieldType, MuxError>;
    fn get(&mut self, path: &Path) -> Option<&Self::FieldType>;
    fn unmut_get(&self, path: &Path) -> Option<&Self::FieldType>;
}

pub trait SetGetPathTrackField<F> {
    type FieldType;
    fn try_set(&mut self, path: &Path, num: u64) -> Result<(), MuxError>;
    fn try_get(&mut self, path: &Path, num: u64) -> Result<&Self::FieldType, MuxError>;
    fn get(&mut self, path: &Path, num: u64) -> Option<&Self::FieldType>;
}

pub trait TryInit {
    fn try_init() -> Result<Self, MuxError>
    where
        Self: Sized;
}

// Delayed expensive init ops
pub trait TryFinalizeInit {
    fn try_finalize_init(&mut self) -> Result<(), MuxError>;
}

pub trait MaxValue {
    const MAX: Self;
}

pub trait IsDefault {
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
