use crate::{MuxError, TrackType};
use std::{fmt, hash::Hash, path::Path, str::FromStr};
use strum_macros::AsRefStr;

/// Target group of mux settings.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum TargetGroup {
    Audio,
    Video,
    Signs,
    Subs,
}

impl TargetGroup {
    /// Returns a [`Path`] representation of the target group name.
    ///
    /// Internally uses the kebab-case string form (e.g., `"audio"`, `"signs"`).
    pub fn as_path(&self) -> &Path {
        Path::new::<str>(self.as_ref())
    }

    /// Attempts to parse any full string as [`TargetGroup::Signs`] in the given strings.
    ///
    /// Recognizes English `"signs"` and Russian `"надписи"` (case-insensitive).
    pub fn try_signs_many(slice: &[String]) -> Result<Self, MuxError> {
        slice
            .iter()
            .find_map(|s| Self::try_signs(s).ok())
            .ok_or_else(|| "No found any signs key".into())
    }

    /// Attempts to parse a full string as [`TargetGroup::Signs`].
    ///
    /// Recognizes English `"signs"` and Russian `"надписи"` (case-insensitive).
    pub fn try_signs(s: &str) -> Result<Self, MuxError> {
        match s.to_lowercase().as_ref() {
            "signs" => Ok(Self::Signs),
            "надписи" => Ok(Self::Signs),
            _ => Err("Is not signs key".into()),
        }
    }
}

impl AsRef<Path> for TargetGroup {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl TryFrom<TrackType> for TargetGroup {
    type Error = MuxError;

    fn try_from(tt: TrackType) -> Result<Self, Self::Error> {
        let group = match tt {
            TrackType::Audio => Self::Audio,
            TrackType::Sub => Self::Subs,
            TrackType::Video => Self::Video,
            _ => return Err("Unsupported track type".into()),
        };
        Ok(group)
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
            _ => return Err(format!("Unrecognized target group: '{}'", s).into()),
        })
    }
}

impl fmt::Display for TargetGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", AsRef::<str>::as_ref(self))
    }
}
