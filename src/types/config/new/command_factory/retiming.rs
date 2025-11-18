use super::Blocks;
use crate::{GlobSetPattern, Msg, undashed};
use clap::{Arg, ArgAction, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn retiming(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpRetimingOptions.as_str_localized())
            .arg(
                Arg::new(undashed!(RmSegments))
                    .long(undashed!(RmSegments))
                    .alias("remove-segments")
                    .value_name("n[,m]...")
                    .help(Msg::HelpRmSegments.as_str_localized())
                    .value_parser(ValueParser::new(GlobSetPattern::from_str)),
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
