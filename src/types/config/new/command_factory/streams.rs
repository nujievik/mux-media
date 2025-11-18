use super::Blocks;
use crate::{Msg, Streams, undashed};
use clap::{Arg, ArgAction, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn streams(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading("Streams copy")
            .arg(
                Arg::new(undashed!(Audio))
                    .short('a')
                    .long(undashed!(Audio))
                    .value_name("[!]n[,m]...")
                    .help(Msg::HelpAudio.as_str_localized())
                    .value_parser(ValueParser::new(Streams::from_str)),
            )
            .arg(
                Arg::new(undashed!(NoAudio))
                    .short('A')
                    .long(undashed!(NoAudio))
                    .help(Msg::HelpNoAudio.as_str_localized())
                    .action(ArgAction::SetTrue)
                    .conflicts_with(undashed!(Audio)),
            )
            .arg(
                Arg::new(undashed!(Subs))
                    .short('s')
                    .long(undashed!(Subs))
                    .value_name("[!]n[,m]...")
                    .help(Msg::HelpSubs.as_str_localized())
                    .value_parser(ValueParser::new(Streams::from_str)),
            )
            .arg(
                Arg::new(undashed!(NoSubs))
                    .short('S')
                    .long(undashed!(NoSubs))
                    .help(Msg::HelpNoSubs.as_str_localized())
                    .action(ArgAction::SetTrue)
                    .conflicts_with(undashed!(Subs)),
            )
            .arg(
                Arg::new(undashed!(Video))
                    .short('d')
                    .long(undashed!(Video))
                    .value_name("[!]n[,m]...")
                    .help(Msg::HelpVideo.as_str_localized())
                    .value_parser(ValueParser::new(Streams::from_str)),
            )
            .arg(
                Arg::new(undashed!(NoVideo))
                    .short('D')
                    .long(undashed!(NoVideo))
                    .help(Msg::HelpNoVideo.as_str_localized())
                    .action(ArgAction::SetTrue)
                    .conflicts_with(undashed!(Video)),
            )
            .arg(
                Arg::new(undashed!(Fonts))
                    .short('f')
                    .long(undashed!(Fonts))
                    .value_name("[!]n[,m]...")
                    .help(Msg::HelpFonts.as_str_localized())
                    .value_parser(ValueParser::new(Streams::from_str)),
            )
            .arg(
                Arg::new(undashed!(NoFonts))
                    .short('F')
                    .long(undashed!(NoFonts))
                    .help(Msg::HelpNoFonts.as_str_localized())
                    .action(ArgAction::SetTrue)
                    .conflicts_with(undashed!(Fonts)),
            )
            .arg(
                Arg::new(undashed!(Attachs))
                    .short('m')
                    .long(undashed!(Attachs))
                    .value_name("[!]n[,m]...")
                    .help(Msg::HelpAttachs.as_str_localized())
                    .value_parser(ValueParser::new(Streams::from_str)),
            )
            .arg(
                Arg::new(undashed!(NoAttachs))
                    .short('M')
                    .long(undashed!(NoAttachs))
                    .help(Msg::HelpNoAttachs.as_str_localized())
                    .action(ArgAction::SetTrue)
                    .conflicts_with(undashed!(Attachs)),
            );

        self
    }
}
