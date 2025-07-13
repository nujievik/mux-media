use enum_map::{Enum, EnumMap};
use strum_macros::EnumIter;

/// Type of media track.
#[derive(Clone, Copy, Default, PartialEq, Enum, EnumIter)]
pub enum TrackType {
    #[default]
    Audio,
    Sub,
    Video,
    Button,
}

impl TrackType {
    /// Returns an iterator over all variants of `TrackType`.
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    /// Returns the string marker for `TrackType` used in mkvtoolnix tools.
    pub fn as_str_mkvtoolnix(self) -> &'static str {
        match self {
            Self::Audio => "audio",
            Self::Sub => "subtitles",
            Self::Video => "video",
            Self::Button => "buttons",
        }
    }

    /// Returns a new `EnumMap<TrackType, T>` with default values.
    pub fn new_enum_map<T>() -> EnumMap<Self, T>
    where
        T: Default,
    {
        EnumMap::default()
    }
}
