use super::retiming::subs::{self, ty::SubType};
use super::{StreamsOrder, StreamsOrderItem};
use crate::config::MarkConfigSubsEncodings;
use crate::ffmpeg::{codec::id::Id, format};
use crate::media_info::MarkMediaInfoStreams;
use crate::{CharEncoding, MediaInfo, Msg, Result, display};
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use log::{debug, warn};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

macro_rules! warn_fail_convert {
    ($item:ident, $err:expr) => {
        warn!(
            "{}: {}. {} '{}' {} {}",
            Msg::FailConvertSubtitlesEncoding,
            $err,
            Msg::CopyAsIs,
            display(&$item.key),
            Msg::Stream,
            $item.key_i_stream,
        )
    };
}

impl StreamsOrder {
    /// Tries convert subs encodings.
    ///
    /// # Errors
    /// - **Only if** config exit_on_err true.
    /// - Fails convert a subtitles.
    pub fn convert_subs_encodings(mut self, mi: &mut MediaInfo) -> Result<StreamsOrder> {
        // all in old numbers
        let mut num_adds = vec![None; self.0.len()];
        let mut max_num_add = 0usize;
        let mut max_num = 0usize;

        for item in &mut self.0 {
            let old_src_num = item.src_num;

            match num_adds[old_src_num] {
                Some(add) => item.src_num += add,
                None if max_num >= old_src_num => (),
                None => {
                    max_num = old_src_num;
                    if max_num_add > 0 {
                        item.src_num += max_num_add;
                        num_adds[old_src_num] = Some(max_num_add);
                    }
                }
            }

            if !item.ty.is_subtitles() {
                continue;
            }

            match convert_item(mi, item) {
                Ok(Some(converted_src)) => {
                    item.src = Some(converted_src);
                    item.i_stream = 0;
                    if !item.is_first_entry {
                        max_num_add += 1;
                        item.src_num = max_num + max_num_add;
                        item.is_first_entry = true;
                        num_adds[old_src_num] = Some(max_num_add);
                    }
                }
                Ok(None) => (),
                Err(e) if mi.cfg.exit_on_err => return Err(e),
                Err(e) => warn_fail_convert!(item, e),
            }
        }

        Ok(self)
    }
}

fn convert_item(mi: &mut MediaInfo, item: &StreamsOrderItem) -> Result<Option<PathBuf>> {
    let cfg = mi.cfg;
    let path_configs = mi.path_configs(&item.key);
    let stream = &mi.try_get(MarkMediaInfoStreams, &item.key)?[item.key_i_stream];

    let (i, encodings) = cfg.stream_value(MarkConfigSubsEncodings, &path_configs, &stream);
    let enc = encodings.get(&i, &stream.lang);

    match enc {
        // do nothing if is utf-8
        Some(enc) if matches!(enc.name(), "UTF-8") => return Ok(None),
        // do nothing if no manual enc && no auto enc
        None if !*cfg.auto_flags.subs_encodings => return Ok(None),
        _ => (),
    }

    if let Some(sub_ty) = SubType::new_from_path(&item.key) {
        return convert_text_file(mi, item, &item.key, None, sub_ty);
    }

    let sub_ty = match SubType::get_from_codec_id(mi, &item.key, item.key_i_stream) {
        Some(ty) => ty,
        None => {
            warn_fail_convert!(item, Msg::UnsupportedSubtitlesFormat.as_str_localized());
            return Ok(None);
        }
    };

    let extracted_path = match extract_subs(mi, item, enc, sub_ty)? {
        Some(p) => p,
        None => return Ok(None),
    };

    convert_text_file(mi, item, &extracted_path, enc, sub_ty)
}

fn temp_path(mi: &MediaInfo, item: &StreamsOrderItem, sub_ty: SubType, pname: &str) -> PathBuf {
    mi.cfg.output.temp_dir().join(format!(
        "{}-{}-{}-{}.{}",
        mi.job,
        pname,
        item.src_num,
        item.key_i_stream,
        sub_ty.as_ext()
    ))
}

fn convert_text_file(
    mi: &mut MediaInfo,
    item: &StreamsOrderItem,
    src: &Path,
    enc: Option<&'static Encoding>,
    sub_ty: SubType,
) -> Result<Option<PathBuf>> {
    let enc = if let Some(enc) = enc {
        enc
    } else {
        match CharEncoding::detect_text_file(src, mi.buffer_mut()) {
            CharEncoding::NotUtf8Compatible(enc) => enc,
            _ => return Ok(None),
        }
    };

    debug!(
        "{} '{}' {} {}...",
        Msg::ConvertingSubtitlesEncoding,
        display(&item.key),
        Msg::Stream,
        item.key_i_stream,
    );

    let dest = temp_path(mi, item, sub_ty, "converted-subs");

    rw_file(src, enc, &dest)?;

    debug!(
        "{} '{}' {} {}",
        Msg::SuccessfullyConvertedSubtitles,
        display(&item.key),
        Msg::Stream,
        item.key_i_stream,
    );
    Ok(Some(dest))
}

fn rw_file(src: &Path, enc: &'static Encoding, dest: &Path) -> Result<()> {
    let mut dest = fs::File::create(dest)?;
    let src = fs::File::open(src)?;
    let mut src = DecodeReaderBytesBuilder::new()
        .encoding(Some(enc))
        .build(src);

    io::copy(&mut src, &mut dest)?;
    Ok(())
}

fn extract_subs(
    mi: &mut MediaInfo,
    item: &StreamsOrderItem,
    enc: Option<&'static Encoding>,
    sub_ty: SubType,
) -> Result<Option<PathBuf>> {
    let extracted_path = temp_path(mi, item, sub_ty, "extracted-subs");

    if enc.is_none() {
        try_extract_some_packets(&item.key, item.key_i_stream, &extracted_path)?;
        match CharEncoding::detect_text_file(&extracted_path, mi.buffer_mut()) {
            CharEncoding::NotUtf8Compatible(_) => (),
            _ => return Ok(None),
        }
    }

    subs::try_extract(&item.key, item.key_i_stream, sub_ty, &extracted_path)?;
    Ok(Some(extracted_path))
}

fn try_extract_some_packets(src: &Path, i_stream: usize, dest: &Path) -> Result<()> {
    let mut ictx = format::input(src)?;
    let ist = ictx
        .stream(i_stream)
        .ok_or_else(|| err!("invalid stream index"))?;

    let mut octx = format::output(dest)?;
    let ost_index = {
        let mut ost = octx.add_stream(Id::None)?;
        ost.set_parameters(ist.parameters());
        ost.index()
    };

    octx.write_header()?;

    const MAX_PACKETS: u8 = 200;
    let mut cnt = 0;

    for (stream, mut packet) in ictx.packets() {
        if stream.index() != i_stream {
            continue;
        }
        packet.set_stream(ost_index);
        packet.write(&mut octx)?;
        cnt += 1;
        if cnt == MAX_PACKETS {
            break;
        }
    }

    octx.write_trailer()?;
    Ok(())
}
