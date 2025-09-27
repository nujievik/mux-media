use super::Blocks;
use super::val_parsers::{InputDirParser, OutputParser};
use crate::{GlobSetPattern, Msg, RangeU64, undashed};
use clap::{Arg, ArgAction, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn io(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpIOOptions.as_str_localized())
            .arg(
                Arg::new(undashed!(Input))
                    .short('i')
                    .long(undashed!(Input))
                    .value_name("dir")
                    .help(Msg::HelpInput.as_str_localized())
                    .value_parser(ValueParser::new(InputDirParser)),
            )
            .arg(
                Arg::new(undashed!(Output))
                    .short('o')
                    .long(undashed!(Output))
                    .value_name("out[,put]")
                    .help(Msg::HelpOutput.as_str_localized())
                    .value_parser(ValueParser::new(OutputParser)),
            )
            .arg(
                Arg::new(undashed!(Range))
                    .short('r')
                    .long(undashed!(Range))
                    .value_name("n[-m]")
                    .help(Msg::HelpRange.as_str_localized())
                    .value_parser(ValueParser::new(RangeU64::from_str)),
            )
            .arg(
                Arg::new(undashed!(Skip))
                    .long(undashed!(Skip))
                    .value_name("n[,m]...")
                    .help(Msg::HelpSkip.as_str_localized())
                    .value_parser(ValueParser::new(GlobSetPattern::from_str)),
            )
            .arg(
                Arg::new(undashed!(Depth))
                    .long(undashed!(Depth))
                    .value_name("n")
                    .help(Msg::HelpDepth.as_str_localized())
                    .value_parser(clap::value_parser!(u8)),
            )
            .arg(
                Arg::new(undashed!(Solo))
                    .long(undashed!(Solo))
                    .help(Msg::HelpSolo.as_str_localized())
                    .action(ArgAction::SetTrue),
            );

        self
    }
}
