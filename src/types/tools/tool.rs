use enum_map::Enum;
use std::fmt;
use strum_macros::{AsRefStr, EnumIter, EnumString};

/// External binary tools used by crate.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, AsRefStr, Enum, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Tool {
    //Ffprobe,
    //Mkvextract,
    Mkvinfo,
    Mkvmerge,
}

impl Tool {
    /// Returns an iterator over all tools.
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    /// Returns an iterator over mkvtoolnix-related tools.
    pub fn iter_mkvtoolnix() -> impl Iterator<Item = Self> {
        Self::iter().filter(|tool| tool.is_mkvtoolnix())
    }

    /// Returns `true` if the tool belongs to the mkvtoolnix suite.
    pub(super) fn is_mkvtoolnix(self) -> bool {
        true
        //self != Self::Ffprobe
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
