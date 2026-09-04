mod auto;
mod global;
mod io;
mod other;
mod retiming;
mod streams;
mod target;
mod val_parsers;

use crate::{Config, ConfigTarget};
use clap::{Command, CommandFactory};

impl CommandFactory for Config {
    fn command() -> Command {
        Blocks::new()
            .io()
            .global()
            .auto()
            .streams()
            .target()
            .retiming()
            .other()
            .version()
            .help()
            .0
    }

    fn command_for_update() -> Command {
        Self::command()
    }
}

impl CommandFactory for ConfigTarget {
    fn command() -> Command {
        Blocks::new().target().version().help().0
    }

    fn command_for_update() -> Command {
        Self::command()
    }
}

struct Blocks(pub Command);

impl Blocks {
    // other fn impl Blocks in modules
    fn new() -> Self {
        Self(
            Command::new(env!("CARGO_PKG_NAME"))
                .no_binary_name(true)
                .version(concat!("v", env!("CARGO_PKG_VERSION")))
                .disable_help_flag(true)
                .disable_version_flag(true)
                .override_usage(concat!(env!("CARGO_PKG_NAME"), " [options]")),
        )
    }
}
