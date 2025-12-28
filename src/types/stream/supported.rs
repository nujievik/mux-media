use crate::{Container, StreamType};
use enum_map::EnumMap;

#[derive(Debug)]
pub struct StreamsSupported {
    container: Container,
    map: Option<EnumMap<StreamType, usize>>,
}

impl StreamsSupported {
    pub fn new(container: Container) -> Self {
        let map = match container {
            //Container::Avi => Some(enum_map! { StreamType::Audio | StreamType::Video => 1, _ => 0 }),
            _ => None,
        };
        Self { container, map }
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

const fn is_supported_ty(sup: &StreamsSupported, _ty: StreamType) -> bool {
    match sup.container {
        //Container::Avi => matches!(ty, StreamType::Audio | StreamType::Video),
        //Container::Mp4 => ty.is_track(),
        Container::Matroska => true,
        //Container::Webm => matches!(ty, StreamType::Audio | StreamType::Video),
    }
}
