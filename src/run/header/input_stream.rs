use crate::ffmpeg::{
    self,
    format::{self, context},
};
use crate::{Config, MediaInfo, Result, StreamsOrderItem, markers::MISubCharEncoding};
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

    let ictx = match get_sub_charenc(mi, src).map(|s| new_ictx_reencode_subs(cfg, job, ord, src, s))
    {
        Some(Ok(ctx)) => Ok(ctx),
        Some(Err(err)) => {
            warn!(
                "Fail reencode charset to UTF-8: {}. Copying src subtitles '{}'",
                err,
                src.display()
            );
            format::input(src)
        }
        None => format::input(src),
    }?;

    Ok(ictx)
}

fn get_sub_charenc<'a>(mi: &'a mut MediaInfo, src: &Path) -> Option<&'a str> {
    if *mi.cfg.auto_flags.encs {
        mi.get(MISubCharEncoding, src)
            .and_then(|enc| enc.get_ffmpeg_sub_charenc())
    } else {
        None
    }
}

fn new_ictx_reencode_subs(
    cfg: &Config,
    job: u8,
    ord: &StreamsOrderItem,
    src: &Path,
    charenc: &str,
) -> Result<context::Input> {
    debug!("Reencoding subtitle charset '{}'...", src.display());

    let enc = Encoding::for_label_no_replacement(charenc.as_bytes())
        .ok_or_else(|| err!("Unrecognized charenc key '{}'", charenc))?;
    let src_file = fs::File::open(src)?;
    let mut reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(enc))
        .build(src_file);

    let mut dest = cfg
        .output
        .temp_dir
        .join(format!("{}-reencoded-subs-{}", job, ord.src_num));
    if let Some(ext) = src.extension() {
        dest.add_extension(ext);
    }
    let mut dest_file = fs::File::create(&dest)?;

    io::copy(&mut reader, &mut dest_file)?;
    let ictx = format::input(&dest)?;
    Ok(ictx)
}
