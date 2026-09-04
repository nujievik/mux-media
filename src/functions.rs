use crate::ffmpeg::{self, codec, format};
use crate::{Config, Result};
use std::path::PathBuf;

/// Tries run muxing, returning a count of successfully muxed media files.
///
/// Delegates implementation to [`Config::mux`].
pub fn mux(cfg: &Config) -> Result<usize> {
    cfg.mux()
}

/// Returns a path unchanged (Unix).
///
/// ```
/// # use mux_media::ensure_long_path_prefix;
/// # use std::path::PathBuf;
/// #
/// let p = PathBuf::from("path");
/// assert_eq!(ensure_long_path_prefix(&p), p);
/// ```
#[cfg(unix)]
#[inline(always)]
pub fn ensure_long_path_prefix(path: impl Into<PathBuf>) -> PathBuf {
    path.into()
}

/// Adds `\\?\` prefix if missing (Windows).
///
/// ```
/// # use mux_media::ensure_long_path_prefix;
/// # use std::path::PathBuf;
/// #
/// let p = PathBuf::from(r"\\?\path");
/// assert_eq!(&ensure_long_path_prefix("path"), &p);
/// assert_eq!(&ensure_long_path_prefix(r"\\?\path"), &p);
/// ```
#[cfg(windows)]
#[inline]
pub fn ensure_long_path_prefix(path: impl Into<PathBuf>) -> PathBuf {
    let path = path.into();

    if path.as_os_str().as_encoded_bytes().starts_with(b"\\\\?\\") {
        return path;
    }

    let mut prf_path = std::ffi::OsString::from("\\\\?\\");
    prf_path.push(path.as_os_str());
    prf_path.into()
}

pub(crate) fn add_copy_stream<'a>(
    ist: &format::stream::Stream,
    octx: &'a mut format::context::Output,
) -> Result<ffmpeg::StreamMut<'a>> {
    let mut ost = octx.add_stream(codec::Id::None)?;
    ost.set_parameters(ist.parameters());

    unsafe {
        (*ost.as_mut_ptr()).sample_aspect_ratio = (*ist.as_ptr()).sample_aspect_ratio;
        (*ost.parameters().as_mut_ptr()).codec_tag = 0;
    }

    Ok(ost)
}
