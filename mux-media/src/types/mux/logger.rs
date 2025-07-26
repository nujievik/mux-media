use log::{Level, LevelFilter, Log, Metadata, Record};
use once_cell::sync::Lazy;
use std::io::{self, Write};
use std::sync::Once;
use supports_color::Stream;

static LOGGER: MuxLogger = MuxLogger;
static INIT: Once = Once::new();

static STDERR_ON_COLOR: Lazy<bool> = Lazy::new(|| supports_color::on(Stream::Stderr).is_some());
static STDOUT_ON_COLOR: Lazy<bool> = Lazy::new(|| supports_color::on(Stream::Stdout).is_some());

/// Logger type used throughout the crate. Based on the [`log`] crate.
pub struct MuxLogger;

impl MuxLogger {
    /// Initializes the global logger with the given [`LevelFilter`].
    ///
    /// This is safe to call multiple times; initialization will only occur once.
    /// Internally uses [`log::set_logger`] wrapped in [`std::sync::Once`] to ensure
    /// only the first call sets the logger.
    ///
    /// If logger initialization somehow fails (which should be impossible due to `Once`),
    /// a fallback message is printed to stderr.
    pub fn init_with_filter(filter: LevelFilter) {
        INIT.call_once(|| {
            log::set_logger(&LOGGER)
                .map(|()| log::set_max_level(filter))
                .unwrap_or_else(|_| eprintln!("Unexpected repeat set_logger()"));
        });
    }

    /// Returns a colored or plain log level prefix for stderr or stdout output.
    ///
    /// Only `Error`, `Warn`, `Debug`, and `Trace` levels return a non-empty string.
    /// `Info` returns an empty string and logs as-is.
    ///
    /// - ANSI color codes are applied to `Error` and `Warn` if stderr supports color.
    /// - ANSI color codes are applied to `Debug` and `Trace` if stdout supports color.
    pub(crate) fn get_color_prefix(level: Level) -> &'static str {
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

        let msg = format!("{}{}\n", Self::get_color_prefix(level), record.args());
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
