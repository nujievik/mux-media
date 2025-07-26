use super::MuxConfig;
use crate::{ParseableArg, ParseableArgs};

macro_rules! parseable_args {
    ($ty:ident, $enum_arg:ident;
    $( $arg:ident => $long:expr ),* ) => {
        impl ParseableArgs for $ty {
            type Arg = $enum_arg;
        }

        #[doc = concat!("[`ParseableArgs`] assotiated with the [`", stringify!($ty), "`].")]
        #[derive(Copy, Clone)]
        pub enum $enum_arg {
            $( $arg ),*
        }

        impl ParseableArg for $enum_arg {
            fn dashed(self) -> &'static str {
                match self {
                    $( Self::$arg => concat!("--", $long) ),*
                }
            }

            fn undashed(self) -> &'static str {
                match self {
                    $( Self::$arg => $long ),*
                }
            }
        }
    };
}

parseable_args!(
    MuxConfig, MuxConfigArg;
    Input => "input",
    Output => "output",
    Range => "range",
    Skip => "skip",
    Depth => "depth",
    Locale => "locale",
    Verbose => "verbose",
    Quiet => "quiet",
    Load => "load",
    Json => "json",
    ExitOnErr => "exit-on-err",
    Pro => "pro",
    HelpAutoCharsets => "auto-charsets / --no-auto-charsets",
    AutoCharsets => "auto-charsets",
    NoAutoCharsets => "no-auto-charsets",
    HelpAutoDefaults => "auto-defaults / --no-auto-defaults",
    AutoDefaults => "auto-defaults",
    NoAutoDefaults => "no-auto-defaults",
    HelpAutoForceds => "auto-forceds / --no-auto-forceds",
    AutoForceds => "auto-forceds",
    NoAutoForceds => "no-auto-forceds",
    HelpAutoEnableds => "auto-enableds / --no-auto-enableds",
    AutoEnableds => "auto-enableds",
    NoAutoEnableds => "no-auto-enableds",
    HelpAutoNames => "auto-names / --no-auto-names",
    AutoNames => "auto-names",
    NoAutoNames => "no-auto-names",
    HelpAutoLangs => "auto-langs / --no-auto-langs",
    AutoLangs => "auto-langs",
    NoAutoLangs => "no-auto-langs",
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
    MaxDefaults => "max-defaults",
    Forceds => "forceds",
    MaxForceds => "max-forceds",
    Enableds => "enableds",
    MaxEnableds => "max-enableds",
    Names => "names",
    Langs => "langs",
    Specials => "specials",
    ListLangs => "list-langs",
    UserTools => "user-tools",
    MkvmergeHelp => "mkvmerge [options]",
    Version => "version",
    Help => "help"
);
