use std::{
    borrow::Borrow,
    ffi::OsStr,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};

/// A wrapper around [`Arc<PathBuf>`].
#[derive(Debug, Ord, PartialOrd)]
pub struct ArcPathBuf(pub Arc<PathBuf>);

crate::deref_singleton_tuple_struct!(ArcPathBuf, Arc<PathBuf>);

impl AsRef<Path> for ArcPathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl AsRef<OsStr> for ArcPathBuf {
    fn as_ref(&self) -> &OsStr {
        self.0.as_path().as_os_str()
    }
}

impl Borrow<Path> for ArcPathBuf {
    fn borrow(&self) -> &Path {
        self.0.as_path()
    }
}

impl Clone for ArcPathBuf {
    /// Returns clone of `ArcPathBuf`.
    ///
    /// Internally `Arc` reference count is increased — no heap allocation occurs.
    fn clone(&self) -> Self {
        ArcPathBuf(Arc::clone(&self.0))
    }
}

impl<P> From<P> for ArcPathBuf
where
    P: Into<PathBuf>,
{
    fn from(p: P) -> ArcPathBuf {
        ArcPathBuf(Arc::new(p.into()))
    }
}

impl PartialEq for ArcPathBuf {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_path() == other.0.as_path()
    }
}
impl Eq for ArcPathBuf {}

impl Hash for ArcPathBuf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_path().hash(state)
    }
}
