use super::super::cli_args::MuxConfigArg;
use super::{Blocks, val_parsers::ConfigParser};
use crate::{CLIArg, LangCode, Msg};
use clap::{Arg, ArgAction, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn global(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpGlobalOptions.to_str_localized())
            .arg(
                Arg::new(MuxConfigArg::Locale.as_long())
                    .long(MuxConfigArg::Locale.as_long())
                    .value_name("lng")
                    .help(Msg::HelpLocale.to_str_localized())
                    .value_parser(ValueParser::new(LangCode::from_str)),
            )
            .arg(
                Arg::new(MuxConfigArg::Verbose.as_long())
                    .short('v')
                    .long(MuxConfigArg::Verbose.as_long())
                    .help(Msg::HelpVerbosity.to_str_localized())
                    .action(ArgAction::Count),
            )
            .arg(
                Arg::new(MuxConfigArg::Quiet.as_long())
                    .short('q')
                    .long(MuxConfigArg::Quiet.as_long())
                    .help(Msg::HelpQuiet.to_str_localized())
                    .action(ArgAction::SetTrue)
                    .conflicts_with(MuxConfigArg::Verbose.as_long()),
            )
            .arg(
                Arg::new(MuxConfigArg::Config.as_long())
                    .short('c')
                    .long(MuxConfigArg::Config.as_long())
                    .value_name("json")
                    .help(Msg::HelpConfig.to_str_localized())
                    .value_parser(ValueParser::new(ConfigParser)),
            )
            .arg(
                Arg::new(MuxConfigArg::NoConfig.as_long())
                    .short('C')
                    .long(MuxConfigArg::NoConfig.as_long())
                    .action(ArgAction::SetTrue)
                    .help(Msg::HelpNoConfig.to_str_localized())
                    .conflicts_with(MuxConfigArg::Config.as_long()),
            )
            .arg(
                Arg::new(MuxConfigArg::ExitOnErr.as_long())
                    .short('e')
                    .long(MuxConfigArg::ExitOnErr.as_long())
                    .alias("exit-on-error")
                    .help(Msg::HelpExitOnErr.to_str_localized())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(MuxConfigArg::Pro.as_long())
                    .short('p')
                    .long(MuxConfigArg::Pro.as_long())
                    .alias("pro-mode")
                    .help(Msg::HelpPro.to_str_localized())
                    .action(ArgAction::SetTrue),
            );

        self
    }
}
