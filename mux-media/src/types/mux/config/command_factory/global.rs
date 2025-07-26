use super::{Blocks, val_parsers::ConfigParser};
use crate::{LangCode, Msg, MuxConfigArg, ParseableArg};
use clap::{Arg, ArgAction, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn global(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpGlobalOptions.to_str_localized())
            .arg(
                Arg::new(MuxConfigArg::Locale.undashed())
                    .long(MuxConfigArg::Locale.undashed())
                    .value_name("lng")
                    .help(Msg::HelpLocale.to_str_localized())
                    .value_parser(ValueParser::new(LangCode::from_str)),
            )
            .arg(
                Arg::new(MuxConfigArg::Verbose.undashed())
                    .short('v')
                    .long(MuxConfigArg::Verbose.undashed())
                    .help(Msg::HelpVerbosity.to_str_localized())
                    .action(ArgAction::Count),
            )
            .arg(
                Arg::new(MuxConfigArg::Quiet.undashed())
                    .short('q')
                    .long(MuxConfigArg::Quiet.undashed())
                    .help(Msg::HelpQuiet.to_str_localized())
                    .action(ArgAction::SetTrue)
                    .conflicts_with(MuxConfigArg::Verbose.undashed()),
            )
            .arg(
                Arg::new(MuxConfigArg::Load.undashed())
                    .long(MuxConfigArg::Load.undashed())
                    .alias("load-json")
                    .value_name("json")
                    .help(Msg::HelpLoad.to_str_localized())
                    .value_parser(ValueParser::new(ConfigParser)),
            )
            .arg(
                Arg::new(MuxConfigArg::Json.undashed())
                    .short('j')
                    .long(MuxConfigArg::Json.undashed())
                    .help(Msg::HelpJson.to_str_localized())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(MuxConfigArg::ExitOnErr.undashed())
                    .short('e')
                    .long(MuxConfigArg::ExitOnErr.undashed())
                    .alias("exit-on-error")
                    .help(Msg::HelpExitOnErr.to_str_localized())
                    .action(ArgAction::SetTrue),
            );

        self
    }
}
