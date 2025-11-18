use super::StreamType;
use crate::{MuxError, Result};
use std::str::FromStr;

impl FromStr for StreamType {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        let ty = match s.trim().to_ascii_lowercase().as_str() {
            "a" | "audio" => Self::Audio,
            "s" | "sub" | "subs" => Self::Sub,
            "d" | "v" | "video" => Self::Video,
            "f" | "font" | "fonts" => Self::Font,
            "m" | "attach" | "attachs" => Self::Attach,
            "other" | "others" => Self::Other,
            _ => return Err(err!("Unrecognized stream type: {}", s)),
        };
        Ok(ty)
    }
}
