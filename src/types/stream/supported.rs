use crate::{Muxer, StreamType};
use enum_map::{EnumMap, enum_map};

#[derive(Debug)]
pub struct StreamsSupported {
    muxer: Muxer,
    map: Option<EnumMap<StreamType, usize>>,
}

impl StreamsSupported {
    pub fn new(muxer: Muxer) -> Self {
        let map = match muxer {
            Muxer::AVI => Some(enum_map! { StreamType::Audio | StreamType::Video => 1, _ => 0 }),
            _ => None,
        };
        Self { muxer, map }
    }

    pub fn is_supported(&mut self, ty: StreamType) -> bool {
        if !is_supported_ty(self, ty) {
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
}

const fn is_supported_ty(sup: &StreamsSupported, ty: StreamType) -> bool {
    match sup.muxer {
        Muxer::AVI => matches!(ty, StreamType::Audio | StreamType::Video),
        Muxer::MP4 => ty.is_track(),
        Muxer::Matroska => true,
        Muxer::Webm => matches!(ty, StreamType::Audio | StreamType::Video),
    }
}
