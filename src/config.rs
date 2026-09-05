pub(crate) mod fields;
pub(crate) mod new;
mod to_args;

pub use fields::{
    MarkConfigChapters, MarkConfigDefaults, MarkConfigForceds, MarkConfigLangMetadata,
    MarkConfigStreams, MarkConfigTitleMetadata,
    auto_flags::ConfigAutoFlags,
    chapters::ConfigChapters,
    dispositions::ConfigDispositions,
    input::ConfigInput,
    log_level::ConfigLogLevel,
    metadata::{ConfigLangMetadata, ConfigMetadata, ConfigTitleMetadata},
    output::ConfigOutput,
    retiming::{ConfigRetiming, ConfigRetimingParts},
    streams::ConfigStreams,
};

pub(crate) use fields::input::{InputType, iters::MediaGroupedByStem};

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{IsDefault, LangCode, Target};
use std::{collections::HashMap, path::PathBuf};

/// A configuration.
///
/// # Warning
///
/// This struct is not fully initialized after construction.
/// You **must** call [`Config::try_finalize_init`] before using some methods.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Config {
    pub input: ConfigInput,
    pub output: ConfigOutput,
    pub locale: LangCode,
    pub overwrite: bool,
    pub jobs: u8,
    pub log_level: ConfigLogLevel,
    pub exit_on_err: bool,
    pub save_config: bool,
    pub auto_flags: ConfigAutoFlags,
    pub streams: ConfigStreams,
    pub chapters: ConfigChapters,
    pub defaults: ConfigDispositions,
    pub forceds: ConfigDispositions,
    pub titles: ConfigTitleMetadata,
    pub langs: ConfigLangMetadata,
    pub retiming: ConfigRetiming,
    pub targets: Option<HashMap<Target, ConfigTarget>>,
    pub is_output_constructed_from_input: bool,
}

/// A configuration for a [`Target`].
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
#[non_exhaustive]
pub struct ConfigTarget {
    pub streams: Option<ConfigStreams>,
    pub chapters: Option<ConfigChapters>,
    pub defaults: Option<ConfigDispositions>,
    pub forceds: Option<ConfigDispositions>,
    pub titles: Option<ConfigTitleMetadata>,
    pub langs: Option<ConfigLangMetadata>,
}

impl Config {
    const JOBS_DEFAULT: u8 = 1;

    fn txt_path(base: impl Into<PathBuf>) -> PathBuf {
        let mut p = base.into();
        p.push(concat!(".", env!("CARGO_PKG_NAME")));
        p.push("config.txt");
        p
    }
}
