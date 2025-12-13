use super::{Blocks, val_parsers::ConfigParser};
use crate::{LangCode, Msg, undashed};
use clap::{Arg, ArgAction, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn global(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpGlobalOptions.as_str_localized())
            .arg(
                Arg::new(undashed!(Locale))
                    .short('l')
                    .long(undashed!(Locale))
                    .value_name("lng")
                    .help(Msg::HelpLocale.as_str_localized())
                    .value_parser(ValueParser::new(LangCode::from_str)),
            )
            .arg(
                Arg::new(undashed!(Jobs))
                    .short('j')
                    .long(undashed!(Jobs))
                    .value_name("n")
                    .help(Msg::HelpJobs.as_str_localized())
                    .value_parser(clap::value_parser!(u8).range(1..)),
            )
            .arg(
                Arg::new(undashed!(Verbose))
                    .short('v')
                    .long(undashed!(Verbose))
                    .help(Msg::HelpVerbosity.as_str_localized())
                    .action(ArgAction::Count),
            )
            .arg(
                Arg::new(undashed!(Quiet))
                    .short('q')
                    .long(undashed!(Quiet))
                    .help(Msg::HelpQuiet.as_str_localized())
                    .action(ArgAction::SetTrue)
                    .conflicts_with(undashed!(Verbose)),
            )
            .arg(
                Arg::new(undashed!(ExitOnErr))
                    .short('e')
                    .long(undashed!(ExitOnErr))
                    .alias("exit-on-error")
                    .help(Msg::HelpExitOnErr.as_str_localized())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(undashed!(Load))
                    .long(undashed!(Load))
                    .alias("load-config")
                    .value_name("json")
                    .help(Msg::HelpLoad.as_str_localized())
                    .value_parser(ValueParser::new(ConfigParser)),
            )
            .arg(
                Arg::new(undashed!(SaveConfig))
                    .long(undashed!(SaveConfig))
                    .help(Msg::HelpSaveConfig.as_str_localized())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(undashed!(Reencode))
                    .long(undashed!(Reencode))
                    .alias("re-encode")
                    .help(Msg::HelpReencode.as_str_localized())
                    .action(ArgAction::SetTrue),
            );

        self
    }
}
