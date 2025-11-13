macro_rules! err {
    ( $($arg:tt)* ) => {
        crate::MuxError::from(format!($($arg)*))
    };
}

mod functions;
mod i18n;
mod traits;
mod types;

/// Field markers for [`MuxConfig`] and [`MediaInfo`].
pub mod markers;

pub use ffmpeg_next as ffmpeg;

pub type Result<T> = std::result::Result<T, MuxError>;

pub use functions::{ensure_long_path_prefix, ensure_trailing_sep, run};
pub use i18n::Msg;

pub use traits::{
    Field, ToFfmpegArgs, ToJsonArgs, TryFinalizeInit,
    lazy_fields::{LazyField, LazyPathField, LazyPathNumField},
};

pub use types::{
    arc_path_buf::ArcPathBuf,
    attachs::{Attachs, FontAttachs, OtherAttachs, id::AttachID, ty::AttachType},
    auto_flags::AutoFlags,
    chapters::Chapters,
    char_encoding::{CharEncoding, SubCharset},
    cli_arg::CliArg,
    codec_id::CodecID,
    duration::Duration,
    extensions::{EXTENSIONS, Extensions},
    ffmpeg_stream::FfmpegStream,
    file_type::FileType,
    globset_pattern::GlobSetPattern,
    input::Input,
    lang_code::LangCode,
    media_info::{
        MediaInfo,
        cache::{CacheMI, CacheState},
    },
    media_number::MediaNumber,
    mux_config::{MuxConfig, MuxConfigTarget},
    mux_current::MuxCurrent,
    mux_error::{MuxError, kind::MuxErrorKind},
    mux_logger::MuxLogger,
    muxer::Muxer,
    output::Output,
    range::RangeU64,
    retiming::options::RetimingOptions,
    target::{Target, group::TargetGroup},
    tools::{Tools, output::ToolOutput, paths::ToolPaths, tool::Tool},
    track_flags::{
        DefaultTrackFlags, ForcedTrackFlags, TrackFlags, counts::TrackFlagsCounts,
        ty::TrackFlagType,
    },
    track_langs::TrackLangs,
    track_names::TrackNames,
    track_order::{TrackOrder, TrackOrderItem, TrackOrderItemRetimed},
    tracks::{AudioTracks, SubTracks, Tracks, VideoTracks, id::TrackID, ty::TrackType},
    value::Value,
    verbosity::Verbosity,
};

#[doc(hidden)]
pub use is_default::IsDefault;

#[doc(hidden)]
pub use functions::{SEP_BYTES, SEP_STR};

#[doc(hidden)]
pub use types::media_info::cache::{
    CacheMICommon, CacheMIOfFile, attach::CacheMIOfFileAttach, track::CacheMIOfFileTrack,
};

use types::{muxer::supported::SupportedTracks, retiming::Retiming};
