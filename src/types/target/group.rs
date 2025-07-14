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
    Buttons,
}

impl TargetGroup {
    /// Returns a [`Path`] representation of the target group name.
    ///
    /// Internally uses the kebab-case string form (e.g., `"audio"`, `"signs"`).
    pub fn as_path(&self) -> &Path {
        Path::new::<str>(self.as_ref())
    }

    /// Attempts to find and return [`TargetGroup::Signs`] if the given slice of strings
    /// contains any recognized variant of the "signs" keyword.
    ///
    /// Accepts both English `"signs"` and Russian `"надписи"` (case-insensitive).
    ///
    /// # Errors
    ///
    /// Returns a [`MuxError`] if no "signs" keyword is found.
    pub fn try_signs_from_slice_string(slice: &[String]) -> Result<Self, MuxError> {
        slice
            .iter()
            .find_map(|s| Self::try_signs_from_str(s).ok())
            .ok_or_else(|| "No found any signs key".into())
    }

    /// Attempts to parse a single string as [`TargetGroup::Signs`].
    ///
    /// Accepts both English `"signs"` and Russian `"надписи"` (case-insensitive).
    ///
    /// # Errors
    ///
    /// Returns a [`MuxError`] if the string does not match any known "signs" keyword.
    pub fn try_signs_from_str(s: &str) -> Result<Self, MuxError> {
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
        write!(f, "{}", AsRef::<str>::as_ref(self))
    }
}
