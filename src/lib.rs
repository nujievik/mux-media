mod i18n;
mod macros;
mod run;
mod traits;
mod types;

pub use i18n::Msg;
pub use run::run;

pub use traits::{
    CLIArg, CLIArgs, GetField, GetOptField, IsDefault, MaxValue, MkvmergeArg, MkvmergeNoArg,
    SetGetField, SetGetPathField, SetGetPathTrackField, ToJsonArgs, ToMkvmergeArg, ToMkvmergeArgs,
    TryFinalizeInit, TryInit,
};

pub use types::{
    attachs::{Attachs, FontAttachs, OtherAttachs, attach_type::AttachType, id::AttachID},
    chapters::Chapters,
    char_encoding::{CharEncoding, SubCharset},
    extensions::EXTENSIONS,
    input::Input,
    lang_code::LangCode,
    media_info::{
        MediaInfo,
        cache::{CacheMI, CacheState},
        // Get Field MediaInfo markers
        set_get_field::{
            MIAttachsInfo, MICmnStem, MIMkvinfo, MIMkvmergeI, MIPathTail, MIRelativeUpmost,
            MISavedTracks, MISubCharset, MITILang, MITIName, MITITrackIDs, MITargetGroup,
            MITargets, MITracksInfo,
        },
    },
    media_number::MediaNumber,
    mux::{
        // Get Field MuxConfig markers
        config::getters::{
            MCAudioTracks, MCButtonTracks, MCChapters, MCDefaultTFlags, MCEnabledTFlags,
            MCExitOnErr, MCFontAttachs, MCForcedTFlags, MCInput, MCLocale, MCNoJson, MCOffOnPro,
            MCOtherAttachs, MCOutput, MCRetiming, MCSpecials, MCSubTracks, MCTools, MCTrackLangs,
            MCTrackNames, MCVerbosity, MCVideoTracks,
        },
        config::{MuxConfig, RawMuxConfig, cli_args::MuxConfigArg},
        error::{MuxError, MuxErrorKind},
        logger::MuxLogger,
    },
    off_on_pro::OffOnPro,
    output::Output,
    range::Range,
    retiming::Retiming,
    specials::Specials,
    targets::{Target, TargetGroup},
    tools::{Tool, Tools},
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
