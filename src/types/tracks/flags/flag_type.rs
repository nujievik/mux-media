use crate::ToMkvmergeArg;
use enum_map::Enum;
use strum_macros::EnumIter;

#[derive(Copy, Clone, PartialEq, Enum, EnumIter)]
pub enum TFlagType {
    Default,
    Forced,
    Enabled,
}

impl TFlagType {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}

impl ToMkvmergeArg for TFlagType {
    fn to_mkvmerge_arg(&self) -> String {
        let s = match self {
            Self::Default => "--default-track-flag",
            Self::Forced => "--forced-display-flag",
            Self::Enabled => "--track-enabled-flag",
        };
        s.to_string()
    }
}
