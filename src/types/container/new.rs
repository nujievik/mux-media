use super::Container;
use crate::{Extension, Msg, MuxLogger, Output};

impl Container {
    pub fn new(output: &Output) -> Container {
        match Extension::new(output.ext.as_encoded_bytes()) {
            //Some(Extension::Avi) => Self::Avi,
            //Some(Extension::Mp4) => Self::Mp4,
            //Some(Extension::Webm) => Self::Webm,
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
