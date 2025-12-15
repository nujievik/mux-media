use super::Muxer;
use crate::{Extension, Msg, MuxLogger, Output};

impl Muxer {
    pub fn new(output: &Output) -> Muxer {
        match Extension::new(output.ext.as_encoded_bytes()) {
            Some(Extension::Avi) => Self::AVI,
            Some(Extension::Mp4) => Self::MP4,
            Some(Extension::Webm) => Self::Webm,
            Some(ext) if ext.is_matroska() => Self::Matroska,
            _ => {
                eprintln!(
                    "{}{}. {} Matroska (.mkv)",
                    MuxLogger::color_prefix(log::Level::Warn),
                    Msg::UnsupOutContainerExt,
                    Msg::Using,
                );
                Self::Matroska
            }
        }
    }
}
