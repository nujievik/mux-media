mod into;
mod new;

use crate::{MuxLogger, ffmpeg};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MuxError {
    #[error("ffmpeg: {0}")]
    Ffmpeg(#[from] ffmpeg::Error),

    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("parse: {0}")]
    Parse(#[from] MuxErrorParse),

    #[error("{0}")]
    Other(#[from] MuxErrorOther),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MuxErrorParse {
    #[error("config: {0}")]
    Config(#[from] clap::Error),

    #[error("float: {0}")]
    Float(#[from] std::num::ParseFloatError),

    #[error("int: {0}")]
    Int(#[from] std::num::ParseIntError),

    #[error("subtitle lines: {0}")]
    SubtitleLines(#[from] subtitle_lines::Error),
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct MuxErrorOther {
    code: i32,
    message: String,
}

impl MuxError {
    pub fn code(&self) -> i32 {
        match self {
            MuxError::Parse(MuxErrorParse::Config(e)) => e.exit_code(),
            MuxError::Other(e) => e.code,
            _ => 1,
        }
    }

    /// Returns `true` if the error code is non-zero.
    pub fn use_stderr(&self) -> bool {
        !matches!(self.code(), 0)
    }

    /// Prints formatted and colored error to stdout or stderr according to its error kind.
    pub fn print(&self) {
        if let MuxError::Parse(MuxErrorParse::Config(e)) = self {
            if let Ok(()) = e.print() {
                return;
            }
        }

        if self.use_stderr() {
            let prefix = MuxLogger::color_prefix(log::Level::Error);
            eprintln!("{}{}", prefix, self);
            eprintln!("\n{}", MuxLogger::try_help());
        } else {
            println!("{}", self);
        }
    }
}
