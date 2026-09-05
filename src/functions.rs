use crate::ffmpeg::{self, codec, format};
use crate::{Config, Result};
use std::path::{self, Path, PathBuf};

#[cfg(windows)]
static LONG_PATH_PREFIX: &str = r"\\?\";

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

    if path
        .as_os_str()
        .as_encoded_bytes()
        .starts_with(LONG_PATH_PREFIX.as_bytes())
    {
        return path;
    }

    let mut prf_path = std::ffi::OsString::from(LONG_PATH_PREFIX);
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

/// Displays a path without `\\?\` prefix if exists.
pub(crate) fn display<P>(path: &P) -> path::Display<'_>
where
    P: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();

    #[cfg(windows)]
    {
        let src_bytes = path.as_os_str().as_encoded_bytes();

        if src_bytes.starts_with(LONG_PATH_PREFIX.as_bytes()) {
            let display_bytes = if src_bytes.len() == 4 {
                &[]
            } else {
                &src_bytes[4..]
            };

            // SAFETY: The prefix `\\?\` consists entirely of ASCII characters (1 byte per character).
            // Slicing at index 4 is guaranteed to fall on a valid UTF-8/WTF-8 code point boundary,
            // ensuring that the remaining `display_bytes` retain a valid WTF-8 structure.
            let os_str = unsafe { std::ffi::OsStr::from_encoded_bytes_unchecked(display_bytes) };

            return Path::new(os_str).display();
        }
    }

    path.display()
}
