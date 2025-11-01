use crate::{CodecID, Muxer, TrackType, ffmpeg::codec::id::Id};
use enum_map::{EnumMap, enum_map};

impl Muxer {
    pub(crate) fn is_supported_codec(self, codec: CodecID) -> bool {
        return match self {
            Muxer::AVI => avi(codec),
            Muxer::MP4 => mp4(codec),
            Muxer::Matroska => true,
            Muxer::Webm => webm(codec),
        };

        fn avi(codec: CodecID) -> bool {
            match codec.0 {
                Id::MP3 => true,
                Id::MPEG4 | Id::MSMPEG4V1 | Id::MSMPEG4V2 | Id::MSMPEG4V3 => true,
                Id::MPEG4SYSTEMS => true,
                _ => false,
            }
        }

        fn mp4(codec: CodecID) -> bool {
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

        fn webm(codec: CodecID) -> bool {
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

pub(crate) struct SupportedTracks {
    muxer: Muxer,
    map: Option<EnumMap<TrackType, u64>>,
}

impl SupportedTracks {
    pub(crate) fn new(muxer: Muxer) -> Self {
        let map = match muxer {
            Muxer::AVI => Some(enum_map! { TrackType::Audio | TrackType::Video => 1, _ => 0 }),
            _ => None,
        };
        Self { muxer, map }
    }

    pub(crate) fn is_supported(&mut self, ty: TrackType) -> bool {
        if !self.is_supported_track(ty) {
            return false;
        }

        match self.map.as_mut() {
            Some(map) if map[ty] > 0 => {
                map[ty] -= 1;
                true
            }
            Some(_) => false,
            None => true,
        }
    }

    fn is_supported_track(&self, ty: TrackType) -> bool {
        match self.muxer {
            Muxer::AVI => matches!(ty, TrackType::Audio | TrackType::Video),
            Muxer::MP4 => ty.is_customizable(),
            Muxer::Matroska => true,
            Muxer::Webm => matches!(ty, TrackType::Audio | TrackType::Video),
        }
    }
}
