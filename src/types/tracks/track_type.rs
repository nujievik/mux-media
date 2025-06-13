use strum_macros::EnumIter;

#[derive(Clone, Copy, Default, PartialEq, EnumIter)]
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
}
