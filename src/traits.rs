use crate::{MediaInfo, MuxError, TrackID};
use std::path::Path;

pub trait CLIArgs {
    type Arg: CLIArg;
}

pub trait CLIArg {
    fn as_long(self) -> &'static str;
    fn to_mkvmerge(self) -> Option<&'static str>;
}

pub trait ToMkvmergeArg {
    fn to_mkvmerge_arg(&self) -> String;
}

impl<T> ToMkvmergeArg for T
where
    T: ToString,
{
    fn to_mkvmerge_arg(&self) -> String {
        self.to_string()
    }
}

pub trait ToMkvmergeArgs {
    fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String>;
    fn to_os_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<std::ffi::OsString>;
}

pub trait GetField<F> {
    type FieldType;
    fn get(&self) -> &Self::FieldType;
}

pub trait GetOptField<F> {
    type FieldType;
    fn get(&self) -> Option<&Self::FieldType>;
}

pub trait SetGetPathField<F> {
    type FieldType;
    fn try_set(&mut self, path: &Path) -> Result<(), MuxError>;
    fn try_get(&mut self, path: &Path) -> Result<&Self::FieldType, MuxError>;
    fn get(&mut self, path: &Path) -> Option<&Self::FieldType>;
}

pub trait SetGetPathTrackField<F> {
    type FieldType;
    fn try_set(&mut self, path: &Path, tid: TrackID) -> Result<(), MuxError>;
    fn try_get(&mut self, path: &Path, tid: TrackID) -> Result<&Self::FieldType, MuxError>;
    fn get(&mut self, path: &Path, tid: TrackID) -> Option<&Self::FieldType>;
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
