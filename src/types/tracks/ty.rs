use crate::ffmpeg::media::Type as FfmpegMediaType;
use enum_map::{Enum, EnumMap};
use strum_macros::EnumIter;

/// Type of media track.
#[derive(Copy, Clone, Debug, Default, PartialEq, Enum, EnumIter)]
pub enum TrackType {
    Audio,
    Sub,
    Video,
    #[default]
    NonCustomizable,
}

impl TrackType {
    /// Returns an iterator over all variants of [`TrackType`].
    /// ```
    /// # use mux_media::TrackType;
    /// #
    /// let mut it = TrackType::iter();
    /// assert_eq!(it.next(), Some(TrackType::Audio));
    /// assert_eq!(it.next(), Some(TrackType::Sub));
    /// assert_eq!(it.next(), Some(TrackType::Video));
    /// assert_eq!(it.next(), Some(TrackType::NonCustomizable));
    /// assert_eq!(it.next(), None);
    /// ```
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    /// Returns a new [`EnumMap<TrackType, T>`] with default values.
    pub(crate) fn map<T>() -> EnumMap<Self, T>
    where
        T: Default,
    {
        EnumMap::default()
    }

    pub(crate) fn iter_customizable() -> impl Iterator<Item = Self> {
        [Self::Video, Self::Audio, Self::Sub].into_iter()
    }

    pub(crate) fn is_sub(self) -> bool {
        matches!(self, TrackType::Sub)
    }

    pub(crate) fn is_customizable(self) -> bool {
        matches!(self, TrackType::Audio | TrackType::Sub | TrackType::Video)
    }

    /// Returns first string symbol of [`TrackType`].
    pub(crate) fn as_first_s(self) -> &'static str {
        match self {
            Self::Video => "v",
            Self::Audio => "a",
            Self::Sub => "s",
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

    pub(crate) fn as_ffmpeg_mty(self) -> FfmpegMediaType {
        match self {
            Self::Video => FfmpegMediaType::Video,
            Self::Audio => FfmpegMediaType::Audio,
            Self::Sub => FfmpegMediaType::Subtitle,
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
            _ => Self::NonCustomizable,
        }
    }
}
