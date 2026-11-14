use crate::ffmpeg::{
    self,
    format::{self, context},
};
use crate::{MediaInfo, Result, media_info::StreamsOrderItem};

pub fn new<'a>(
    _: &mut MediaInfo,
    icontexts: &'a mut Vec<context::Input>,
    ord: &StreamsOrderItem,
) -> Result<ffmpeg::Stream<'a>> {
    if icontexts.get(ord.src_num).is_none() {
        let ictx = format::input(ord.src())?;
        icontexts.push(ictx);
    }

    let ictx = &icontexts[ord.src_num];
    ictx.stream(ord.i_stream)
        .ok_or_else(|| err!("not found stream"))
}
