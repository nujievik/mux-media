use crate::{Msg, MuxError};
use log::{debug, trace, warn};
use std::{ffi::OsStr, fmt, path::Path, process::Command};

#[inline(always)]
pub(crate) fn warn_err_write_json(err: MuxError) {
    warn!(
        "{}: {}. {} CLI ({})",
        Msg::ErrWriteJson,
        err.to_str_localized(),
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
pub(crate) fn warn_not_out_save_any(out: &Path) {
    warn!(
        "{} '{}'. {}",
        Msg::NotOutSaveAny,
        out.display(),
        Msg::Skipping
    )
}

#[inline(always)]
pub(crate) fn warn_not_out_change(out: &Path) {
    warn!(
        "{}. {} '{}'",
        Msg::NotOutChange,
        Msg::Skipping,
        out.display()
    )
}

#[inline(always)]
pub(crate) fn warn_not_saved_track_or_attach(path: &Path) {
    warn!(
        "{}. {} '{}'",
        Msg::NotSavedTrackOrAttach,
        Msg::Skipping,
        path.display()
    )
}

#[inline(always)]
pub(crate) fn warn_not_recognized_media(path: &Path, e: MuxError) {
    warn!(
        "{} '{}': {}. {}",
        Msg::NotRecognizedMedia,
        path.display(),
        e.to_str_localized(),
        Msg::Skipping
    )
}

#[inline(always)]
pub(crate) fn debug_no_ext_media(stem: &OsStr) {
    debug!(
        "{}. {} '{}'",
        Msg::NoExtMediaFound,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display()
    )
}

#[inline(always)]
pub(crate) fn debug_running_command(cmd: &Command, json_args: Option<Vec<String>>) {
    debug!("{}:\n{}", Msg::RunningCommand, CommandDisplay(cmd));

    if let Some(args) = json_args {
        debug!("{}:\n{}", Msg::ArgsInJson, ArgsDisplay(&args));
    }
}

#[inline(always)]
pub(crate) fn trace_found_repeat(stem: &OsStr) {
    trace!(
        "{}. {} '{}'",
        Msg::FoundRepeat,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display(),
    )
}

struct CommandDisplay<'a>(&'a Command);

impl<'a> fmt::Display for CommandDisplay<'a> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut try_write = |oss: &OsStr| -> fmt::Result {
            let p: &Path = oss.as_ref();
            write!(f, "\"{}\" ", p.display())
        };

        try_write(self.0.get_program())?;

        for arg in self.0.get_args() {
            try_write(arg)?;
        }

        Ok(())
    }
}

struct ArgsDisplay<'a>(&'a [String]);

impl<'a> fmt::Display for ArgsDisplay<'a> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for arg in self.0 {
            write!(f, "\"{}\" ", arg)?;
        }
        Ok(())
    }
}
