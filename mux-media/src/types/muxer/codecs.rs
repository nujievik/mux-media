use crate::Muxer;
use phf::{Set, phf_set};

/// A static sets of supported codecs by muxer.
pub struct MuxerCodecs {
    pub avi: &'static Set<&'static str>,
    pub mp4: &'static Set<&'static str>,
    pub webm: &'static Set<&'static str>,
}

/// A static collection of supported codecs by muxer.
pub static MUXER_CODECS: MuxerCodecs = MuxerCodecs {
    avi: &AVI,
    mp4: &MP4,
    webm: &WEBM,
};

impl MuxerCodecs {
    pub(crate) fn is_supported(&self, muxer: Muxer, s: &str) -> bool {
        match muxer {
            Muxer::AVI => self.avi.contains(s),
            Muxer::MP4 => self.mp4.contains(s),
            Muxer::Matroska => true,
            Muxer::Webm => self.webm.contains(s),
        }
    }
}

pub static AVI: Set<&str> = phf_set! {
    "MP3",
    "MPEG-4p2",
};

pub static MP4: Set<&str> = phf_set! {
    "AAC",
    "AC-3",
    "AVC/H.264/MPEG-4p10",
    "A_AC3",
    "A_VORBIS",
    "MP3",
    "V_MPEG4/ISO/AVC",
    "Vorbis",
};

pub static WEBM: Set<&str> = phf_set! {
    "A_VORBIS",
    "Opus",
    "VP8",
    "VP9",
    "Vorbis",
};
