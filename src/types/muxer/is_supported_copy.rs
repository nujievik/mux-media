use crate::{CodecId, Muxer, Stream, ffmpeg::codec::id::Id};

impl Muxer {
    pub(crate) fn is_supported_copy(&self, stream: &Stream) -> bool {
        let codec = stream.codec;
        return match self {
            Muxer::Avi => avi(codec),
            Muxer::Mp4 => mp4(codec),
            Muxer::Matroska => true,
            Muxer::Webm => webm(codec),
        };

        fn avi(codec: CodecId) -> bool {
            match codec.0 {
                Id::MP3 => true,
                Id::MPEG4 | Id::MSMPEG4V1 | Id::MSMPEG4V2 | Id::MSMPEG4V3 => true,
                Id::MPEG4SYSTEMS => true,
                _ => false,
            }
        }

        fn mp4(codec: CodecId) -> bool {
            match codec.0 {
                Id::AAC | Id::AAC_LATM => true,
                Id::AC3 => true,
                Id::EAC3 => true,
                Id::MP3 => true,
                Id::VORBIS => true,
                // video
                Id::H264 => true,
                Id::MPEG4 | Id::MSMPEG4V1 | Id::MSMPEG4V2 | Id::MSMPEG4V3 => true,
                Id::MPEG4SYSTEMS => true,
                _ => false,
            }
        }

        fn webm(codec: CodecId) -> bool {
            match codec.0 {
                Id::OPUS => true,
                Id::VORBIS => true,
                // video
                Id::VP8 => true,
                Id::VP9 => true,
                // subs
                Id::WEBVTT => true,
                _ => false,
            }
        }
    }
}
