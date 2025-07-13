use crate::{Msg, MuxError, TrackType};
use std::{fmt, path::PathBuf, str::FromStr};
use strum_macros::AsRefStr;

/// Target group of mux settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum TargetGroup {
    Audio,
    Video,
    Signs,
    Subs,
    Buttons,
}

/// Target of mux settings.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Target {
    Group(TargetGroup),
    Path(PathBuf),
}

impl Target {
    /// Prints the list of supported targets to stdout.
    pub fn print_list_targets() {
        println!("{}", Msg::ListTargets.to_str_localized());
    }
}

impl TargetGroup {
    /// Returns `Ok(Self::Signs)` if slice contains signs key; otherwise, returns error.
    pub fn try_signs_from_slice_string(slice: &[String]) -> Result<Self, MuxError> {
        slice
            .iter()
            .find_map(|s| Self::try_signs_from_str(s).ok())
            .ok_or_else(|| "No found any signs key".into())
    }

    /// Returns `Ok(Self::Signs)` if string is a signs key; otherwise, returns error.
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

impl FromStr for TargetGroup {
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

impl fmt::Display for TargetGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
