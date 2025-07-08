use enum_map::Enum;
use std::fmt;
use strum_macros::{AsRefStr, EnumIter, EnumString};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, AsRefStr, Enum, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Tool {
    Ffprobe,
    Mkvextract,
    Mkvinfo,
    Mkvmerge,
}

impl Tool {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub fn iter_mkvtoolnix() -> impl Iterator<Item = Self> {
        Self::iter().filter(|tool| tool.is_mkvtoolnix())
    }

    pub(super) fn is_mkvtoolnix(self) -> bool {
        self != Self::Ffprobe
    }

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
