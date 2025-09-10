use crate::{MuxError, Result, TrackType};
use std::{fmt, hash::Hash, path::Path, str::FromStr};
use strum_macros::AsRefStr;

/// Target group of mux settings.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum TargetGroup {
    Global,
    Audio,
    Subs,
    Video,
}

impl TargetGroup {
    /// Returns a [`Path`] representation of the target group name.
    ///
    /// Internally uses the kebab-case string form (e.g., `"audio"`, `"signs"`).
    pub fn as_path(&self) -> &Path {
        Path::new::<str>(self.as_ref())
    }
}

impl AsRef<Path> for TargetGroup {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl TryFrom<TrackType> for TargetGroup {
    type Error = MuxError;

    /// Tries convert `TrackType` into `TargetGroup`.
    ///
    /// # Errors
    ///
    /// Returns an error if `TrackType` not Audio, Sub or Video.
    ///
    /// # Examples
    /// ```
    /// # use mux_media::{TargetGroup, TrackType};
    /// assert_eq!(TargetGroup::try_from(TrackType::Audio).unwrap(), TargetGroup::Audio);
    /// assert_eq!(TargetGroup::try_from(TrackType::Sub).unwrap(), TargetGroup::Subs);
    /// assert_eq!(TargetGroup::try_from(TrackType::Video).unwrap(), TargetGroup::Video);
    /// TargetGroup::try_from(TrackType::NonCustomizable).unwrap_err();
    /// ```
    fn try_from(ty: TrackType) -> Result<Self> {
        let group = match ty {
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

    /// Parses a string s to return a value of this type.
    /// ```
    /// # use mux_media::TargetGroup;
    /// # use std::str::FromStr;
    /// #
    /// assert_eq!(TargetGroup::from_str("g").unwrap(), TargetGroup::Global);
    /// assert_eq!(TargetGroup::from_str("global").unwrap(), TargetGroup::Global);
    /// assert_eq!(TargetGroup::from_str("a").unwrap(), TargetGroup::Audio);
    /// assert_eq!(TargetGroup::from_str("audio").unwrap(), TargetGroup::Audio);
    /// assert_eq!(TargetGroup::from_str("v").unwrap(), TargetGroup::Video);
    /// assert_eq!(TargetGroup::from_str("video").unwrap(), TargetGroup::Video);
    /// assert_eq!(TargetGroup::from_str("s").unwrap(), TargetGroup::Subs);
    /// assert_eq!(TargetGroup::from_str("subs").unwrap(), TargetGroup::Subs);
    /// assert_eq!(TargetGroup::from_str("subtitles").unwrap(), TargetGroup::Subs);
    /// TargetGroup::from_str("missing").unwrap_err();
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s.trim() {
            "g" => Self::Global,
            "global" => Self::Global,
            "a" => Self::Audio,
            "audio" => Self::Audio,
            "v" => Self::Video,
            "video" => Self::Video,
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
