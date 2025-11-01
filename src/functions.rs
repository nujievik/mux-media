use crate::{Msg, MuxConfig, MuxLogger, Result, TryFinalizeInit, ffmpeg};
use log::{info, warn};
use std::path::{MAIN_SEPARATOR, PathBuf};

/// Byte form of [`MAIN_SEPARATOR`].
/// ```
/// # use mux_media::SEP_BYTES;
/// # use std::path::MAIN_SEPARATOR;
/// assert_eq!(SEP_BYTES, &[MAIN_SEPARATOR as u8]);
/// ```
pub const SEP_BYTES: &[u8] = &[MAIN_SEPARATOR as u8];

/// String form of [`MAIN_SEPARATOR`].
/// ```
/// # use mux_media::SEP_STR;
/// # use std::path::MAIN_SEPARATOR;
/// let s = str::from_utf8(&[MAIN_SEPARATOR as u8]).unwrap();
/// assert_eq!(SEP_STR, s);
/// ```
// checked for valid UTF-8 at compile time
pub const SEP_STR: &str = match str::from_utf8(SEP_BYTES) {
    Ok(s) => s,
    Err(_) => panic!("MAIN_SEPARATOR is not valid UTF-8"),
};

/// Runs muxing, invoking all other components.
///
/// # Errors
///
/// 1. Successful exit cases (e.g., `--help`, `--list-targets`, etc.)
///    return an error with exit code `0`.
///
/// 2. CLI or JSON argument parsing failures
///    return an error with exit code `2`.
///
/// 3. All other errors return exit code `1`.
///
///    - Critical errors return immediately.
///
///    - Errors while processing current media return an error if `--exit-on-err` is set;
///      otherwise, muxing continues with the next media.
pub fn run() -> Result<()> {
    let cfg = init_cfg()?;
    MuxLogger::init_with_filter(cfg.verbosity.into());
    init_ffmpeg(&cfg)?;

    let result = cfg.mux();
    cfg.output.remove_created_dirs();

    return result.map(|cnt| match cnt {
        0 => warn!("{}", Msg::NotMuxedAny),
        _ => {
            info!("{} {} {}", Msg::SuccessMuxed, cnt, Msg::LMedia);
            cfg.save_config_or_warn();
        }
    });

    fn init_cfg() -> Result<MuxConfig> {
        let mut cfg = MuxConfig::try_init()?;
        if let Err(e) = cfg.try_finalize_init() {
            cfg.output.remove_created_dirs();
            Err(e)
        } else {
            Ok(cfg)
        }
    }

    fn init_ffmpeg(cfg: &MuxConfig) -> Result<()> {
        if let Err(e) = ffmpeg::init() {
            cfg.output.remove_created_dirs();
            Err(e.into())
        } else {
            ffmpeg::log::set_level(ffmpeg::log::Level::Quiet);
            Ok(())
        }
    }
}

/// Adds trailing [`MAIN_SEPARATOR`] if missing.
///
/// ```
/// # use mux_media::ensure_trailing_sep;
/// # use std::path::{PathBuf, MAIN_SEPARATOR};
/// #
/// let s = format!("path{}", MAIN_SEPARATOR);
/// let expected = PathBuf::from(&s);
/// assert_eq!(&expected, &ensure_trailing_sep("path"));
/// assert_eq!(&expected, &ensure_trailing_sep(s));
/// ```
#[inline]
pub fn ensure_trailing_sep(path: impl Into<PathBuf>) -> PathBuf {
    let path = path.into();

    if path.as_os_str().as_encoded_bytes().ends_with(SEP_BYTES) {
        return path;
    }

    let mut path = path.into_os_string();
    path.push(SEP_STR);
    path.into()
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
