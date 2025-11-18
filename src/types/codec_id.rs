use crate::ffmpeg::codec::id::Id;

/// A wrapper around [`ffmpeg::codec::id::Id`].
#[derive(Copy, Clone, Debug)]
pub struct CodecId(pub Id);

deref_singleton_tuple_struct!(CodecId, Id);

impl Default for CodecId {
    fn default() -> Self {
        CodecId(Id::None)
    }
}

impl CodecId {
    pub(crate) fn is_attach(self) -> bool {
        match self.0 {
            Id::PNG => true,
            Id::LJPEG | Id::JPEGLS | Id::JPEG2000 => true,
            _ => false,
        }
    }

    pub(crate) fn is_font(self) -> bool {
        match self.0 {
            Id::TTF => true,
            Id::OTF => true,
            _ => false,
        }
    }
}
