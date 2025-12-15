use crate::{
    ArcPathBuf, CacheMIOfFile, CacheState, Config, Extension, MediaInfo, Msg, MuxCurrent, MuxError,
    Muxer, Result,
    ffmpeg::{self, sys},
    i18n::logs,
    markers::MICmnStem,
    types::input::iters::MediaGroupedByStem,
};
use log::{error, info, trace};
use rayon::prelude::*;
use std::{
    ffi::{CString, OsString},
    fs,
    path::{Path, PathBuf},
    ptr,
    sync::Mutex,
};

impl Config {
    /// Tries perform muxing, returning a count of successfully muxed media files.
    ///
    /// # Errors
    ///
    /// - **Only if** [`Config::exit_on_err`] is true.
    ///
    /// - Returns a muxing error if one occurs during processing.
    //
    // Panics on a [`Mutex::lock`] error.
    // This indicates an internal logic error that must be fixed.
    #[inline]
    pub fn mux(&self) -> Result<usize> {
        let fonts = init_external_fonts(self);
        let cnt = Mutex::new(0usize);
        let it = Mutex::new(self.input.iter_media_grouped_by_stem());

        (0..self.jobs).into_par_iter().try_for_each(|j| {
            let mut mi = MediaInfo::new(self, j);
            loop {
                let g = { it.lock().map_or(None, |mut it| it.next()) };
                match g {
                    Some(g) => mux_media_group(self, fonts.as_ref(), &cnt, &mut mi, g)?,
                    None => return Ok::<(), MuxError>(()),
                }
                mi.clear();
            }
        })?;

        return Ok(cnt.into_inner().unwrap_or(0));

        fn init_external_fonts(cfg: &Config) -> Option<(ArcPathBuf, CacheMIOfFile)> {
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

        fn mux_media_group(
            cfg: &Config,
            fonts: Option<&(ArcPathBuf, CacheMIOfFile)>,
            cnt: &Mutex<usize>,
            mi: &mut MediaInfo<'_>,
            m: MediaGroupedByStem,
        ) -> Result<()> {
            let out = cfg.output.build_out(m.out_name_middle);
            info!("{} '{}'...", Msg::Muxing, out.display());

            match init_current_media(fonts, mi, m.stem, m.files, &out) {
                MuxCurrent::Ok(()) => (),
                MuxCurrent::Continue => return Ok(()),
                MuxCurrent::Err(e) => return Err(e),
            }

            match cfg.muxer.mux_current(mi, &out) {
                MuxCurrent::Ok(tool_out) => {
                    trace!("{}", tool_out);
                    info!("{} '{}'", Msg::SuccessMuxed, out.display());
                    if let Ok(mut cnt) = cnt.lock() {
                        *cnt += 1;
                    }
                }
                MuxCurrent::Continue => (),
                MuxCurrent::Err(e) if cfg.exit_on_err => return Err(e),
                MuxCurrent::Err(e) => error!("{}", e),
            };

            Ok(())
        }

        fn init_current_media(
            fonts: Option<&(ArcPathBuf, CacheMIOfFile)>,
            mi: &mut MediaInfo,
            stem: OsString,
            files: Vec<PathBuf>,
            out: &Path,
        ) -> MuxCurrent<()> {
            if out.exists() {
                logs::warn_file_is_already_exists(out);
                return MuxCurrent::Continue;
            }

            mi.set_cmn(MICmnStem, stem);

            if let Err(e) = mi
                .try_insert_many(files)
                .and_then(|_| mi.try_finalize_init_streams())
            {
                return Err(e).into();
            }

            if let Some((f, cache)) = fonts {
                mi.cache.of_files.insert(f.clone(), cache.clone());
            }

            if mi.cache.of_files.is_empty() {
                logs::warn_not_out_save_any(out);
                MuxCurrent::Continue
            } else {
                MuxCurrent::Ok(())
            }
        }
    }
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
