mod command_factory;
pub(crate) mod fields;
mod mux;
mod new;
mod to_json_args;

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{
    AudioTracks, AutoFlags, Chapters, DefaultTrackFlags, FontAttachs, ForcedTrackFlags, Input,
    IsDefault, LangCode, Muxer, OtherAttachs, Output, RetimingOptions, SubTracks, Target,
    ToolPaths, TrackLangs, TrackNames, Verbosity, VideoTracks,
};
use std::collections::HashMap;

/// Contains mux configuration.
///
/// # Warning
///
/// This struct is not fully initialized after construction.
/// You **must** call [MuxConfig::try_finalize_init] before using some methods.
#[derive(Clone, Debug, PartialEq)]
pub struct MuxConfig {
    pub input: Input,
    pub output: Output,
    pub locale: LangCode,
    pub verbosity: Verbosity,
    pub exit_on_err: bool,
    pub save_config: bool,
    pub reencode: bool,
    pub threads: u8,
    pub auto_flags: AutoFlags,
    pub audio_tracks: AudioTracks,
    pub sub_tracks: SubTracks,
    pub video_tracks: VideoTracks,
    pub chapters: Chapters,
    pub font_attachs: FontAttachs,
    pub other_attachs: OtherAttachs,
    pub default_track_flags: DefaultTrackFlags,
    pub forced_track_flags: ForcedTrackFlags,
    pub track_names: TrackNames,
    pub track_langs: TrackLangs,
    pub retiming: RetimingOptions,
    pub targets: Option<HashMap<Target, MuxConfigTarget>>,
    pub tool_paths: ToolPaths,
    pub muxer: Muxer,
    pub is_output_constructed_from_input: bool,
}

/// Contains mux settings for target.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct MuxConfigTarget {
    pub audio_tracks: Option<AudioTracks>,
    pub sub_tracks: Option<SubTracks>,
    pub video_tracks: Option<VideoTracks>,
    pub chapters: Option<Chapters>,
    pub font_attachs: Option<FontAttachs>,
    pub other_attachs: Option<OtherAttachs>,
    pub default_track_flags: Option<DefaultTrackFlags>,
    pub forced_track_flags: Option<ForcedTrackFlags>,
    pub track_names: Option<TrackNames>,
    pub track_langs: Option<TrackLangs>,
}
