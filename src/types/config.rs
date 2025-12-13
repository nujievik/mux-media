pub(crate) mod fields;
mod mux;
pub(crate) mod new;
mod to_json_args;

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{
    AutoFlags, Chapters, DefaultDispositions, ForcedDispositions, Input, IsDefault, LangCode,
    LangMetadata, LogLevel, Muxer, NameMetadata, Output, RetimingOptions, Streams, Target,
    ToolPaths,
};
use std::collections::HashMap;

/// Mux configuration.
///
/// # Warning
///
/// This struct is not fully initialized after construction.
/// You **must** call [`Config::try_finalize_init`] before using some methods.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Config {
    pub input: Input,
    pub output: Output,
    pub locale: LangCode,
    pub jobs: u8,
    pub log_level: LogLevel,
    pub exit_on_err: bool,
    pub save_config: bool,
    pub reencode: bool,
    pub auto_flags: AutoFlags,
    pub streams: Streams,
    pub chapters: Chapters,
    pub defaults: DefaultDispositions,
    pub forceds: ForcedDispositions,
    pub names: NameMetadata,
    pub langs: LangMetadata,
    pub retiming_options: RetimingOptions,
    pub targets: Option<HashMap<Target, ConfigTarget>>,
    pub tool_paths: ToolPaths,
    pub muxer: Muxer,
    pub is_output_constructed_from_input: bool,
}

/// Mux configuration for a target.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
#[non_exhaustive]
pub struct ConfigTarget {
    pub streams: Option<Streams>,
    pub chapters: Option<Chapters>,
    pub defaults: Option<DefaultDispositions>,
    pub forceds: Option<ForcedDispositions>,
    pub names: Option<NameMetadata>,
    pub langs: Option<LangMetadata>,
}

impl Config {
    const JSON_NAME: &str = "mux-media.json";
    const JOBS_DEFAULT: u8 = 1;
}
