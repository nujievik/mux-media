use super::*;
use crate::FFMPEG_PADDING;
use crate::ffmpeg::{self, codec, ffi};

pub(crate) fn copy<'a>(
    ist: &format::stream::Stream,
    octx: &'a mut format::context::Output,
) -> Result<ffmpeg::StreamMut<'a>> {
    let meta = ist.metadata();
    let mimetype = meta
        .get("mimetype")
        .ok_or(err!("attachment stream has not mimetype metatada"))?;
    let filename = meta
        .get("filename")
        .ok_or(err!("attachment stream has not filename metatada"))?;

    let av_pkt = unsafe { &(*ist.as_ptr()).attached_pic };

    if av_pkt.data.is_null() || av_pkt.size <= 0 {
        return Err(err!("attachment stream has not attached_pic packet"));
    }

    let size = av_pkt.size as usize;

    let buf = unsafe {
        let buf = ffi::av_malloc(size + FFMPEG_PADDING) as *mut u8;
        if buf.is_null() {
            return Err(err!("fail av_malloc"));
        }

        std::ptr::copy_nonoverlapping(av_pkt.data, buf, size);
        // zeroed padding
        std::ptr::write_bytes(buf.add(size), 0, FFMPEG_PADDING);
        buf
    };

    let mut ost = octx.add_stream(codec::Id::None)?;
    ost.set_parameters(ist.parameters());
    unsafe {
        (*ost.as_mut_ptr()).disposition = ffi::AV_DISPOSITION_ATTACHED_PIC;
    }

    let ost_params = unsafe { &mut *ost.parameters().as_mut_ptr() };
    ost_params.codec_tag = 0;
    ost_params.codec_type = ffi::AVMediaType::AVMEDIA_TYPE_ATTACHMENT;
    ost_params.codec_id = ffi::AVCodecID::AV_CODEC_ID_NONE;
    ost_params.extradata = buf;
    ost_params.extradata_size = size as _;

    let mut meta = ffmpeg::Dictionary::new();
    meta.set("mimetype", mimetype);
    meta.set("filename", filename);
    ost.set_metadata(meta);

    Ok(ost)
}
