pub(crate) mod group;

use crate::{ArcPathBuf, Msg, TargetGroup};
use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

/// Target of mux settings.
#[derive(Clone, Debug)]
pub enum Target {
    Group(TargetGroup),
    Path(ArcPathBuf),
}

impl Target {
    /// Returns a [`Path`] representation.
    pub fn as_path(&self) -> &Path {
        match self {
            Self::Group(g) => g.as_path(),
            Self::Path(apb) => apb.as_path(),
        }
    }

    /// Prints the list of supported targets to stdout.
    pub fn print_list_targets() {
        println!("{}", Msg::ListTargets.to_str_localized());
    }
}

impl AsRef<Path> for Target {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl Borrow<Path> for Target {
    fn borrow(&self) -> &Path {
        self.as_path()
    }
}

impl From<PathBuf> for Target {
    fn from(pb: PathBuf) -> Self {
        Self::Path(pb.into())
    }
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        self.as_path() == other.as_path()
    }
}
impl Eq for Target {}

impl Hash for Target {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_path().hash(state)
    }
}
