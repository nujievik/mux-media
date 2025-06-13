use atty::Stream;
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::io::{self, Write};
use std::sync::Once;

static LOGGER: MuxLogger = MuxLogger;
static INIT: Once = Once::new();

pub struct MuxLogger;

impl MuxLogger {
    pub fn init_with_filter(filter: LevelFilter) {
        INIT.call_once(|| {
            log::set_logger(&LOGGER)
                .map(|()| log::set_max_level(filter))
                .unwrap();
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

        match record.level() {
            Level::Error | Level::Warn => {
                let color = if atty::is(Stream::Stderr) {
                    record_level_to_color(record.level())
                } else {
                    ""
                };
                let reset = if color.is_empty() { "" } else { "\x1b[0m" };

                let msg = format!(
                    "{}{}{}: {}\n",
                    color,
                    record_level_to_str(record.level()),
                    reset,
                    record.args()
                );
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

#[inline]
fn record_level_to_color(level: Level) -> &'static str {
    match level {
        Level::Error => "\x1b[31m", // Red
        Level::Warn => "\x1b[33m",  // Yellow/Orange
        _ => "\x1b[32m",            // Green
    }
}

#[inline]
fn record_level_to_str(level: Level) -> &'static str {
    match level {
        Level::Error => "Error",
        Level::Warn => "Warning",
        _ => "",
    }
}
