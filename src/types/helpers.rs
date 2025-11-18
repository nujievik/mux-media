use crate::{Result, StreamType, ffmpeg};
use std::{
    ffi::{OsStr, OsString},
    fs::{File, canonicalize},
    io::BufWriter,
    path::{Path, PathBuf},
};

#[inline]
pub(crate) fn try_write_args_to_json<I, T>(args: I, json: &Path) -> Result<Vec<String>>
where
    I: IntoIterator<Item = T>,
    T: AsRef<OsStr>,
{
    let args = args
        .into_iter()
        .map(|arg| {
            arg.as_ref().to_str().map(|s| s.to_string()).ok_or_else(|| {
                let path = Path::new(arg.as_ref());
                format!("Unsupported UTF-8 symbol in '{}'", path.display()).into()
            })
        })
        .collect::<Result<Vec<String>>>()?;

    let file = File::create(json)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &args)?;

    Ok(args)
}

#[inline(always)]
pub(crate) fn try_canonicalize_and_open(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = canonicalize(path)?;
    if !path.is_file() {
        return Err("Is not a file".into());
    }
    File::open(&path)?;
    Ok(path)
}

#[inline(always)]
pub(crate) fn os_str_starts_with(prefix: &OsStr, longer: &OsStr) -> bool {
    longer
        .as_encoded_bytes()
        .starts_with(prefix.as_encoded_bytes())
}

#[inline]
pub(crate) fn os_str_tail(prefix: &OsStr, longer: &OsStr) -> Result<OsString> {
    let prefix_b = prefix.as_encoded_bytes();
    let longer_b = longer.as_encoded_bytes();

    if !longer_b.starts_with(prefix_b) {
        return Err(format!("Longer {:?} is not starts with {:?}", longer, prefix).into());
    }

    let prefix_len = prefix_b.len();

    if longer_b.len() == prefix_len {
        return Ok(OsString::new());
    }

    let bytes = &longer_b[prefix_len..];
    // Safety: `bytes` is a suffix of `longer_b`, which was originally obtained from a valid `OsStr`.
    // Since `prefix_b` is a valid prefix, the remaining bytes (`bytes`) are also guaranteed
    // to form a valid `OsStr` on this platform.
    unsafe { Ok(OsStr::from_encoded_bytes_unchecked(bytes).into()) }
}

pub(crate) fn try_ffmpeg_opened(
    ty: StreamType,
    stream: &ffmpeg::Stream,
) -> Result<ffmpeg::decoder::Opened> {
    let d = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?.decoder();

    let d = if ty.is_audio() {
        d.audio()?.0
    } else if ty.is_video() {
        d.video()?.0
    } else {
        unreachable!("Unsupported stream type");
    };

    Ok(d)
}

pub(crate) fn ffmpeg_stream_i_tb(stream: &ffmpeg::Stream) -> (usize, f64) {
    let i = stream.index();
    let tb = stream.time_base();
    let tb_f64 = tb.numerator() as f64 / tb.denominator() as f64;
    (i, tb_f64)
}
