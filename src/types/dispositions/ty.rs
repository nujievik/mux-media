use crate::ffmpeg::format::stream;
use enum_map::Enum;
use strum_macros::{AsRefStr, EnumIter};

/// A type of disposition.
#[derive(Copy, Clone, Debug, PartialEq, AsRefStr, Enum, EnumIter)]
#[non_exhaustive]
#[strum(serialize_all = "kebab-case")]
pub enum DispositionType {
    Default,
    Forced,
}

impl DispositionType {
    pub(crate) fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub(crate) fn bits(self) -> i32 {
        match self {
            DispositionType::Default => stream::Disposition::DEFAULT.bits(),
            DispositionType::Forced => stream::Disposition::FORCED.bits(),
        }
    }
}
