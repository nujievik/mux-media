use crate::{Msg, MuxError, Muxer};
use log::{debug, warn};
use std::{ffi::OsStr, fmt, path::Path, process::Command};

pub(crate) fn warn_container_does_not_support(muxer: Muxer, src: &Path, i_stream: usize) {
    warn!(
        "{} {} {}. {} '{}' stream {}",
        muxer,
        Msg::ContainerDoesNotSupport,
        Msg::LMultipleTracksOrTypeTrack,
        Msg::Skipping,
        src.display(),
        i_stream
    );
}

#[inline(always)]
pub(crate) fn warn_err_write_json(err: MuxError) {
    warn!(
        "{}: {}. {} CLI ({})",
        Msg::ErrWriteJson,
        err.as_str_localized(),
        Msg::Using,
        Msg::MayFailIfCommandLong
    )
}

#[inline(always)]
pub(crate) fn warn_file_is_already_exists(path: &Path) {
    warn!(
        "{}. {} '{}'",
        Msg::FileIsAlreadyExists,
        Msg::Skipping,
        path.display()
    )
}

#[inline(always)]
pub(crate) fn warn_no_ext_media(stem: &OsStr) {
    warn!(
        "{}. {} '{}'",
        Msg::NoExtMediaFound,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display()
    )
}

#[inline(always)]
pub(crate) fn warn_not_out_save_any(out: &Path) {
    warn!(
        "{} '{}'. {}",
        Msg::NotOutSaveAny,
        out.display(),
        Msg::Skipping
    )
}

/*
#[inline(always)]
pub(crate) fn warn_not_out_change(out: &Path) {
    warn!(
        "{}. {} '{}'",
        Msg::NotOutChange,
        Msg::Skipping,
        out.display()
    )
}
*/

/*
#[inline(always)]
pub(crate) fn warn_not_saved_track_or_attach(path: &Path) {
    warn!(
        "{}. {} '{}'",
        Msg::NotSavedTrackOrAttach,
        Msg::Skipping,
        path.display()
    )
}
*/

#[inline(always)]
pub(crate) fn warn_not_recognized_media(path: &Path, e: MuxError) {
    warn!(
        "{} '{}': {}. {}",
        Msg::NotRecognizedMedia,
        path.display(),
        e.as_str_localized(),
        Msg::Skipping
    )
}

#[inline(always)]
pub(crate) fn debug_running_command(cmd: &Command, json_args: Option<Vec<String>>) {
    debug!("{}:\n{}", Msg::RunningCommand, CommandDisplay(cmd));

    if let Some(args) = json_args {
        debug!("{}:\n{}", Msg::ArgsInJson, JsonArgsDisplay(&args));
    }
}

#[inline(always)]
pub(crate) fn debug_found_repeat(stem: &OsStr) {
    debug!(
        "{}. {} '{}'",
        Msg::FoundRepeat,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display(),
    )
}

pub(crate) fn debug_media_out_of_range(stem: &OsStr) {
    debug!(
        "{}. {} '{}'",
        Msg::MediaNumOutOfRange,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display(),
    )
}

#[cfg(unix)]
const CONTINUE_CMD: char = '\\';
#[cfg(windows)]
const CONTINUE_CMD: char = '^';

struct CommandDisplay<'a>(&'a Command);

impl<'a> fmt::Display for CommandDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let write_with_continue = |f: &mut fmt::Formatter<'_>, oss: &OsStr| {
            let p: &Path = oss.as_ref();
            write!(f, "\"{}\" {}\n", p.display(), CONTINUE_CMD)
        };

        let write = |f: &mut fmt::Formatter<'_>, oss: &OsStr| {
            let p: &Path = oss.as_ref();
            write!(f, "\"{}\"", p.display())
        };

        let args = self.0.get_args();
        let args_len = args.len();

        if args_len > 0 {
            write_with_continue(f, self.0.get_program())?;
        } else {
            write(f, self.0.get_program())?;
        }

        let last_i = match args_len {
            0 => 0,
            _ => args_len - 1,
        };

        for (i, arg) in args.into_iter().enumerate() {
            if i < last_i {
                write_with_continue(f, arg)?;
            } else {
                write(f, arg)?;
            }
        }

        Ok(())
    }
}

struct JsonArgsDisplay<'a>(&'a [String]);

impl<'a> fmt::Display for JsonArgsDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut last_i = self.0.len();
        if last_i > 0 {
            last_i -= 1;
        }

        for (i, arg) in self.0.into_iter().enumerate() {
            if i < last_i {
                write!(f, "\"{}\" {}\n", arg, CONTINUE_CMD)?;
            } else {
                write!(f, "\"{}\"", arg)?;
            }
        }

        Ok(())
    }
}
