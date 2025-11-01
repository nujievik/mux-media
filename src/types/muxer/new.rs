use super::Muxer;
use crate::{EXTENSIONS, Msg, MuxLogger, Output};

impl From<&Output> for Muxer {
    fn from(out: &Output) -> Self {
        match out.ext.as_encoded_bytes() {
            ext if EXTENSIONS.avi.contains(ext) => Self::AVI,
            ext if EXTENSIONS.mp4.contains(ext) => Self::MP4,
            ext if EXTENSIONS.webm.contains(ext) => Self::Webm,
            ext if EXTENSIONS.matroska.contains(ext) => Self::Matroska,
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
