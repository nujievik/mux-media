extern crate mux_media_macros;

mod functions;
mod i18n;
mod macros;
mod traits;
mod types;

/// Field markers for [`MuxConfig`] and [`MediaInfo`].
pub mod markers;

pub use mux_media_macros::IsDefault;

pub use functions::{SEP_BYTES, SEP_STR, ensure_long_path_prefix, ensure_trailing_sep, mux, run};
pub use i18n::Msg;

pub use traits::{
    Field, MaxValue, MutField, MutPathField, MutPathNumField, ParseableArg, ParseableArgs,
    ToJsonArgs, ToMkvmergeArgs, TryFinalizeInit, TryInit, is_default::IsDefault,
};

pub use types::{
    arc_path_buf::ArcPathBuf,
    attachs::{Attachs, FontAttachs, OtherAttachs, attach_type::AttachType, id::AttachID},
    auto_flags::AutoFlags,
    chapters::Chapters,
    char_encoding::{CharEncoding, SubCharset},
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
    mux::{
        config::{MuxConfig, MuxConfigTarget, RawMuxConfig, parseable_args::MuxConfigArg},
        error::{MuxError, MuxErrorKind},
        logger::MuxLogger,
    },
    muxer::Muxer,
    output::Output,
    range::Range,
    specials::Specials,
    target::{Target, group::TargetGroup},
    tools::{Tools, output::ToolOutput, tool::Tool},
    tracks::{
        AudioTracks, ButtonTracks, SubTracks, Tracks, VideoTracks,
        flags::counts::TFlagsCounts,
        flags::flag_type::TFlagType,
        flags::{DefaultTFlags, EnabledTFlags, ForcedTFlags, TFlags},
        id::TrackID,
        langs::TrackLangs,
        names::TrackNames,
        order::TrackOrder,
        track_type::TrackType,
    },
    verbosity::Verbosity,
};

#[doc(hidden)]
pub use traits::{MkvmergeArg, MkvmergeNoArg, ToMkvmergeArg};

#[doc(hidden)]
pub use types::{
    media_info::cache::{attach::CacheMIOfFileAttach, track::CacheMIOfFileTrack},
    mux::current::MuxCurrent,
    retiming::Retiming,
};
