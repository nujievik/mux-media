use crate::{Msg, MuxError};
use log::{debug, warn};
use std::{ffi::OsStr, fmt, path::Path, process::Command};

#[inline(always)]
pub(super) fn warn_err_write_json(err: MuxError) {
    warn!(
        "{}: {}. {} CLI ({})",
        Msg::ErrWriteJson,
        err.to_str_localized(),
        Msg::Using,
        Msg::MayFailIfCommandLong
    )
}

#[inline(always)]
pub(super) fn debug_running_command(cmd: &Command, json_args: Option<Vec<String>>) {
    debug!("{}:\n{}", Msg::RunningCommand, CommandDisplay(cmd));

    if let Some(args) = json_args {
        debug!("Args in JSON:\n{}", ArgsDisplay(&args));
    }
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
