mod is_supported_copy;
mod new;

use crate::IsDefault;
use strum_macros::Display;

/// A supported muxer.
#[derive(Copy, Clone, Debug, Default, Display, PartialEq, IsDefault)]
#[non_exhaustive]
pub enum Muxer {
    Avi,
    Mp4,
    #[default]
    Matroska,
    Webm,
}

impl Muxer {
    /// Returns a main output extension for a muxer.
    ///
    /// ```
    /// use mux_media::Muxer;
    /// assert_eq!(Muxer::Avi.as_ext(), "avi");
    /// assert_eq!(Muxer::Mp4.as_ext(), "mp4");
    /// assert_eq!(Muxer::Matroska.as_ext(), "mkv");
    /// assert_eq!(Muxer::Webm.as_ext(), "webm");
    /// ```
    #[inline]
    pub const fn as_ext(self) -> &'static str {
        match self {
            Self::Avi => "avi",
            Self::Mp4 => "mp4",
            Self::Matroska => "mkv",
            Self::Webm => "webm",
        }
    }
}
