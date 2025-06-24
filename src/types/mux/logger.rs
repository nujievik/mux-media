use log::{Level, LevelFilter, Log, Metadata, Record};
use once_cell::sync::Lazy;
use std::io::{self, Write};
use std::sync::Once;
use supports_color::Stream;

static LOGGER: MuxLogger = MuxLogger;
static INIT: Once = Once::new();
static STDERR_ON_COLOR: Lazy<bool> = Lazy::new(|| supports_color::on(Stream::Stderr).is_some());

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

        match level {
            Level::Error | Level::Warn => {
                let msg = format!("{}{}\n", get_stderr_color_prefix(level), record.args());
                let msg = msg.as_bytes();
                let _ = io::stderr()
                    .write_all(msg)
                    .or_else(|_| io::stdout().write_all(msg));
            }
            _ => {
                let msg = format!("{}\n", record.args());
                let _ = io::stdout().write_all(msg.as_bytes());
            }
        }
    }

    fn flush(&self) {}
}

/// Returns a colored or plain log level prefix for stderr output.
///
/// Only `Error` and `Warn` levels return a non-empty string. ANSI color codes
/// are applied if stderr supports them.
pub(crate) fn get_stderr_color_prefix(level: log::Level) -> &'static str {
    match level {
        Level::Error if *STDERR_ON_COLOR => "\x1b[31mError\x1b[0m: ",
        Level::Error => "Error: ",
        Level::Warn if *STDERR_ON_COLOR => "\x1b[33mWarning\x1b[0m: ",
        Level::Warn => "Warning: ",
        _ => "",
    }
}
