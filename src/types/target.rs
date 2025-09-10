pub(crate) mod group;

use crate::{ArcPathBuf, Msg, MuxError, TargetGroup, mux_err};
use std::{
    borrow::Borrow,
    ffi::{OsStr, OsString},
    fs,
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
        println!("{}", Msg::ListTargets.as_str_localized());
    }
}

impl From<PathBuf> for Target {
    fn from(pb: PathBuf) -> Self {
        Self::Path(pb.into())
    }
}

impl TryFrom<&OsStr> for Target {
    type Error = MuxError;

    fn try_from(oss: &OsStr) -> Result<Self, Self::Error> {
        if let Some(group) = oss.to_str().and_then(|s| s.parse::<TargetGroup>().ok()) {
            return Ok(Target::Group(group));
        }

        let path = fs::canonicalize(oss).map_err(|e| {
            mux_err!(
                "Incorrect path target '{}': {}",
                Path::new(oss).display(),
                e
            )
        })?;

        Ok(path.into())
    }
}

impl TryFrom<&OsString> for Target {
    type Error = MuxError;

    #[inline(always)]
    fn try_from(oss: &OsString) -> Result<Self, Self::Error> {
        Self::try_from(oss.as_os_str())
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
