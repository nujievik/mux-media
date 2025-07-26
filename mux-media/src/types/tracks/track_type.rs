use enum_map::{Enum, EnumMap};
use strum_macros::EnumIter;

/// Type of media track.
#[derive(Copy, Clone, Debug, Default, PartialEq, Enum, EnumIter)]
pub enum TrackType {
    Video,
    Audio,
    Sub,
    Button,
    #[default]
    NonCustomizable,
}

impl TrackType {
    /// Returns an iterator over all variants of [`TrackType`].
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    /// Returns the string marker for [`TrackType`] used in mkvtoolnix tools.
    pub fn as_str_mkvtoolnix(self) -> &'static str {
        match self {
            Self::Video => "video",
            Self::Audio => "audio",
            Self::Sub => "subtitles",
            Self::Button => "buttons",
            Self::NonCustomizable => "",
        }
    }

    /// Returns a new [`EnumMap<TrackType, T>`] with default values.
    pub fn map<T>() -> EnumMap<Self, T>
    where
        T: Default,
    {
        EnumMap::default()
    }
}

impl From<matroska::Tracktype> for TrackType {
    fn from(mtt: matroska::Tracktype) -> Self {
        match mtt {
            matroska::Tracktype::Video => Self::Video,
            matroska::Tracktype::Audio => Self::Audio,
            matroska::Tracktype::Subtitle => Self::Sub,
            matroska::Tracktype::Buttons => Self::Button,
            _ => Self::NonCustomizable,
        }
    }
}
