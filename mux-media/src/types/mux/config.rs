mod command_factory;
pub(crate) mod fields;
mod from_arg_matches;
pub(crate) mod parseable_args;
mod raw;
mod to_json_args;
mod try_init;

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{
    AudioTracks, AutoFlags, ButtonTracks, Chapters, DefaultTFlags, EnabledTFlags, FontAttachs,
    ForcedTFlags, Input, LangCode, Muxer, OtherAttachs, Output, Specials, SubTracks, Target, Tool,
    Tools, TrackLangs, TrackNames, Verbosity, VideoTracks,
};
use std::{collections::HashMap, ffi::OsString};

/// Contains raw user-defined mux settings.
pub struct RawMuxConfig {
    pub locale: Option<LangCode>,
    pub list_targets: bool,
    pub list_containers: bool,
    pub list_langs: bool,
    pub run_command: Option<(Tool, Vec<OsString>)>,
    pub args: Vec<OsString>,
    pub trg_args: Option<HashMap<Target, Vec<OsString>>>,
}

/// Contains mux settings.
///
/// # Warning
///
/// This struct is not fully initialized after construction.
/// You **must** call [Self::try_finalize_init] before using some methods.
pub struct MuxConfig {
    input: Input,
    output: Output,
    locale: LangCode,
    verbosity: Verbosity,
    exit_on_err: bool,
    json: bool,
    reencode: bool,
    auto_flags: AutoFlags,
    audio_tracks: AudioTracks,
    sub_tracks: SubTracks,
    video_tracks: VideoTracks,
    button_tracks: ButtonTracks,
    chapters: Chapters,
    font_attachs: FontAttachs,
    other_attachs: OtherAttachs,
    default_t_flags: DefaultTFlags,
    forced_t_flags: ForcedTFlags,
    enabled_t_flags: EnabledTFlags,
    track_names: TrackNames,
    track_langs: TrackLangs,
    specials: Specials,
    targets: Option<HashMap<Target, MuxConfigTarget>>,
    user_tools: bool,
    tools: Tools,
    muxer: Muxer,
    is_output_constructed_from_input: bool,
}

/// Contains mux settings for target.
pub struct MuxConfigTarget {
    audio_tracks: Option<AudioTracks>,
    sub_tracks: Option<SubTracks>,
    video_tracks: Option<VideoTracks>,
    button_tracks: Option<ButtonTracks>,
    chapters: Option<Chapters>,
    font_attachs: Option<FontAttachs>,
    other_attachs: Option<OtherAttachs>,
    default_t_flags: Option<DefaultTFlags>,
    forced_t_flags: Option<ForcedTFlags>,
    enabled_t_flags: Option<EnabledTFlags>,
    track_names: Option<TrackNames>,
    track_langs: Option<TrackLangs>,
    specials: Option<Specials>,
}
