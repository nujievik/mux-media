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
    ($arg:ident) => {
        std::ffi::OsString::from($crate::dashed!($arg))
    };

    (@push_true, $self:ident, $args:ident; $( $field:ident, $arg:ident ),*) => {{
        $(
            if $self.$field {
                $args.push(to_args!($arg));
            }
        )*
    }};

    (@get_values, $self:expr) => {{
        let mut map = std::collections::BTreeSet::<String>::new();

        if let Some(xs) = $self.idxs.as_ref() {
            xs.iter().for_each(|(k, v)| {
                map.insert(format!("{}:{}", k, v));
            });
        }

        if let Some(xs) = $self.ranges.as_ref() {
            xs.iter().for_each(|(k, v)| {
                map.insert(format!("{}:{}", k, v));
            });
        }

        if let Some(xs) = $self.langs.as_ref() {
            xs.iter().for_each(|(k, v)| {
                map.insert(format!("{}:{}", k, v));
            });
        }

        if map.is_empty() {
            $self.single_val.as_ref().map(|v| v.to_string())
        } else {
            Some(map.into_iter().collect::<Vec<_>>().join(","))
        }
    }};
}

pub mod config;
mod functions;
mod i18n;
pub mod media_info;
mod run;
mod traits;
mod types;

pub type Error = MuxError;
pub type Result<T> = std::result::Result<T, MuxError>;

pub use config::{Config, ConfigTarget, fields::dispositions::ty::DispositionType};
pub use functions::{ensure_long_path_prefix, ensure_trailing_sep, mux};
pub use i18n::Msg;
pub use media_info::MediaInfo;
pub use run::run;
pub use traits::{
    Field, ToTxtConfig, TryFinalizeInit,
    lazy_fields::{LazyField, LazyPathField},
};
pub use types::{
    arc_path_buf::ArcPathBuf,
    char_encoding::CharEncoding,
    cli_arg::CliArg,
    codec_id::CodecId,
    duration::Duration,
    extension::Extension,
    globset_pattern::GlobSetPattern,
    lang::{Lang, LangCode},
    media_number::MediaNumber,
    mux_error::MuxError,
    mux_logger::MuxLogger,
    range::RangeUsize,
    stream::{
        Stream,
        order::{StreamsOrder, StreamsOrderItem},
        ty::StreamType,
    },
    target::Target,
    value::Value,
};

static VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));

use ffmpeg_next as ffmpeg;
use is_default::IsDefault;

use config::MediaGroupedByStem;
use functions::add_copy_stream;
use media_info::cache::CacheState;
use types::{
    helpers,
    retiming::{RetimedStream, Retiming, RetimingChapter},
};
