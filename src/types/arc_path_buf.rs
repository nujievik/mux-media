use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};

/// A wrapper for `Arc<PathBuf>`.
#[derive(Debug, Ord, PartialOrd)]
pub struct ArcPathBuf(Arc<PathBuf>);

crate::deref_singleton_tuple_fields!(ArcPathBuf, Arc<PathBuf>);

impl AsRef<Path> for ArcPathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
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
        ArcPathBuf(self.0.clone())
    }
}

impl From<&Path> for ArcPathBuf {
    fn from(path: &Path) -> Self {
        ArcPathBuf(Arc::new(path.to_path_buf()))
    }
}

impl From<PathBuf> for ArcPathBuf {
    fn from(pb: PathBuf) -> Self {
        ArcPathBuf(Arc::new(pb))
    }
}

impl From<&PathBuf> for ArcPathBuf {
    fn from(path: &PathBuf) -> Self {
        ArcPathBuf(Arc::new(path.to_path_buf()))
    }
}

impl From<Arc<PathBuf>> for ArcPathBuf {
    fn from(arc: Arc<PathBuf>) -> Self {
        Self(arc)
    }
}

impl From<ArcPathBuf> for Arc<PathBuf> {
    fn from(pa: ArcPathBuf) -> Self {
        pa.0
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
