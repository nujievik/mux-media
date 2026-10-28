mod is;
mod new;

use enum_map::{Enum, EnumMap};
use std::path::Path;
use strum_macros::{AsRefStr, EnumIter};

/// A type of stream.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, AsRefStr, Enum, EnumIter)]
#[strum(serialize_all = "kebab-case")]
#[non_exhaustive]
pub enum StreamType {
    Video,
    Audio,
    Subtitles,
    Other,
    /// Attached TTF or OTF font.
    AttachedFont,
    AttachedPic,
    AttachedOther,
}

impl StreamType {
    /// Returns string representation.
    ///
    /// ```
    /// use mux_media::StreamType;
    /// use strum::IntoEnumIterator;
    ///
    /// assert_eq!("video", StreamType::Video.as_str());
    ///
    /// for ty in StreamType::iter() {
    ///     assert_eq!(ty, ty.as_str().parse::<StreamType>().unwrap());
    /// }
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    #[inline]
    pub fn as_path(&self) -> &Path {
        Path::new(self.as_str())
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
