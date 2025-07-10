use log::{Level, LevelFilter, Log, Metadata, Record};
use once_cell::sync::Lazy;
use std::io::{self, Write};
use std::sync::Once;
use supports_color::Stream;

static LOGGER: MuxLogger = MuxLogger;
static INIT: Once = Once::new();

static STDERR_ON_COLOR: Lazy<bool> = Lazy::new(|| supports_color::on(Stream::Stderr).is_some());
static STDOUT_ON_COLOR: Lazy<bool> = Lazy::new(|| supports_color::on(Stream::Stdout).is_some());

pub struct MuxLogger;

impl MuxLogger {
    pub fn init_with_filter(filter: LevelFilter) {
        INIT.call_once(|| {
            // set_logger() returns Err if a logger has already been set.
            // That means Err is excluded because used INIT.call_once()
            log::set_logger(&LOGGER)
                .map(|()| log::set_max_level(filter))
                .unwrap_or_else(|_| eprintln!("Unexpected repeat set_logger()"));
        });
    }

    /// Returns a colored or plain log level prefix for stderr or stdout output.
    ///
    /// Only `Error`, `Warn`, `Debug`, and `Trace` levels return a non-empty string.
    /// ANSI color codes for `Error` and `Warn` are applied if stderr supports them.
    /// ANSI color codes for `Debug` and `Trace` are applied if stdout supports them.
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
