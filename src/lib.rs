mod i18n;
mod macros;
mod run;
mod traits;
mod types;

/// Field markers for [`MuxConfig`] and [`MediaInfo`].
pub mod markers;

pub use i18n::Msg;
pub use run::run;

pub use traits::{
    CLIArg, CLIArgs, GetField, GetOptField, IsDefault, MaxValue, MkvmergeArg, MkvmergeNoArg,
    SetGetField, SetGetPathField, SetGetPathTrackField, ToJsonArgs, ToMkvmergeArg, ToMkvmergeArgs,
    TryFinalizeInit, TryInit,
};

pub use types::{
    arc_path_buf::ArcPathBuf,
    attachs::{Attachs, FontAttachs, OtherAttachs, attach_type::AttachType, id::AttachID},
    chapters::Chapters,
    char_encoding::{CharEncoding, SubCharset},
    extensions::{EXTENSIONS, Extensions},
    globset_pattern::GlobSetPattern,
    input::Input,
    lang_code::LangCode,
    media_info::{
        MediaInfo,
        cache::{CacheMI, CacheState},
    },
    media_number::MediaNumber,
    mux::{
        config::{MuxConfig, MuxConfigTarget, RawMuxConfig, cli_args::MuxConfigArg},
        error::{MuxError, MuxErrorKind},
        logger::MuxLogger,
    },
    output::Output,
    pro_flags::ProFlags,
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
pub use types::media_info::cache::{attach::CacheMIOfFileAttach, track::CacheMIOfFileTrack};

#[doc(hidden)]
pub use types::{mux::config::getters::MCRetiming, retiming::Retiming};
