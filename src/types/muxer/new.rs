use super::Muxer;
use crate::{Extension, Msg, MuxLogger, Output};

impl Muxer {
    pub fn new(output: &Output) -> Muxer {
        match Extension::new(output.ext.as_encoded_bytes()) {
            Some(Extension::Avi) => Self::Avi,
            Some(Extension::Mp4) => Self::Mp4,
            Some(Extension::Webm) => Self::Webm,
            Some(Extension::Mkv) => Self::Matroska,
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
