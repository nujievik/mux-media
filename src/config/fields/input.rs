pub(crate) mod iters;
mod new;
mod to_args;

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{ArcPathBuf, GlobSetPattern, RangeUsize};
use enum_map::{Enum, EnumMap};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigInput {
    // must ensures that files is unempty.
    pub(crate) ty: InputType,
    pub range: Option<RangeUsize>,
    pub skip: Option<GlobSetPattern>,
    pub depth: u8,
    pub solo: bool,
    pub(crate) file_dirs: EnumMap<InputFileType, Vec<ArcPathBuf>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputType {
    Dir(PathBuf),
    Files(Vec<PathBuf>),
}

/// A type of input file.
#[derive(Copy, Clone, Debug, Enum)]
#[non_exhaustive]
pub enum InputFileType {
    Font,
    Media,
}

impl ConfigInput {
    pub fn dir(&self) -> &Path {
        match &self.ty {
            InputType::Dir(dir) => dir,
            InputType::Files(xs) => &xs[0].parent().unwrap_or(Path::new(".")),
        }
    }
}
