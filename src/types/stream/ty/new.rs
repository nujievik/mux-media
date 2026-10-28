use super::StreamType;
use crate::{MuxError, Result};
use std::str::FromStr;

impl FromStr for StreamType {
    type Err = MuxError;

    fn from_str(mut s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        let last_i = bytes.len() - 1;

        if bytes.len() > 1 && bytes[last_i] == b's' {
            s = unsafe { str::from_utf8_unchecked(&bytes[..last_i]) };
        }

        let ty = match s {
            "d" | "v" | "video" => Self::Video,
            "a" | "audio" => Self::Audio,
            "s" | "sub" | "subtitle" => Self::Subtitles,
            "f" | "font" | "attached-font" => Self::AttachedFont,
            "p" | "pic" | "picture" | "attached-pic" => Self::AttachedPic,
            "other" => Self::Other,
            "other-attach" | "attached-other" => Self::AttachedOther,
            _ => return Err(err!("unrecognized stream type")),
        };
        Ok(ty)
    }
}
