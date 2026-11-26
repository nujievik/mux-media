use super::*;
use crate::ffmpeg::{self, codec, ffi};
use crate::{Extension, FFMPEG_PADDING};
use std::{ffi::c_int, fs, ptr};

pub(crate) fn add_font_file<'a>(
    octx: &'a mut format::context::Output,
    font: &Path,
    filename: &str,
) -> Result<ffmpeg::StreamMut<'a>> {
    let mimetype = match Extension::from_path(font) {
        Some(Extension::Otf) => "application/vnd.ms-opentype",
        Some(Extension::Ttf) => "application/x-truetype-font",
        _ => return Err(err!("unsupported_extension")),
    };

    let data = fs::read(font)?;
    let size = data.len();

    if size > c_int::MAX as usize {
        return Err(err!("font size > c_int::MAX"));
    }

    let buf = unsafe {
        let buf = ffi::av_malloc(size + FFMPEG_PADDING) as *mut u8;
        if buf.is_null() {
            return Err(err!("fail av_malloc"));
        }

        ptr::copy_nonoverlapping(data.as_ptr(), buf, size);
        ptr::write_bytes(buf.add(size), 0, FFMPEG_PADDING);
        buf
    };

    let mut ost = octx.add_stream(codec::Id::None)?;

    let ost_params = unsafe { &mut *ost.parameters().as_mut_ptr() };
    ost_params.codec_tag = 0;
    ost_params.codec_type = ffi::AVMediaType::AVMEDIA_TYPE_ATTACHMENT;
    ost_params.extradata = buf;
    ost_params.extradata_size = size as _;

    let mut meta = ffmpeg::Dictionary::new();
    meta.set("mimetype", mimetype);
    meta.set("filename", filename);
    ost.set_metadata(meta);

    Ok(ost)
}
