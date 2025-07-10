use super::Blocks;
use crate::{CLIArg, GlobSetPattern, Msg, MuxConfigArg};
use clap::{Arg, ArgAction, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn retiming(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpRetimingOptions.to_str_localized())
            .arg(
                Arg::new(MuxConfigArg::RmSegments.as_long())
                    .long(MuxConfigArg::RmSegments.as_long())
                    .alias("remove-segments")
                    .value_name("n[,m]...")
                    .help(Msg::HelpRmSegments.to_str_localized())
                    .hide(true)
                    .value_parser(ValueParser::new(GlobSetPattern::from_str)),
            )
            .arg(
                Arg::new(MuxConfigArg::NoLinked.as_long())
                    .long(MuxConfigArg::NoLinked.as_long())
                    .help(Msg::HelpNoLinked.to_str_localized())
                    .hide(true)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(MuxConfigArg::LessRetiming.as_long())
                    .long(MuxConfigArg::LessRetiming.as_long())
                    .help(Msg::HelpLessRetiming.to_str_localized())
                    .hide(true)
                    .action(ArgAction::SetTrue),
            );

        self
    }
}
