use super::super::cli_args::MuxConfigArg;
use super::Blocks;
use super::val_parsers::{InputDirParser, OutputParser, patterns_parser};
use crate::{CLIArg, Msg, Range};
use clap::{Arg, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn io(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpIOOptions.to_str_localized())
            .arg(
                Arg::new(MuxConfigArg::Input.as_long())
                    .short('i')
                    .long(MuxConfigArg::Input.as_long())
                    .value_name("dir")
                    .help(Msg::HelpInput.to_str_localized())
                    .value_parser(ValueParser::new(InputDirParser)),
            )
            .arg(
                Arg::new(MuxConfigArg::Output.as_long())
                    .short('o')
                    .long(MuxConfigArg::Output.as_long())
                    .value_name("out[,put]")
                    .help(Msg::HelpOutput.to_str_localized())
                    .value_parser(ValueParser::new(OutputParser)),
            )
            .arg(
                Arg::new(MuxConfigArg::Range.as_long())
                    .short('r')
                    .long(MuxConfigArg::Range.as_long())
                    .value_name("n[-m]")
                    .help(Msg::HelpRange.to_str_localized())
                    .value_parser(ValueParser::new(Range::<u64>::from_str)),
            )
            .arg(
                Arg::new(MuxConfigArg::Skip.as_long())
                    .long(MuxConfigArg::Skip.as_long())
                    .value_name("n[,m]...")
                    .help(Msg::HelpSkip.to_str_localized())
                    .value_parser(ValueParser::new(patterns_parser)),
            )
            .arg(
                Arg::new(MuxConfigArg::Up.as_long())
                    .long(MuxConfigArg::Up.as_long())
                    .value_name("n")
                    .help(Msg::HelpUp.to_str_localized())
                    .value_parser(clap::value_parser!(u8)),
            )
            .arg(
                Arg::new(MuxConfigArg::Check.as_long())
                    .long(MuxConfigArg::Check.as_long())
                    .value_name("n")
                    .help(Msg::HelpCheck.to_str_localized())
                    .value_parser(clap::value_parser!(u16).range(1..)),
            )
            .arg(
                Arg::new(MuxConfigArg::Down.as_long())
                    .long(MuxConfigArg::Down.as_long())
                    .value_name("n")
                    .help(Msg::HelpDown.to_str_localized())
                    .value_parser(clap::value_parser!(u8)),
            );

        self
    }
}
