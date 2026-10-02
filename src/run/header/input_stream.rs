use crate::ffmpeg::{
    self,
    format::{self, context},
};
use crate::media_info::MarkMediaInfoSubsEncoding;
use crate::{CharEncoding, Config, MediaInfo, Msg, Result, StreamsOrderItem, display};
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use log::{debug, warn};
use std::{fs, io, path::Path};

pub fn new<'a>(
    mi: &mut MediaInfo,
    icontexts: &'a mut Vec<context::Input>,
    ord: &StreamsOrderItem,
) -> Result<ffmpeg::Stream<'a>> {
    if icontexts.get(ord.src_num).is_none() {
        icontexts.push(new_ictx(mi, ord)?);
    }

    let ictx = &icontexts[ord.src_num];
    ictx.stream(ord.i_stream)
        .ok_or_else(|| err!("Not found stream"))
}

fn new_ictx(mi: &mut MediaInfo, ord: &StreamsOrderItem) -> Result<context::Input> {
    let cfg = mi.cfg;
    let job = mi.job;
    let src = ord.src();

    if let Some(CharEncoding::NotUtf8Compatible(enc)) = mi.get(MarkMediaInfoSubsEncoding, src) {
        match new_ictx_reencode_subs(cfg, job, ord, src, enc) {
            Ok(ictx) => return Ok(ictx),
            Err(err) => {
                warn!(
                    "fail reencode subtutle to UTF-8: {}. Copying without reencode '{}'",
                    err,
                    display(src)
                );
            }
        }
    }

    let ictx = format::input(src)?;
    Ok(ictx)
}

fn new_ictx_reencode_subs(
    cfg: &Config,
    job: u8,
    ord: &StreamsOrderItem,
    src: &Path,
    enc: &'static Encoding,
) -> Result<context::Input> {
    debug!("{} '{}'...", Msg::ConvertingSubtitleEncoding, display(src));

    let src_file = fs::File::open(src)?;
    let mut reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(enc))
        .build(src_file);

    let mut dest = cfg
        .output
        .temp_dir()
        .join(format!("{}-reencoded-subs-{}", job, ord.src_num));
    if let Some(ext) = src.extension() {
        dest.add_extension(ext);
    }
    let mut dest_file = fs::File::create(&dest)?;

    io::copy(&mut reader, &mut dest_file)?;
    let ictx = format::input(&dest)?;
    Ok(ictx)
}
