use super::Blocks;
use super::val_parsers::{InputDirParser, OutputParser};
use crate::{GlobSetPattern, Msg, MuxConfigArg, ParseableArg, Range};
use clap::{Arg, ArgAction, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn io(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpIOOptions.to_str_localized())
            .arg(
                Arg::new(MuxConfigArg::Input.undashed())
                    .short('i')
                    .long(MuxConfigArg::Input.undashed())
                    .value_name("dir")
                    .help(Msg::HelpInput.to_str_localized())
                    .value_parser(ValueParser::new(InputDirParser)),
            )
            .arg(
                Arg::new(MuxConfigArg::Output.undashed())
                    .short('o')
                    .long(MuxConfigArg::Output.undashed())
                    .value_name("out[,put]")
                    .help(Msg::HelpOutput.to_str_localized())
                    .value_parser(ValueParser::new(OutputParser)),
            )
            .arg(
                Arg::new(MuxConfigArg::Range.undashed())
                    .short('r')
                    .long(MuxConfigArg::Range.undashed())
                    .value_name("n[-m]")
                    .help(Msg::HelpRange.to_str_localized())
                    .value_parser(ValueParser::new(Range::<u64>::from_str)),
            )
            .arg(
                Arg::new(MuxConfigArg::Skip.undashed())
                    .long(MuxConfigArg::Skip.undashed())
                    .value_name("n[,m]...")
                    .help(Msg::HelpSkip.to_str_localized())
                    .value_parser(ValueParser::new(GlobSetPattern::from_str)),
            )
            .arg(
                Arg::new(MuxConfigArg::Depth.undashed())
                    .long(MuxConfigArg::Depth.undashed())
                    .value_name("n")
                    .help(Msg::HelpDepth.to_str_localized())
                    .value_parser(clap::value_parser!(u8)),
            )
            .arg(
                Arg::new(MuxConfigArg::Solo.undashed())
                    .long(MuxConfigArg::Solo.undashed())
                    .help(Msg::HelpSolo.to_str_localized())
                    .action(ArgAction::SetTrue),
            );

        self
    }
}
