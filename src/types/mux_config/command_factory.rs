mod auto;
mod global;
mod io;
mod other;
mod retiming;
mod target;
mod val_parsers;

use super::{MuxConfig, MuxConfigTarget};
use clap::{Command, CommandFactory};

impl CommandFactory for MuxConfig {
    fn command() -> Command {
        Blocks::new()
            .io()
            .global()
            .auto()
            .target()
            .retiming()
            .other()
            .version()
            .help()
            .into()
    }

    fn command_for_update() -> Command {
        Self::command()
    }
}

impl CommandFactory for MuxConfigTarget {
    fn command() -> Command {
        Blocks::new().target().version().help().into()
    }

    fn command_for_update() -> Command {
        Self::command()
    }
}

impl From<Blocks> for Command {
    fn from(blocks: Blocks) -> Command {
        blocks.0
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
