use log::{Level, LevelFilter, Log, Metadata, Record};
use std::{
    io::{self, Write},
    sync::{LazyLock, Once},
};
use supports_color::{Stream, on};

static LOGGER: MuxLogger = MuxLogger;
static INIT: Once = Once::new();

static STDERR_ON_COLOR: LazyLock<bool> = LazyLock::new(|| on(Stream::Stderr).is_some());
static STDOUT_ON_COLOR: LazyLock<bool> = LazyLock::new(|| on(Stream::Stdout).is_some());

/// A logger imlementing the [`log`] logger.
pub struct MuxLogger;

impl MuxLogger {
    /// Initializes the global logger with the given [`LevelFilter`].
    ///
    /// This is safe to call multiple times; initialization will only occur once.
    /// ```
    /// use mux_media::MuxLogger;
    /// let f = log::LevelFilter::Warn;
    /// MuxLogger::init_with_filter(f);
    /// MuxLogger::init_with_filter(f);
    /// ```
    pub fn init_with_filter(filter: LevelFilter) {
        INIT.call_once(|| {
            log::set_logger(&LOGGER)
                .map(|()| log::set_max_level(filter))
                .expect("Unexpected repeat log::set_logger()");
        });
    }

    /// Returns a colored or plain log level prefix for stderr or stdout output.
    ///
    /// Only `Error`, `Warn`, `Debug`, and `Trace` levels return a non-empty string.
    /// `Info` returns an empty string and logs as-is.
    ///
    /// - ANSI color codes are applied to `Error` and `Warn` if stderr supports color.
    /// - ANSI color codes are applied to `Debug` and `Trace` if stdout supports color.
    pub(crate) fn color_prefix(level: Level) -> &'static str {
        match level {
            Level::Error if *STDERR_ON_COLOR => "\x1b[31mError\x1b[0m: ",
            Level::Error => "Error: ",
            Level::Warn if *STDERR_ON_COLOR => "\x1b[33mWarning\x1b[0m: ",
            Level::Warn => "Warning: ",
            Level::Debug if *STDOUT_ON_COLOR => "\x1b[34mDebug\x1b[0m: ",
            Level::Debug => "Debug: ",
            Level::Trace if *STDOUT_ON_COLOR => "\x1b[35mTrace\x1b[0m: ",
            Level::Trace => "Trace: ",
            _ => "",
        }
    }

    /// Returns a colored or plain clap-style try help string.
    pub(crate) fn try_help() -> &'static str {
        if *STDERR_ON_COLOR {
            "For more information, try '\x1b[34m--help\x1b[0m'."
        } else {
            "For more information, try '--help'."
        }
    }
}

impl Log for MuxLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = record.level();

        let msg = format!("{}{}\n", Self::color_prefix(level), record.args());
        let msg = msg.as_bytes();

        match level {
            Level::Error | Level::Warn => {
                let _ = io::stderr()
                    .write_all(msg)
                    .or_else(|_| io::stdout().write_all(msg));
            }
            _ => {
                let _ = io::stdout().write_all(msg);
            }
        }
    }

    fn flush(&self) {}
}
