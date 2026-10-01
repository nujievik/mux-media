macro_rules! err {
    ( $($arg:tt)* ) => {
        crate::MuxError::new_with(format!($($arg)*))
    };
}

macro_rules! some_or {
    ($x:expr, $or:expr) => {
        match $x {
            Some(x) => x,
            None => $or,
        }
    };
}

macro_rules! deref_singleton_tuple_struct {
    ($wrapper:ty, $inner:ty) => {
        impl std::ops::Deref for $wrapper {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    ($wrapper:ty, $inner:ty, @from_str) => {
        deref_singleton_tuple_struct!($wrapper, $inner);

        impl std::str::FromStr for $wrapper {
            type Err = $crate::MuxError;

            fn from_str(s: &str) -> $crate::Result<Self> {
                s.parse::<$inner>().map(Self).map_err(Into::into)
            }
        }
    };
}

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
                if !is_first {
                    $writer.write(b",")?;
                }

                $writer.write(k.format_into($buf).as_bytes())?;
                $writer.write(b":")?;
                $writer.write(v.as_str().as_bytes())?;

                is_first = false;
            }
        }

        if let Some(xs) = $values.ranges.as_ref() {
            for (k, v) in xs {
                if !is_first {
                    $writer.write(b",")?;
                }

                crate::helpers::write_range($writer, k, $buf)?;
                $writer.write(b":")?;
                $writer.write(v.as_str().as_bytes())?;

                is_first = false;
            }
        }

        if let Some(xs) = $values.langs.as_ref() {
            for (k, v) in xs {
                if !is_first {
                    $writer.write(b",")?;
                }

                $writer.write(k.as_str().as_bytes())?;
                $writer.write(b":")?;
                $writer.write(v.as_str().as_bytes())?;

                is_first = false;
            }
        }

        $writer.write(b"\n")?;
    }};
}

pub mod config;
pub mod media_info;

mod helpers;
mod i18n;
mod run;
mod traits;
mod types;

pub type Error = MuxError;
pub type Result<T> = std::result::Result<T, MuxError>;

pub type FxIndexMap<K, V> = indexmap::IndexMap<K, V, rustc_hash::FxBuildHasher>;
pub type FxIndexSet<T> = indexmap::IndexSet<T, rustc_hash::FxBuildHasher>;

pub use rustc_hash::{FxHashMap, FxHashSet};
pub use subtitle_lines::Time;

pub use config::{Config, ConfigTarget, fields::dispositions::ty::DispositionType};
pub use helpers::{ensure_long_path_prefix, mux};
pub use i18n::Msg;
pub use media_info::MediaInfo;
pub use run::run;
pub use traits::{
    Field, ToArgs, TryFinalizeInit,
    lazy_fields::{LazyField, LazyPathField},
};
pub use types::{
    arc_path_buf::ArcPathBuf,
    char_encoding::CharEncoding,
    cli_arg::CliArg,
    codec_id::CodecId,
    extension::Extension,
    globset_pattern::GlobSetPattern,
    lang::{Lang, LangCode},
    media_number::MediaNumber,
    mux_error::{MuxError, MuxErrorParse},
    mux_logger::MuxLogger,
    my_bool::Bool,
    range::RangeUsize,
    stream::{Stream, ty::StreamType},
    streams_order::{StreamsOrder, StreamsOrderItem},
    target::Target,
    value::Value,
};

static VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));

use ffmpeg_next as ffmpeg;
use is_default::IsDefault;

use config::MediaGroupedByStem;
use config::fields::chapters::ConfigChaptersTimeRange;
use helpers::*;
use media_info::cache::CacheState;
use types::retiming::{Retiming, RetimingChapter};
