use crate::{MuxConfigArg, ParseableArg, ToMkvmergeArg};
use enum_map::Enum;
use strum_macros::EnumIter;

/// Flag type of media track.
#[derive(Copy, Clone, Debug, PartialEq, Enum, EnumIter)]
pub enum TFlagType {
    Default,
    Forced,
    Enabled,
}

impl TFlagType {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub(crate) fn iter_ffmpeg_supported() -> impl Iterator<Item = Self> {
        [Self::Default, Self::Forced].into_iter()
    }

    pub(crate) fn as_str_ffmpeg(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Forced => "forced",
            Self::Enabled => unreachable!("Unsupported ffmpeg flag: Enabled"),
        }
    }
}

impl ToMkvmergeArg for TFlagType {
    fn to_mkvmerge_arg(&self) -> String {
        match self {
            Self::Default => MuxConfigArg::DefaultTrackFlag,
            Self::Forced => MuxConfigArg::ForcedDisplayFlag,
            Self::Enabled => MuxConfigArg::TrackEnabledFlag,
        }
        .dashed()
        .to_owned()
    }
}
