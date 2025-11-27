mod is_supported_copy;
mod new;

use crate::{
    Dispositions, IsDefault, LangMetadata, MediaInfo, MuxCurrent, NameMetadata, Result,
    StreamsOrder, ToFfmpegArgs, Tool, ToolOutput, i18n::logs,
};
use std::{ffi::OsString, path::Path};
use strum_macros::Display;

/// Supported muxers.
#[derive(Copy, Clone, Debug, Default, Display, PartialEq, IsDefault)]
#[non_exhaustive]
pub enum Muxer {
    AVI,
    MP4,
    #[default]
    Matroska,
    Webm,
}

impl Muxer {
    /// Returns main output extension for the muxer.
    ///
    /// ```
    /// # use mux_media::Muxer;
    /// assert_eq!(Muxer::AVI.as_ext(), "avi");
    /// assert_eq!(Muxer::MP4.as_ext(), "mp4");
    /// assert_eq!(Muxer::Matroska.as_ext(), "mkv");
    /// assert_eq!(Muxer::Webm.as_ext(), "webm");
    /// ```
    #[inline]
    pub const fn as_ext(self) -> &'static str {
        match self {
            Self::AVI => "avi",
            Self::MP4 => "mp4",
            Self::Matroska => "mkv",
            Self::Webm => "webm",
        }
    }

    /// Runs muxing for single media output.
    #[inline]
    pub fn mux_current(self, mi: &mut MediaInfo, out: &Path) -> MuxCurrent<ToolOutput> {
        let mut args = Vec::<OsString>::new();

        if let Err(e) = try_append_args(&mut args, mi) {
            return MuxCurrent::Err(e);
        }

        if args.len() < 3 {
            logs::warn_not_out_save_any(out);
            return MuxCurrent::Continue;
        }

        args.push(out.into());

        mi.tools.run(Tool::Ffmpeg, &args).into()
    }
}

fn try_append_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) -> Result<()> {
    StreamsOrder::append_ffmpeg_args(args, mi)?;
    Dispositions::append_ffmpeg_args(args, mi)?;
    LangMetadata::append_ffmpeg_args(args, mi)?;
    NameMetadata::append_ffmpeg_args(args, mi)?;

    Ok(())
}
