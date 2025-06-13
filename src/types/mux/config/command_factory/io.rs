use super::Blocks;
use super::val_parsers::{InputDirParser, OutputParser, patterns_parser};
use crate::{CLIArg, CLIArgs, Input, MuxConfig, Range};
use clap::{Arg, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn io(mut self) -> Self {
        self.cmd = self
            .cmd
            .next_help_heading("I/O options")
            .arg(
                Arg::new(<Input as CLIArgs>::Arg::Input.as_long())
                    .short('i')
                    .long(<Input as CLIArgs>::Arg::Input.as_long())
                    .value_name("dir")
                    .help("File search start directory")
                    .value_parser(ValueParser::new(InputDirParser)),
            )
            .arg(
                Arg::new(<MuxConfig as CLIArgs>::Arg::Output.as_long())
                    .short('o')
                    .long(<MuxConfig as CLIArgs>::Arg::Output.as_long())
                    .value_name("out[,put]")
                    .help("Output paths pattern: out{num}[put]")
                    .value_parser(ValueParser::new(OutputParser)),
            )
            .arg(
                Arg::new(<Input as CLIArgs>::Arg::Range.as_long())
                    .short('r')
                    .long(<Input as CLIArgs>::Arg::Range.as_long())
                    .value_name("n[-m]")
                    .help("Number range of file names to mux")
                    .value_parser(ValueParser::new(Range::<u32>::from_str)),
            )
            .arg(
                Arg::new(<Input as CLIArgs>::Arg::Skip.as_long())
                    .long(<Input as CLIArgs>::Arg::Skip.as_long())
                    .value_name("n[,m]...")
                    .help("Skip files with path patterns")
                    .value_parser(ValueParser::new(patterns_parser)),
            )
            .arg(
                Arg::new(<Input as CLIArgs>::Arg::Up.as_long())
                    .long(<Input as CLIArgs>::Arg::Up.as_long())
                    .value_name("n")
                    .help("Max directory levels to search up")
                    .value_parser(clap::value_parser!(u8)),
            )
            .arg(
                Arg::new(<Input as CLIArgs>::Arg::Check.as_long())
                    .long(<Input as CLIArgs>::Arg::Check.as_long())
                    .value_name("n")
                    .help("Max files to check per level while up search")
                    .value_parser(clap::value_parser!(u16).range(1..)),
            )
            .arg(
                Arg::new(<Input as CLIArgs>::Arg::Down.as_long())
                    .long(<Input as CLIArgs>::Arg::Down.as_long())
                    .value_name("n")
                    .help("Max directory levels to search down")
                    .value_parser(clap::value_parser!(u8)),
            );

        self
    }
}
