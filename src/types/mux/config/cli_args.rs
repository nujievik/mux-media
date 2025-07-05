use super::MuxConfig;
use crate::{CLIArg, CLIArgs};

macro_rules! cli_args {
    ($ty:ident, $enum_arg:ident;
    $( $arg:ident => $long:expr ),* ) => {
        impl CLIArgs for $ty {
            type Arg = $enum_arg;
        }

        #[derive(Copy, Clone)]
        pub enum $enum_arg {
            $( $arg ),*
        }

        impl CLIArg for $enum_arg {
            fn as_long(self) -> &'static str {
                match self {
                    $( Self::$arg => $long ),*
                }
            }
        }
    };
}

cli_args!(
    MuxConfig, MuxConfigArg;
    Input => "input",
    Output => "output",
    Range => "range",
    Skip => "skip",
    Up => "up",
    Check => "check",
    Down => "down",
    Locale => "locale",
    Verbose => "verbose",
    Quiet => "quiet",
    Config => "config",
    NoConfig => "no-config",
    ExitOnErr => "exit-on-err",
    Pro => "pro",
    HelpAddDefaults => "add-defaults / --no-add-defaults",
    AddDefaults => "add-defaults",
    NoAddDefaults => "no-add-defaults",
    HelpAddForceds => "add-forceds / --no-add-forceds",
    AddForceds => "add-forceds",
    NoAddForceds => "no-add-forceds",
    HelpAddEnableds => "add-enableds / --no-add-enableds",
    AddEnableds => "add-enableds",
    NoAddEnableds => "no-add-enableds",
    HelpAddNames => "add-names / --no-add-names",
    AddNames => "add-names",
    NoAddNames => "no-add-names",
    HelpAddLangs => "add-langs / --no-add-langs",
    AddLangs => "add-langs",
    NoAddLangs => "no-add-langs",
    HelpSortFonts => "sort-fonts / --no-sort-fonts",
    SortFonts => "sort-fonts",
    NoSortFonts => "no-sort-fonts",
    RmSegments => "rm-segments",
    NoLinked => "no-linked",
    LessRetiming => "less-retiming",
    Target => "target",
    TargetHelp => "target <trg> [options]",
    ListTargets => "list-targets",
    Audio => "audio",
    NoAudio => "no-audio",
    Subs => "subs",
    NoSubs => "no-subs",
    Video => "video",
    NoVideo => "no-video",
    Buttons => "buttons",
    NoButtons => "no-buttons",
    Chapters => "chapters",
    NoChapters => "no-chapters",
    Fonts => "fonts",
    NoFonts => "no-fonts",
    Attachs => "attachs",
    NoAttachs => "no-attachs",
    Defaults => "defaults",
    LimDefaults => "lim-defaults",
    Forceds => "forceds",
    LimForceds => "lim-forceds",
    Enableds => "enableds",
    LimEnableds => "lim-enableds",
    Names => "names",
    Langs => "langs",
    Specials => "specials",
    ListLangs => "list-langs",
    FfprobeHelp => "ffprobe [options]",
    MkvextractHelp => "mkvextract [options]",
    MkvinfoHelp => "mkvinfo [options]",
    MkvmergeHelp => "mkvmerge [options]",
    Version => "version",
    Help => "help"
);
