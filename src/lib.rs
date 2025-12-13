macro_rules! err {
    ( $($arg:tt)* ) => {
        crate::MuxError::from(format!($($arg)*))
    };
}

mod functions;
mod i18n;
/// Field markers for [`Config`] and [`MediaInfo`].
pub mod markers;
mod traits;
mod types;

pub type Error = MuxError;
pub type Result<T> = std::result::Result<T, MuxError>;

pub use functions::{SEP_BYTES, SEP_STR, ensure_long_path_prefix, ensure_trailing_sep, mux, run};
pub use i18n::Msg;

pub use traits::{
    Field, ToFfmpegArgs, ToJsonArgs, TryFinalizeInit,
    lazy_fields::{LazyField, LazyPathField},
};

pub use types::{
    arc_path_buf::ArcPathBuf,
    auto_flags::AutoFlags,
    chapters::Chapters,
    char_encoding::CharEncoding,
    cli_arg::CliArg,
    codec_id::CodecId,
    config::{Config, ConfigTarget},
    dispositions::{DefaultDispositions, Dispositions, ForcedDispositions, ty::DispositionType},
    duration::Duration,
    extensions::{EXTENSIONS, Extensions},
    file_type::FileType,
    globset_pattern::GlobSetPattern,
    input::{Input, iters::MediaGroupedByStem},
    lang::{Lang, LangCode},
    log_level::LogLevel,
    media_info::{
        MediaInfo,
        cache::{CacheMI, CacheMIOfFile, CacheState},
    },
    media_number::MediaNumber,
    metadata::{LangMetadata, Metadata, NameMetadata},
    mux_current::MuxCurrent,
    mux_error::{MuxError, kind::MuxErrorKind},
    mux_logger::MuxLogger,
    muxer::Muxer,
    output::Output,
    range::RangeUsize,
    retiming::options::RetimingOptions,
    stream::{
        Stream,
        order::{StreamsOrder, StreamsOrderItem},
        streams::Streams,
        ty::StreamType,
    },
    target::Target,
    tools::{Tools, output::ToolOutput, paths::ToolPaths, tool::Tool},
    value::Value,
};

use ffmpeg_next as ffmpeg;
use is_default::IsDefault;

use types::{
    helpers,
    retiming::{RetimedStream, Retiming, RetimingChapter},
    stream::supported::StreamsSupported,
};
