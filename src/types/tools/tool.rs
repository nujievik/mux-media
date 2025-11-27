use crate::CliArg;
use enum_map::Enum;
use std::fmt;
use strum_macros::{AsRefStr, EnumIter, EnumString};

/// External binary tools used by crate.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, AsRefStr, Enum, EnumIter, EnumString)]
#[non_exhaustive]
#[strum(serialize_all = "kebab-case")]
pub enum Tool {
    Ffmpeg,
}

impl Tool {
    /// Returns an iterator over all tools.
    /// ```
    /// # use mux_media::Tool;
    /// #
    /// let mut it = Tool::iter();
    /// assert_eq!(it.next(), Some(Tool::Ffmpeg));
    /// assert_eq!(it.next(), None);
    /// ```
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub(crate) fn as_cli_arg(self) -> CliArg {
        match self {
            Self::Ffmpeg => CliArg::Ffmpeg,
        }
    }

    /// Returns `true` if the tool belongs to the mkvtoolnix suite.
    pub(super) fn is_mkvtoolnix(self) -> bool {
        false
    }

    /// Returns the associated package name (`"mkvtoolnix"` or `"ffmpeg"`).
    pub(super) fn as_str_package(self) -> &'static str {
        if self.is_mkvtoolnix() {
            "mkvtoolnix"
        } else {
            "ffmpeg"
        }
    }
}

impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
