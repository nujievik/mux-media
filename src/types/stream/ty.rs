mod is;
mod new;

use enum_map::{Enum, EnumMap};
use std::path::Path;
use strum_macros::{AsRefStr, EnumIter};

/// A type of stream.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, AsRefStr, Enum, EnumIter)]
#[non_exhaustive]
#[strum(serialize_all = "kebab-case")]
pub enum StreamType {
    Video,
    Audio,
    Sub,
    Other,
    Font,
    Attach,
}

impl StreamType {
    pub(crate) fn as_path(&self) -> &Path {
        Path::new(self.as_ref())
    }
}

impl StreamType {
    /// Returns a new [`EnumMap<StreamType, T>`] with default values.
    pub(crate) fn map<T>() -> EnumMap<Self, T>
    where
        T: Default,
    {
        EnumMap::default()
    }
}
