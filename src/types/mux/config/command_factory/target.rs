use super::{Blocks, val_parsers::ChaptersParser};
use crate::{
    AudioTracks, ButtonTracks, CLIArg, CLIArgs, Chapters, DefaultTFlags, EnabledTFlags,
    FontAttachs, ForcedTFlags, OtherAttachs, Specials, SubTracks, TrackLangs, TrackNames,
    VideoTracks,
};
use clap::{Arg, ArgAction, builder::ValueParser};
use std::str::FromStr;

impl Blocks {
    pub fn target(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading("Target options")
            .arg(
                Arg::new("target_help")
                    .short('t')
                    .long("target <trg> [options]")
                    .help("Set next options for target")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("list_targets")
                    .long("list-targets")
                    .help("Show supported targets")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(<AudioTracks as CLIArgs>::Arg::Audio.as_long())
                    .short('a')
                    .long(<AudioTracks as CLIArgs>::Arg::Audio.as_long())
                    .aliases(&["audio-tracks", "atracks"])
                    .value_name("[!]n[,m]...")
                    .help("[!]Copy audio tracks n,m etc.")
                    .value_parser(ValueParser::new(AudioTracks::from_str)),
            )
            .arg(
                Arg::new(<AudioTracks as CLIArgs>::Arg::NoAudio.as_long())
                    .short('A')
                    .long(<AudioTracks as CLIArgs>::Arg::NoAudio.as_long())
                    .alias("noaudio")
                    .help("Don't copy any audio track")
                    .action(ArgAction::SetTrue)
                    .conflicts_with(<AudioTracks as CLIArgs>::Arg::Audio.as_long()),
            )
            .arg(
                Arg::new(<SubTracks as CLIArgs>::Arg::Subs.as_long())
                    .short('s')
                    .long(<SubTracks as CLIArgs>::Arg::Subs.as_long())
                    .aliases(&["subtitle-tracks", "subtitles", "sub-tracks", "stracks"])
                    .value_name("[!]n[,m]...")
                    .help("[!]Copy subtitle tracks n,m etc.")
                    .value_parser(ValueParser::new(SubTracks::from_str)),
            )
            .arg(
                Arg::new(<SubTracks as CLIArgs>::Arg::NoSubs.as_long())
                    .short('S')
                    .long(<SubTracks as CLIArgs>::Arg::NoSubs.as_long())
                    .aliases(&["no-subtitles", "nosubtitles", "nosubs"])
                    .help("Don't copy any subtitle track")
                    .action(ArgAction::SetTrue)
                    .conflicts_with(<SubTracks as CLIArgs>::Arg::Subs.as_long()),
            )
            .arg(
                Arg::new(<VideoTracks as CLIArgs>::Arg::Video.as_long())
                    .short('d')
                    .long(<VideoTracks as CLIArgs>::Arg::Video.as_long())
                    .aliases(&["video-tracks", "vtracks"])
                    .value_name("[!]n[,m]...")
                    .help("[!]Copy video tracks n,m etc.")
                    .value_parser(ValueParser::new(VideoTracks::from_str)),
            )
            .arg(
                Arg::new(<VideoTracks as CLIArgs>::Arg::NoVideo.as_long())
                    .short('D')
                    .long(<VideoTracks as CLIArgs>::Arg::NoVideo.as_long())
                    .alias("novideo")
                    .help("Don't copy any video track")
                    .action(ArgAction::SetTrue)
                    .conflicts_with(<VideoTracks as CLIArgs>::Arg::Video.as_long()),
            )
            .arg(
                Arg::new(<ButtonTracks as CLIArgs>::Arg::Buttons.as_long())
                    .short('b')
                    .long(<ButtonTracks as CLIArgs>::Arg::Buttons.as_long())
                    .aliases(&["button-tracks", "btracks"])
                    .value_name("[!]n[,m]...")
                    .help("[!]Copy button tracks n,m etc.")
                    .value_parser(ValueParser::new(ButtonTracks::from_str)),
            )
            .arg(
                Arg::new(<ButtonTracks as CLIArgs>::Arg::NoButtons.as_long())
                    .short('B')
                    .long(<ButtonTracks as CLIArgs>::Arg::NoButtons.as_long())
                    .alias("nobuttons")
                    .help("Don't copy any button track")
                    .action(ArgAction::SetTrue)
                    .conflicts_with(<ButtonTracks as CLIArgs>::Arg::Buttons.as_long()),
            )
            .arg(
                Arg::new(<Chapters as CLIArgs>::Arg::Chapters.as_long())
                    .short('c')
                    .long(<Chapters as CLIArgs>::Arg::Chapters.as_long())
                    .value_name("chp")
                    .help("Chapters info from chp file")
                    .value_parser(ValueParser::new(ChaptersParser)),
            )
            .arg(
                Arg::new(<Chapters as CLIArgs>::Arg::NoChapters.as_long())
                    .short('C')
                    .long(<Chapters as CLIArgs>::Arg::NoChapters.as_long())
                    .help("Don't keep chapters")
                    .action(ArgAction::SetTrue)
                    .conflicts_with(<Chapters as CLIArgs>::Arg::Chapters.as_long()),
            )
            .arg(
                Arg::new(<FontAttachs as CLIArgs>::Arg::Fonts.as_long())
                    .short('f')
                    .long(<FontAttachs as CLIArgs>::Arg::Fonts.as_long())
                    .value_name("[!]n[,m]...")
                    .help("[!]Copy font attachments n,m etc.")
                    .value_parser(ValueParser::new(FontAttachs::from_str)),
            )
            .arg(
                Arg::new(<FontAttachs as CLIArgs>::Arg::NoFonts.as_long())
                    .short('F')
                    .long(<FontAttachs as CLIArgs>::Arg::NoFonts.as_long())
                    .alias("nofonts")
                    .help("Don't copy any font attachment")
                    .action(ArgAction::SetTrue)
                    .conflicts_with(<FontAttachs as CLIArgs>::Arg::Fonts.as_long()),
            )
            .arg(
                Arg::new(<OtherAttachs as CLIArgs>::Arg::Attachs.as_long())
                    .short('m')
                    .long(<OtherAttachs as CLIArgs>::Arg::Attachs.as_long())
                    .alias("attachments")
                    .value_name("[!]n[,m]...")
                    .help("[!]Copy other attachments n,m etc.")
                    .value_parser(ValueParser::new(OtherAttachs::from_str)),
            )
            .arg(
                Arg::new(<OtherAttachs as CLIArgs>::Arg::NoAttachs.as_long())
                    .short('M')
                    .long(<OtherAttachs as CLIArgs>::Arg::NoAttachs.as_long())
                    .aliases(&["no-attachments", "noattachments", "noattachs"])
                    .help("Don't copy any other attachment")
                    .action(ArgAction::SetTrue)
                    .conflicts_with(<OtherAttachs as CLIArgs>::Arg::Attachs.as_long()),
            )
            .arg(
                Arg::new(<DefaultTFlags as CLIArgs>::Arg::Defaults.as_long())
                    .long(<DefaultTFlags as CLIArgs>::Arg::Defaults.as_long())
                    .aliases(&["default-track-flags", "default-tracks"])
                    .value_name("[n:]B[,m:B]...")
                    .help("Bool default-track-flags")
                    .value_parser(ValueParser::new(DefaultTFlags::from_str)),
            )
            .arg(
                Arg::new(<DefaultTFlags as CLIArgs>::Arg::LimDefaults.as_long())
                    .long(<DefaultTFlags as CLIArgs>::Arg::LimDefaults.as_long())
                    .value_name("n")
                    .help("Max true default-track-flags in auto")
                    .value_parser(clap::value_parser!(u64)),
            )
            .arg(
                Arg::new(<ForcedTFlags as CLIArgs>::Arg::Forceds.as_long())
                    .long(<ForcedTFlags as CLIArgs>::Arg::Forceds.as_long())
                    .aliases(&["forced-display-flags", "forced-tracks"])
                    .value_name("[n:]B[,m:B]...")
                    .help("Bool forced-display-flags")
                    .value_parser(ValueParser::new(ForcedTFlags::from_str)),
            )
            .arg(
                Arg::new(<ForcedTFlags as CLIArgs>::Arg::LimForceds.as_long())
                    .long(<ForcedTFlags as CLIArgs>::Arg::LimForceds.as_long())
                    .value_name("n")
                    .help("Max true forced-display-flags in auto")
                    .value_parser(clap::value_parser!(u64)),
            )
            .arg(
                Arg::new(<EnabledTFlags as CLIArgs>::Arg::Enableds.as_long())
                    .long(<EnabledTFlags as CLIArgs>::Arg::Enableds.as_long())
                    .alias("track-enabled-flags")
                    .value_name("[n:]B[,m:B]...")
                    .help("Bool track-enabled-flags")
                    .value_parser(ValueParser::new(EnabledTFlags::from_str)),
            )
            .arg(
                Arg::new(<EnabledTFlags as CLIArgs>::Arg::LimEnableds.as_long())
                    .long(<EnabledTFlags as CLIArgs>::Arg::LimEnableds.as_long())
                    .value_name("n")
                    .help("Max true track-enabled-flags in auto")
                    .value_parser(clap::value_parser!(u64)),
            )
            .arg(
                Arg::new(<TrackNames as CLIArgs>::Arg::Names.as_long())
                    .long(<TrackNames as CLIArgs>::Arg::Names.as_long())
                    .alias("track-names")
                    .value_name("[n:]N[,m:N]...")
                    .help("Track names")
                    .value_parser(ValueParser::new(TrackNames::from_str)),
            )
            .arg(
                Arg::new(<TrackLangs as CLIArgs>::Arg::Langs.as_long())
                    .long(<TrackLangs as CLIArgs>::Arg::Langs.as_long())
                    .alias("languages")
                    .value_name("[n:]L[,m:L]...")
                    .help("Track languages")
                    .value_parser(ValueParser::new(TrackLangs::from_str)),
            )
            .arg(
                Arg::new(<Specials as CLIArgs>::Arg::Specials.as_long())
                    .long(<Specials as CLIArgs>::Arg::Specials.as_long())
                    .value_name("\"n[ m]...\"")
                    .allow_hyphen_values(true)
                    .help("Set unpresented mkvmerge options")
                    .value_parser(ValueParser::new(Specials::from_str)),
            );

        self
    }
}
