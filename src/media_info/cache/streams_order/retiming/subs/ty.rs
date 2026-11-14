use crate::media_info::MarkMediaInfoStreams;
use crate::{CodecId, Extension, MediaInfo, display};
use std::path::Path;

#[derive(Copy, Clone, Debug)]
pub enum SubType {
    Srt,
    Ssa,
    Vtt,
}

impl SubType {
    pub fn new_from_path(file: &Path) -> Option<SubType> {
        let ext = Extension::from_path(file)?;
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

    pub fn get_from_codec_id(mi: &MediaInfo, src: &Path, i_stream: usize) -> Option<SubType> {
        use crate::ffmpeg::codec::id::Id;

        let c = *mi
            .immut(MarkMediaInfoStreams, src)
            .map(|xs| &xs[i_stream].codec)
            .unwrap_or(&CodecId::default());

        let ty = match c.0 {
            Id::ASS | Id::SSA => SubType::Ssa,
            Id::SRT | Id::SUBRIP => SubType::Srt,
            Id::WEBVTT => SubType::Vtt,
            _ => return None,
        };
        Some(ty)
    }

    pub fn from_codec_id(mi: &MediaInfo, src: &Path, i_stream: usize) -> SubType {
        SubType::get_from_codec_id(mi, src, i_stream).unwrap_or_else(|| {
            log::warn!(
                "unsupported format of subs '{}' stream {}. Try retime as .srt",
                display(src),
                i_stream
            );
            SubType::Srt
        })
    }

    pub const fn as_ext(self) -> &'static str {
        match self {
            Self::Srt => "srt",
            Self::Ssa => "ass",
            Self::Vtt => "vtt",
        }
    }
}
