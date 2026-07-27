use super::Blocks;
use crate::{
    DefaultDispositions, ForcedDispositions, LangMetadata, Msg, NameMetadata, Streams, undashed,
};
use clap::{Arg, ArgAction, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn target(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpTargetOptions.as_str_localized())
            .arg(
                Arg::new(undashed!(Target))
                    .short('t')
                    .long(undashed!(Target))
                    .value_name("trg")
                    .help(Msg::HelpTargetHelp.as_str_localized())
                    .trailing_var_arg(true)
                    .allow_hyphen_values(true)
                    .num_args(1..),
            )
            .arg(
                Arg::new(undashed!(ListTargets))
                    .long(undashed!(ListTargets))
                    .help(Msg::HelpListTargets.as_str_localized())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(undashed!(Streams))
                    .long(undashed!(Streams))
                    .value_name("[!]n[,m]...")
                    .help(Msg::HelpStreams.as_str_localized())
                    .value_parser(ValueParser::new(Streams::from_str)),
            )
            .arg(
                Arg::new(undashed!(NoStreams))
                    .long(undashed!(NoStreams))
                    .help(Msg::HelpNoStreams.as_str_localized())
                    .action(ArgAction::SetTrue)
                    .conflicts_with(undashed!(Streams)),
            )
            /*
            .arg(
                Arg::new(undashed!(Chapters))
                    .short('c')
                    .long(undashed!(Chapters))
                    .value_name("file")
                    .help(Msg::HelpChapters.as_str_localized())
                    .value_parser(ValueParser::new(ChaptersParser)),
            )
            */
            .arg(
                Arg::new(undashed!(NoChapters))
                    .short('C')
                    .long(undashed!(NoChapters))
                    .help(Msg::HelpNoChapters.as_str_localized())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(undashed!(Defaults))
                    .long(undashed!(Defaults))
                    .value_name("[n:]B[,m:B]...")
                    .help(Msg::HelpDefaults.as_str_localized())
                    .value_parser(ValueParser::new(DefaultDispositions::from_str)),
            )
            .arg(
                Arg::new(undashed!(MaxDefaults))
                    .long(undashed!(MaxDefaults))
                    .value_name("n")
                    .help(Msg::HelpMaxDefaults.as_str_localized())
                    .value_parser(clap::value_parser!(usize)),
            )
            .arg(
                Arg::new(undashed!(Forceds))
                    .long(undashed!(Forceds))
                    .value_name("[n:]B[,m:B]...")
                    .help(Msg::HelpForceds.as_str_localized())
                    .value_parser(ValueParser::new(ForcedDispositions::from_str)),
            )
            .arg(
                Arg::new(undashed!(MaxForceds))
                    .long(undashed!(MaxForceds))
                    .value_name("n")
                    .help(Msg::HelpMaxForceds.as_str_localized())
                    .value_parser(clap::value_parser!(usize)),
            )
            .arg(
                Arg::new(undashed!(Names))
                    .long(undashed!(Names))
                    .value_name("[n:]N[,m:N]...")
                    .help(Msg::HelpNames.as_str_localized())
                    .value_parser(ValueParser::new(NameMetadata::from_str)),
            )
            .arg(
                Arg::new(undashed!(Langs))
                    .long(undashed!(Langs))
                    .value_name("[n:]L[,m:L]...")
                    .help(Msg::HelpLangs.as_str_localized())
                    .value_parser(ValueParser::new(LangMetadata::from_str)),
            );

        self
    }
}
