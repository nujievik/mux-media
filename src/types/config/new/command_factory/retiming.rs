use super::Blocks;
use crate::{Msg, RetimingOptions, undashed};
use clap::{Arg, ArgAction, builder::ValueParser};

impl Blocks {
    pub fn retiming(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpRetimingOptions.as_str_localized())
            .arg(
                Arg::new(undashed!(Parts))
                    .long(undashed!(Parts))
                    .value_name("[!]n[,m]...")
                    .help(Msg::HelpParts.as_str_localized())
                    .value_parser(ValueParser::new(RetimingOptions::from_str_parts)),
            )
            .arg(
                Arg::new(undashed!(NoLinked))
                    .long(undashed!(NoLinked))
                    .help(Msg::HelpNoLinked.as_str_localized())
                    .action(ArgAction::SetTrue),
            );

        self
    }
}
