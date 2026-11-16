pub(crate) mod iters;
mod new;
mod to_args;

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{ArcPathBuf, GlobSetPattern, RangeUsizeInclusive, Value};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigInput {
    pub(crate) fonts: Vec<PathBuf>,
    pub(crate) media: Value<InputMedia>,

    pub overwrite: bool,
    pub range: Option<RangeUsizeInclusive>,
    pub skip: Option<GlobSetPattern>,
    pub depth: u8,
    pub orphans_too: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum InputMedia {
    Dir(InputMediaDir),
    Files(Vec<ArcPathBuf>),
}
// TODO: move range, skip, depth, orphans_too from ConfigInput
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct InputMediaDir {
    pub(crate) dir: PathBuf,
    pub(crate) dir_media: Vec<ArcPathBuf>,
    pub(crate) other_media: Vec<ArcPathBuf>,
}

impl ConfigInput {
    pub fn dir(&self) -> &Path {
        match &*self.media {
            InputMedia::Dir(d) => &d.dir,
            InputMedia::Files(xs) => xs[0].parent().unwrap_or(Path::new(".")),
        }
    }
}
