use crate::{CodecId, Extension, MediaInfo, markers::MIStreams};
use std::path::Path;

#[derive(Copy, Clone, Debug)]
pub enum SubType {
    Srt,
    Ssa,
    Vtt,
}

impl SubType {
    pub fn new_from_path(file: &Path) -> Option<SubType> {
        let ext = Extension::new_from_path(file)?;
        Self::new_from_extension(ext)
    }

    pub fn new_from_extension(ext: Extension) -> Option<SubType> {
        match ext {
            Extension::Ass | Extension::Ssa => Some(Self::Ssa),
            Extension::Srt => Some(Self::Srt),
            Extension::Vtt => Some(Self::Vtt),
            _ => None,
        }
    }

    pub fn from_codec_id(mi: &MediaInfo, src: &Path, i_stream: usize) -> SubType {
        use crate::ffmpeg::codec::id::Id;

        let c = *mi
            .immut(MIStreams, src)
            .map(|xs| &xs[i_stream].codec)
            .unwrap_or(&CodecId::default());

        match c.0 {
            Id::ASS | Id::SSA => SubType::Ssa,
            Id::SRT | Id::SUBRIP => SubType::Srt,
            Id::WEBVTT => SubType::Vtt,
            _ => {
                log::warn!(
                    "Unsupported codec {:?} of sub stream {}. Try retime as .srt",
                    c,
                    i_stream
                );
                SubType::Srt
            }
        }
    }

    pub const fn as_ext(self) -> &'static str {
        match self {
            Self::Srt => "srt",
            Self::Ssa => "ass",
            Self::Vtt => "vtt",
        }
    }
}
