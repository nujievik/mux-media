use crate::{DefaultTFlags, EnabledTFlags, ForcedTFlags, MkvmergeArg, ToMkvmergeArg};
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
            Self::Default => DefaultTFlags::MKVMERGE_ARG,
            Self::Forced => ForcedTFlags::MKVMERGE_ARG,
            Self::Enabled => EnabledTFlags::MKVMERGE_ARG,
        };
        s.to_string()
    }
}
