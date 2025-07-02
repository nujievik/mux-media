use crate::{Msg, MuxError, TrackType};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TargetGroup {
    Audio,
    Video,
    Signs,
    Subs,
    Buttons,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Target {
    Group(TargetGroup),
    Path(PathBuf),
}

impl Target {
    pub fn print_list_targets() {
        println!("{}", Msg::ListTargets.to_str_localized());
    }
}

impl TargetGroup {
    pub fn try_signs_from_slice_string(slice: &[String]) -> Result<Self, MuxError> {
        slice
            .iter()
            .find_map(|s| Self::try_signs_from_str(s).ok())
            .ok_or_else(|| "No found any signs key".into())
    }

    pub fn try_signs_from_str(s: &str) -> Result<Self, MuxError> {
        match s.to_lowercase().as_ref() {
            "signs" => Ok(Self::Signs),
            "надписи" => Ok(Self::Signs),
            _ => Err("Is not signs key".into()),
        }
    }
}

impl From<TrackType> for TargetGroup {
    fn from(tt: TrackType) -> Self {
        match tt {
            TrackType::Audio => Self::Audio,
            TrackType::Sub => Self::Subs,
            TrackType::Video => Self::Video,
            TrackType::Button => Self::Buttons,
        }
    }
}

impl std::str::FromStr for TargetGroup {
    type Err = crate::MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "a" => Self::Audio,
            "audio" => Self::Audio,
            "v" => Self::Video,
            "video" => Self::Video,
            "signs" => Self::Signs,
            "s" => Self::Subs,
            "subs" => Self::Subs,
            "subtitles" => Self::Subs,
            "buttons" => Self::Buttons,
            _ => return Err(format!("Unrecognized target group: '{}'", s).into()),
        })
    }
}
