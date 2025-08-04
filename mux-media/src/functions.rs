use crate::{
    Input, MediaInfo, Msg, MuxConfig, MuxCurrent, MuxError, MuxLogger, Muxer, Output,
    TryFinalizeInit, TryInit,
    i18n::logs,
    markers::{MCExitOnErr, MCInput, MCMuxer, MCOutput, MCVerbosity, MICmnStem},
};
use log::{LevelFilter, error, info, trace, warn};
use std::{
    ffi::OsString,
    path::{MAIN_SEPARATOR, Path, PathBuf},
};

/// Byte form of [`MAIN_SEPARATOR`].
pub const SEP_BYTES: &[u8] = &[MAIN_SEPARATOR as u8];

/// String form of [`MAIN_SEPARATOR`].
// checked for valid UTF-8 at compile time
pub const SEP_STR: &str = match str::from_utf8(SEP_BYTES) {
    Ok(s) => s,
    Err(_) => panic!("MAIN_SEPARATOR is not valid UTF-8"),
};

/// Runs muxing and invokes all other components.
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
pub fn run() -> Result<(), MuxError> {
    let mc = {
        let mut mc = MuxConfig::try_init()?;
        mc.try_finalize_init()?;
        mc
    };

    MuxLogger::init_with_filter(LevelFilter::from(*mc.field::<MCVerbosity>()));

    let result = mux(&mc);
    mc.field::<MCOutput>().remove_created_dirs();

    result.map(|cnt| match cnt {
        0 => warn!("{}", Msg::NotMuxedAny),
        _ => {
            info!("{} {} {}", Msg::SuccessMuxed, cnt, Msg::LMedia);
            mc.write_args_to_json_or_log();
        }
    })
}

/// Runs muxing with [`MuxConfig`].
///
/// Returns the number of successfully muxed media.
///
/// # Errors
///
/// - **Only if** [`MuxConfig`] is initialized with `exit_on_err = true`.
///
/// - Returns a muxing error.
pub fn mux(mux_config: &MuxConfig) -> Result<usize, MuxError> {
    let (input, output, exit_on_err, muxer, mut mi, mut cnt) = init_mux(mux_config);

    for media in input.iter_media_grouped_by_stem() {
        let out = output.build_out(media.out_name_middle);
        info!("{} '{}'...", Msg::Muxing, out.display());

        match init_current_media(exit_on_err, &mut mi, media.stem, media.files, &out) {
            MuxCurrent::Continue => continue,
            MuxCurrent::Ok(()) => (),
            MuxCurrent::Err(e) => return Err(e),
        }

        match muxer.mux_current(&mut mi, &out) {
            MuxCurrent::Continue => continue,
            MuxCurrent::Ok(tool_out) => {
                trace!("{}", tool_out);
                tool_out.log_warns();
                info!("{} '{}'", Msg::SuccessMuxed, out.display());
                cnt += 1;
            }
            MuxCurrent::Err(e) if exit_on_err => return Err(e),
            MuxCurrent::Err(e) => error!("{}", e),
        };

        mi.clear_current();
    }

    Ok(cnt)
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
/// let expected = PathBuf::from(r"\\?\path");
/// assert_eq!(&expected, &ensure_long_path_prefix("path"));
/// assert_eq!(&expected, &ensure_long_path_prefix(r"\\?\path"));
/// ```
#[cfg(windows)]
#[inline]
pub fn ensure_long_path_prefix(path: impl Into<PathBuf>) -> PathBuf {
    let path = path.into();

    if path.as_os_str().as_encoded_bytes().starts_with(b"\\\\?\\") {
        return path;
    }

    let mut prf_path = OsString::from("\\\\?\\");
    prf_path.push(path.as_os_str());
    prf_path.into()
}

#[inline(always)]
fn init_mux<'a>(mc: &'a MuxConfig) -> (&'a Input, &'a Output, bool, Muxer, MediaInfo<'a>, usize) {
    let input = mc.field::<MCInput>();
    let output = mc.field::<MCOutput>();
    let exit_on_err = *mc.field::<MCExitOnErr>();
    let muxer = *mc.field::<MCMuxer>();
    let mi = MediaInfo::from(mc);

    (input, output, exit_on_err, muxer, mi, 0)
}

#[inline(always)]
fn init_current_media(
    exit_on_err: bool,
    mi: &mut MediaInfo,
    stem: OsString,
    files: Vec<PathBuf>,
    out: &Path,
) -> MuxCurrent<()> {
    if out.exists() {
        logs::warn_file_is_already_exists(out);
        return MuxCurrent::Continue;
    }

    mi.set_cmn::<MICmnStem>(stem);

    if let Err(e) = mi.try_insert_many_filtered(files, exit_on_err) {
        return Err(e).into();
    }

    if mi.is_no_files() {
        logs::warn_not_out_save_any(out);
        return MuxCurrent::Continue;
    }

    MuxCurrent::Ok(())
}
