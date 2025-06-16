mod i18n;
mod macros;
mod run;
mod traits;
mod types;

pub use i18n::Msg;
pub use run::run;

pub use traits::{
    CLIArg, CLIArgs, GetField, GetOptField, IsDefault, MaxValue, SetGetField, SetGetPathField,
    SetGetPathTrackField, ToMkvmergeArg, ToMkvmergeArgs, TryFinalizeInit, TryInit,
};

pub use types::{
    attachs::{Attachs, FontAttachs, OtherAttachs, attach_type::AttachType, id::AttachID},
    chapters::Chapters,
    extensions::EXTENSIONS,
    input::Input,
    lang_code::LangCode,
    // Get Field MediaInfo markers
    media_info::set_get_field::{
        MIAttachsInfo, MICharEncoding, MICmnTrackOrder, MIMkvinfo, MIMkvmergeI, MIPathTail,
        MIRelativeUpmost, MISavedTrackNums, MITILang, MITIName, MITargetGroup, MITargets,
        MITracksInfo,
    },
    media_info::{CacheState, MICache, MediaInfo},
    media_number::MediaNumber,
    mux::{
        // Get Field MuxConfig markers
        config::getters::{
            MCAudioTracks, MCButtonTracks, MCChapters, MCDefaultTFlags, MCEnabledTFlags,
            MCExitOnErr, MCFontAttachs, MCForcedTFlags, MCInput, MCLocale, MCOffOnPro,
            MCOtherAttachs, MCOutput, MCRetiming, MCSpecials, MCSubTracks, MCTools, MCTrackLangs,
            MCTrackNames, MCVerbosity, MCVideoTracks,
        },
        config::{MuxConfig, RawMuxConfig},
        error::{MuxError, MuxErrorKind},
        logger::MuxLogger,
    },
    off_on_pro::OffOnPro,
    os_helpers,
    output::Output,
    range::Range,
    retiming::Retiming,
    specials::Specials,
    targets::{Target, TargetGroup},
    tools::{Tool, Tools},
    tracks::{
        AudioTracks, ButtonTracks, DefaultTFlags, EnabledTFlags, ForcedTFlags, SubTracks, TFlags,
        TrackLangs, TrackNames, Tracks, VideoTracks, flags::counts::TFlagsCounts, id::TrackID,
        order::TrackOrder, track_type::TrackType,
    },
    verbosity::Verbosity,
};
