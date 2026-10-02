macro_rules! to_args {
    ($writer:ident, $arg:ident) => {
        $writer
            .write($crate::dashed!($arg).as_bytes())
            .and_then(|_| $writer.write(b"\n"))
    };

    ($writer:ident, $arg:expr, @v) => {
        $writer.write($arg).and_then(|_| $writer.write(b"\n"))
    };

    ($writer:ident, $values:expr, @write_map, $buf:ident) => {{
        let mut is_first = true;

        if let Some(v) = $values.single_val.as_ref() {
            $writer.write(v.as_str().as_bytes())?;
        }

        if let Some(xs) = $values.idxs.as_ref() {
            for (k, v) in xs {
                crate::helpers::write_part_of_arg($writer, &mut is_first, |$writer| {
                    $writer.write(k.format_into($buf).as_bytes())?;
                    $writer.write(b":")?;
                    $writer.write(v.as_str().as_bytes())
                })?;
            }
        }

        if let Some(xs) = $values.ranges.as_ref() {
            for (k, v) in xs {
                crate::helpers::write_part_of_arg($writer, &mut is_first, |$writer| {
                    crate::helpers::write_range($writer, k, $buf)?;
                    $writer.write(b":")?;
                    $writer.write(v.as_str().as_bytes())
                })?;
            }
        }

        if let Some(xs) = $values.langs.as_ref() {
            for (k, v) in xs {
                crate::helpers::write_part_of_arg($writer, &mut is_first, |$writer| {
                    $writer.write(k.as_str().as_bytes())?;
                    $writer.write(b":")?;
                    $writer.write(v.as_str().as_bytes())
                })?;
            }
        }

        $writer.write(b"\n")?;
    }};
}

pub(crate) mod fields;
pub(crate) mod new;
mod to_args;

pub use fields::{
    MarkConfigChapters, MarkConfigDefaults, MarkConfigForceds, MarkConfigLangMetadata,
    MarkConfigStreams, MarkConfigSubsEncoding, MarkConfigTitleMetadata,
    auto_flags::ConfigAutoFlags,
    chapters::ConfigChapters,
    dispositions::ConfigDispositions,
    input::ConfigInput,
    log_level::ConfigLogLevel,
    metadata::{ConfigLangMetadata, ConfigMetadata, ConfigTitleMetadata},
    output::ConfigOutput,
    retiming::{ConfigRetiming, ConfigRetimingParts},
    streams::ConfigStreams,
    subs_encoding::ConfigSubsEncoding,
};

pub(crate) use fields::input::{InputType, iters::MediaGroupedByStem};

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{FxIndexMap, IsDefault, LangCode, Target};
use std::path::PathBuf;

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
    pub subs_encoding: ConfigSubsEncoding,

    pub retiming: ConfigRetiming,
    pub target_configs: FxIndexMap<Target, ConfigTarget>,
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
    pub subs_encoding: Option<ConfigSubsEncoding>,
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
