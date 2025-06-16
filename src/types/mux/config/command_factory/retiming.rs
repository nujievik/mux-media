use super::{Blocks, val_parsers::patterns_parser};
use crate::{CLIArg, CLIArgs, Retiming};
use clap::{Arg, ArgAction, builder::ValueParser};

impl Blocks {
    pub fn retiming(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading("Retiming options")
            .arg(
                Arg::new(<Retiming as CLIArgs>::Arg::RmSegments.as_long())
                    .long(<Retiming as CLIArgs>::Arg::RmSegments.as_long())
                    .alias("remove-segments")
                    .value_name("n[,m]...")
                    .help("Remove segments with name patterns")
                    .value_parser(ValueParser::new(patterns_parser)),
            )
            .arg(
                Arg::new(<Retiming as CLIArgs>::Arg::NoLinked.as_long())
                    .long(<Retiming as CLIArgs>::Arg::NoLinked.as_long())
                    .help("Remove linked segments")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(<Retiming as CLIArgs>::Arg::LessRetiming.as_long())
                    .long(<Retiming as CLIArgs>::Arg::LessRetiming.as_long())
                    .help("No retiming if linked segments outside main")
                    .action(ArgAction::SetTrue),
            );

        self
    }
}
