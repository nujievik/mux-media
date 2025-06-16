use super::Blocks;
use crate::{CLIArg, CLIArgs, LangCode, MuxConfig, OffOnPro, Verbosity};
use clap::{Arg, ArgAction, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn global(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading("Global options")
            .arg(
                Arg::new(<Verbosity as CLIArgs>::Arg::Verbose.as_long())
                    .short('v')
                    .long(<Verbosity as CLIArgs>::Arg::Verbose.as_long())
                    .help("Increase verbosity")
                    .action(ArgAction::Count),
            )
            .arg(
                Arg::new(<Verbosity as CLIArgs>::Arg::Quiet.as_long())
                    .short('q')
                    .long(<Verbosity as CLIArgs>::Arg::Quiet.as_long())
                    .help("Suppress logging")
                    .action(ArgAction::SetTrue)
                    .conflicts_with(<Verbosity as CLIArgs>::Arg::Verbose.as_long()),
            )
            .arg(
                Arg::new(<MuxConfig as CLIArgs>::Arg::Locale.as_long())
                    .short('l')
                    .long(<MuxConfig as CLIArgs>::Arg::Locale.as_long())
                    .value_name("lng")
                    .help("Locale language (on logging and sort)")
                    .value_parser(ValueParser::new(LangCode::from_str)),
            )
            .arg(
                Arg::new(<MuxConfig as CLIArgs>::Arg::ExitOnErr.as_long())
                    .short('e')
                    .long(<MuxConfig as CLIArgs>::Arg::ExitOnErr.as_long())
                    .alias("exit-on-error")
                    .help("Skip mux for next files if err")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(<OffOnPro as CLIArgs>::Arg::Pro.as_long())
                    .short('p')
                    .long("pro")
                    .alias("pro-mode")
                    .help("Off all auto 'Off on Pro options'")
                    .action(ArgAction::SetTrue),
            );

        self
    }
}
