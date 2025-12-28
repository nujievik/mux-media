//mod is_supported_copy;
mod new;

use crate::IsDefault;
use strum_macros::Display;

/// A supported output container.
#[derive(Copy, Clone, Debug, Default, Display, PartialEq, IsDefault)]
#[non_exhaustive]
pub enum Container {
    #[default]
    Matroska,
}

impl Container {
    /// Returns a main output extension for a container.
    #[inline]
    pub(crate) const fn as_ext(self) -> &'static str {
        match self {
            //Self::Avi => "avi",
            //Self::Mp4 => "mp4",
            Self::Matroska => "mkv",
            //Self::Webm => "webm",
        }
    }
}
