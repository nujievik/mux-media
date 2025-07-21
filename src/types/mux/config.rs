pub(crate) mod cli_args;
mod command_factory;
mod from_arg_matches;
pub(crate) mod getters;
mod raw;
mod to_json_args;
mod try_init;

use crate::{
    AudioTracks, ButtonTracks, Chapters, DefaultTFlags, EnabledTFlags, FontAttachs, ForcedTFlags,
    Input, LangCode, OtherAttachs, Output, ProFlags, Retiming, Specials, SubTracks, Target, Tool,
    Tools, TrackLangs, TrackNames, Verbosity, VideoTracks,
};
use std::{collections::HashMap, ffi::OsString};

/// Contains raw user-defined mux settings.
pub struct RawMuxConfig {
    pub locale: Option<LangCode>,
    pub list_langs: bool,
    pub list_targets: bool,
    pub call_tool: Option<(Tool, Vec<OsString>)>,
    pub args: Vec<OsString>,
    pub trg_args: Option<HashMap<Target, Vec<OsString>>>,
}

/// Contains mux settings.
///
/// # Warning
///
/// This struct is not fully initialized after construction.
/// You **must** call `self.try_finalize_init` before using some methods.
pub struct MuxConfig {
    input: Input,
    output: Output,
    locale: LangCode,
    verbosity: Verbosity,
    no_json: bool,
    exit_on_err: bool,
    pro_flags: ProFlags,
    retiming: Retiming,
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
