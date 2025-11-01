use crate::{AttachType, CodecID};

/// Cache of [`crate::MediaInfo`] is separate for each attach in media.
#[derive(Clone, Debug, Default)]
pub struct CacheMIOfFileAttach {
    pub stream_i: usize,
    pub codec_id: CodecID,
    pub ty: AttachType,
}
