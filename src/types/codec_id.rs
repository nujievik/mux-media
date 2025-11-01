use crate::ffmpeg::codec::id::Id;

/// A wrapper around [`ffmpeg::codec::id::Id`].
#[derive(Copy, Clone, Debug)]
pub struct CodecID(pub Id);

deref_singleton_tuple_struct!(CodecID, Id);

impl Default for CodecID {
    fn default() -> Self {
        CodecID(Id::None)
    }
}

impl CodecID {
    pub(crate) fn is_attach_other(self) -> bool {
        match self.0 {
            Id::PNG => true,
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
