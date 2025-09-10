mod functions;
mod i18n;
mod macros;
mod traits;
mod types;

/// Field markers for [`MuxConfig`] and [`MediaInfo`].
pub mod markers;

pub type Result<T> = std::result::Result<T, MuxError>;

pub use functions::{ensure_long_path_prefix, ensure_trailing_sep, run};
pub use i18n::Msg;

pub use traits::{
    Field, ParseableArg, ParseableArgs, ToFfmpegArgs, ToJsonArgs, ToMkvmergeArgs, TryFinalizeInit,
    lazy_fields::{LazyField, LazyPathField, LazyPathNumField},
};

pub use types::{
    arc_path_buf::ArcPathBuf,
    attachs::{Attachs, FontAttachs, OtherAttachs, attach_type::AttachType, id::AttachID},
    auto_flags::AutoFlags,
    chapters::Chapters,
    char_encoding::{CharEncoding, SubCharset},
    duration::Duration,
    extensions::{EXTENSIONS, Extensions},
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
    specials::Specials,
    target::{Target, group::TargetGroup},
    tools::{Tools, output::ToolOutput, paths::ToolPaths, tool::Tool},
    track_flags::{
        DefaultTrackFlags, EnabledTrackFlags, ForcedTrackFlags, TrackFlags,
        counts::TrackFlagsCounts, flag_type::TrackFlagType,
    },
    track_langs::TrackLangs,
    track_names::TrackNames,
    track_order::{TrackOrder, TrackOrderItem},
    tracks::{AudioTracks, SubTracks, Tracks, VideoTracks, id::TrackID, track_type::TrackType},
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

pub(crate) use types::{
    media_info::cache::{ExternalSegments, track::RawTrackCache},
    mux_config::parseable_args::MuxConfigArg,
    muxer::codecs::MUXER_CODECS,
    retiming::Retiming,
};
