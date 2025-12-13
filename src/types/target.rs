use crate::{ArcPathBuf, Msg, Result, StreamType};
use std::{
    borrow::Borrow,
    ffi::OsStr,
    fs,
    hash::{Hash, Hasher},
    path::Path,
};

/// A target of [`ConfigTarget`](crate::ConfigTarget).
#[derive(Clone, Debug)]
pub enum Target {
    Global,
    Stream(StreamType),
    Path(ArcPathBuf),
}

impl Target {
    /// Parse [`Target`] from a os string.
    pub fn from_os_str<OS: AsRef<OsStr>>(os: OS) -> Result<Target> {
        let os = os.as_ref();

        if let Some(t) = os.to_str().and_then(|s| get_from_str(s)) {
            return Ok(t);
        }

        let path = fs::canonicalize(os)
            .map_err(|e| err!("Incorrect path target '{}': {}", Path::new(os).display(), e))?;

        return Ok(Self::Path(path.into()));

        fn get_from_str(s: &str) -> Option<Target> {
            let s = s.trim().to_ascii_lowercase();
            if matches!(s.as_str(), "g" | "global") {
                Some(Target::Global)
            } else if let Ok(ty) = s.parse::<StreamType>() {
                Some(Target::Stream(ty))
            } else {
                None
            }
        }
    }

    pub(crate) fn to_str(&self) -> Option<&str> {
        match self {
            Self::Global => Some("global"),
            Self::Stream(ty) => Some(ty.as_ref()),
            Self::Path(p) => p.to_str(),
        }
    }

    /// Returns a [`Path`] representation.
    pub(crate) fn as_path(&self) -> &Path {
        match self {
            Self::Global => Path::new("global"),
            Self::Stream(ty) => ty.as_path(),
            Self::Path(apb) => apb.as_path(),
        }
    }

    /// Prints the list of supported targets to stdout.
    pub(crate) fn print_list_targets() {
        println!("{}", Msg::ListTargets.as_str_localized());
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
