mod into;
mod new;

use crate::{MuxLogger, ffmpeg};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MuxError {
    #[error("config parse: {0}")]
    ConfigParse(#[from] clap::Error),

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
    #[error("float: {0}")]
    Float(#[from] std::num::ParseFloatError),

    #[error("int: {0}")]
    Int(#[from] std::num::ParseIntError),

    #[error("srt subtitles: {0}")]
    SrtSubtitles(#[from] rsubs_lib::SRTError),

    #[error("ssa subtitles: {0}")]
    SsaSubtitles(#[from] rsubs_lib::SSAError),

    #[error("vtt subtitles: {0}")]
    VttSubtitles(#[from] rsubs_lib::VTTError),
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
            MuxError::ConfigParse(e) => e.exit_code(),
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
        if let MuxError::ConfigParse(e) = self {
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
