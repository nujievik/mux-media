use crate::{
    ArcPathBuf, CacheMIOfFile, CacheState, Config, Extension, MediaInfo, Muxer, Result,
    ffmpeg::{self, sys},
};
use std::{
    ffi::CString,
    fs,
    path::{Path, PathBuf},
    ptr,
};

pub(super) fn init_external_fonts(cfg: &Config) -> Option<(ArcPathBuf, CacheMIOfFile)> {
    if !matches!(cfg.muxer, Muxer::Matroska) {
        return None;
    }

    let fonts = cfg.input.collect_fonts();
    if fonts.is_empty() {
        return None;
    }

    let fall = |e| {
        log::warn!("Fail write external fonts: {}. Skipping", e);
        None
    };

    let out = cfg.output.temp_dir.join("external-fonts.mkv");
    if let Err(e) = write_temp_fonts(fonts, &out) {
        return fall(e);
    }

    let streams = match MediaInfo::help_build_streams(&out) {
        Ok(xs) => xs,
        Err(e) => return fall(e),
    };
    let cache = CacheMIOfFile {
        streams: CacheState::Cached(streams),
        ..Default::default()
    };

    Some((out.into(), cache))
}

fn write_temp_fonts(fonts: Vec<PathBuf>, out: &Path) -> Result<()> {
    let mut octx = ffmpeg::format::output(out)?;
    add_dummy_subtitle_stream(&mut octx)?;
    add_attachments(&mut octx, fonts);

    octx.write_header()?;
    write_dummy_subtitle_packet(&mut octx)?;
    octx.write_trailer()?;
    Ok(())
}

fn add_dummy_subtitle_stream(octx: &mut ffmpeg::format::context::Output) -> Result<()> {
    unsafe {
        let st = sys::avformat_new_stream(octx.as_mut_ptr(), ptr::null());
        if st.is_null() {
            return Err(err!("Fail add dummy subtitle stream"));
        }

        (*st).time_base = sys::AVRational { num: 1, den: 1000 };

        let par = (*st).codecpar;
        (*par).codec_type = sys::AVMediaType::AVMEDIA_TYPE_SUBTITLE;
        (*par).codec_id = sys::AVCodecID::AV_CODEC_ID_SUBRIP;
    }
    Ok(())
}

fn add_attachments(octx: &mut ffmpeg::format::context::Output, fonts: Vec<PathBuf>) {
    for font in &fonts {
        let ext = some_or!(font.extension(), continue);
        let ext = some_or!(Extension::new(ext.as_encoded_bytes()), continue);
        let mime = match ext {
            Extension::Otf => c"application/vnd.ms-opentype",
            Extension::Ttf => c"application/x-truetype-font",
            _ => continue,
        };

        let name = some_or!(font.file_name(), continue);
        let name = some_or!(CString::new(name.as_encoded_bytes()).ok(), continue);

        let data = some_or!(fs::read(font).ok(), continue);

        unsafe {
            let size = data.len();
            let buf = sys::av_malloc(size + sys::AV_INPUT_BUFFER_PADDING_SIZE as usize) as *mut u8;
            if buf.is_null() {
                continue;
            }

            let st = sys::avformat_new_stream(octx.as_mut_ptr(), ptr::null());
            if st.is_null() {
                sys::av_free(buf as *mut _);
                continue;
            }

            let par = (*st).codecpar;
            (*par).codec_type = sys::AVMediaType::AVMEDIA_TYPE_ATTACHMENT;
            (*par).codec_id = sys::AVCodecID::AV_CODEC_ID_NONE;

            ptr::copy_nonoverlapping(data.as_ptr(), buf, size);
            (*par).extradata = buf;
            (*par).extradata_size = size as i32;

            sys::av_dict_set(&mut (*st).metadata, c"filename".as_ptr(), name.as_ptr(), 0);
            sys::av_dict_set(&mut (*st).metadata, c"mimetype".as_ptr(), mime.as_ptr(), 0);
        }
    }
}

fn write_dummy_subtitle_packet(octx: &mut ffmpeg::format::context::Output) -> Result<()> {
    let err = || Err(err!("Fail write dummy subtitle stream"));
    let text = b"1\n00:00:00,000 --> 00:00:01,000\n.\n";

    unsafe {
        let buf = sys::av_malloc(text.len()) as *mut u8;
        if buf.is_null() {
            return err();
        }

        ptr::copy_nonoverlapping(text.as_ptr(), buf, text.len());

        let pkt = sys::av_packet_alloc();
        if pkt.is_null() {
            sys::av_free(buf as *mut _);
            return err();
        }

        sys::av_packet_from_data(pkt, buf, text.len() as i32);

        (*pkt).stream_index = 0;
        (*pkt).pts = 0;
        (*pkt).dts = 0;
        (*pkt).duration = 1000;

        let ret = sys::av_interleaved_write_frame(octx.as_mut_ptr(), pkt);

        sys::av_packet_unref(pkt);
        sys::av_packet_free(&mut (pkt as *mut _));

        if ret < 0 {
            return err();
        }
    }
    Ok(())
}
