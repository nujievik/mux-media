mod is;
mod new;

use enum_map::{Enum, EnumMap};
use std::path::Path;
use strum_macros::{AsRefStr, EnumIter};

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

    pub(crate) fn as_first_s(&self) -> &str {
        match self {
            Self::Audio => "a",
            Self::Sub => "s",
            Self::Video => "v",
            _ => unreachable!(),
        }
    }

    pub(crate) fn iter_track() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter().filter(|ty| ty.is_track())
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
