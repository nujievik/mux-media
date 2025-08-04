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

    /// Returns a new [`EnumMap<TrackType, T>`] with default values.
    pub fn map<T>() -> EnumMap<Self, T>
    where
        T: Default,
    {
        EnumMap::default()
    }

    pub(crate) fn iter_customizable() -> impl Iterator<Item = Self> {
        [Self::Video, Self::Audio, Self::Sub].into_iter()
    }

    /// Returns char form of [`TrackType`].
    pub(crate) fn as_char(self) -> char {
        match self {
            Self::Video => 'v',
            Self::Audio => 'a',
            Self::Sub => 's',
            _ => unreachable!("Non-customizable flag"),
        }
    }

    /// Return mkvtoolnix-compatible form of [`TrackType`].
    pub(crate) fn as_str_mkvtoolnix(self) -> &'static str {
        match self {
            Self::Video => "video",
            Self::Audio => "audio",
            Self::Sub => "subtitles",
            _ => unreachable!("Non-customizable flag"),
        }
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
