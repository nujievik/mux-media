use crate::ffmpeg::{codec::id::Id, util::media::Type};

#[derive(Copy, Clone, Debug)]
pub struct FfmpegStream {
    pub ty: Type,
    pub id: Id,
}
