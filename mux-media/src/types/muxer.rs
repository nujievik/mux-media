mod avi;
mod matroska;
//mod mp4;

use crate::{EXTENSIONS, Input, MediaInfo, MuxCurrent, Output, ToolOutput, Tools};
use std::path::{Path, PathBuf};

pub enum Muxer {
    AVI,
    //MP4,
    Matroska,
}

impl Muxer {
    #[inline(always)]
    pub(crate) fn mux_current(
        &self,
        input: &Input,
        tools: &Tools,
        mi: &mut MediaInfo,
        fonts: &mut Option<Vec<PathBuf>>,
        out: &Path,
    ) -> MuxCurrent<ToolOutput> {
        match self {
            Self::AVI => Self::mux_current_avi(tools, mi, out),
            Self::Matroska => Self::mux_current_matroska(input, tools, mi, fonts, out),
            //Self::MP4 => Self::mux_current_mp4(tools, mi, out),
        }
    }
}

impl From<&Output> for Muxer {
    fn from(out: &Output) -> Self {
        match out.ext().as_encoded_bytes() {
            ext if EXTENSIONS.avi.contains(ext) => Self::AVI,
            ext if EXTENSIONS.matroska.contains(ext) => Self::Matroska,
            //ext if EXTENSIONS.mp4.contains(ext) => Self::MP4,
            _ => {
                eprintln!("Unsupported container extension. Using Matroska container");
                Self::Matroska
            }
        }
    }
}
