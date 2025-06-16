use enum_map::{Enum, EnumMap, enum_map};
use strum_macros::EnumIter;

#[derive(Clone, Copy, Default, PartialEq, Enum, EnumIter)]
pub enum TrackType {
    #[default]
    Audio,
    Sub,
    Video,
    Button,
}

impl TrackType {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub fn as_str_mkvtoolnix(self) -> &'static str {
        match self {
            Self::Audio => "audio",
            Self::Sub => "subtitles",
            Self::Video => "video",
            Self::Button => "buttons",
        }
    }

    pub fn new_enum_map<T>() -> EnumMap<Self, T>
    where
        T: Default,
    {
        enum_map! { _ => T::default() }
    }
}
