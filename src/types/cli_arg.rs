macro_rules! enum_cli_arg {
    ($( $var:ident => $s:expr ),* $(,)?) => {
        /// An enum of CLI arguments.
        #[derive(Copy, Clone, Debug)]
        #[non_exhaustive]
        pub enum CliArg {
            $( $var ),*
        }

        impl CliArg {
            /// Returns argument with leading dashes (e.g. `"--input"`).
            pub const fn dashed(self) -> &'static str {
                match self {
                    $( Self::$var => concat!("--", $s) ),*
                }
            }

            /// Returns argument without leading dashes (e.g. `"input"`).
            pub const fn undashed(self) -> &'static str {
                match self {
                    $( Self::$var => $s ),*
                }
            }
        }
    };
}

enum_cli_arg! {
    Input => "input",
    Output => "output",
    Range => "range",
    Skip => "skip",
    Depth => "depth",
    Solo => "solo",
    Locale => "locale",
    Jobs => "jobs",
    Verbose => "verbose",
    Quiet => "quiet",
    ExitOnErr => "exit-on-err",
    Load => "load",
    SaveConfig => "save-config",
    Reencode => "reencode",
    Pro => "pro",
    HelpAutoDefaults => "auto-defaults / --no-auto-defaults",
    AutoDefaults => "auto-defaults",
    NoAutoDefaults => "no-auto-defaults",
    HelpAutoForceds => "auto-forceds / --no-auto-forceds",
    AutoForceds => "auto-forceds",
    NoAutoForceds => "no-auto-forceds",
    HelpAutoNames => "auto-names / --no-auto-names",
    AutoNames => "auto-names",
    NoAutoNames => "no-auto-names",
    HelpAutoLangs => "auto-langs / --no-auto-langs",
    AutoLangs => "auto-langs",
    NoAutoLangs => "no-auto-langs",
    HelpAutoEncs => "auto-encs / --no-auto-encs",
    AutoEncs => "auto-encs",
    NoAutoEncs => "no-auto-encs",
    Target => "target",
    ListTargets => "list-targets",
    Streams => "streams",
    NoStreams => "no-streams",
    Audio => "audio",
    NoAudio => "no-audio",
    Subs => "subs",
    NoSubs => "no-subs",
    Video => "video",
    NoVideo => "no-video",
    Fonts => "fonts",
    NoFonts => "no-fonts",
    Attachs => "attachs",
    NoAttachs => "no-attachs",
    Chapters => "chapters",
    NoChapters => "no-chapters",
    Defaults => "defaults",
    MaxDefaults => "max-defaults",
    Forceds => "forceds",
    MaxForceds => "max-forceds",
    Names => "names",
    Langs => "langs",
    Parts => "parts",
    NoLinked => "no-linked",
    ListContainers => "list-containers",
    ListLangs => "list-langs",
    Sys => "sys",
    Ffmpeg => "ffmpeg",
    Version => "version",
    Help => "help",
}

/// Returns an [`CliArg`] string with leading dashes (e.g. `"--input"`).
#[macro_export]
macro_rules! dashed {
    ($arg:ident) => {
        $crate::CliArg::$arg.dashed()
    };
}

/// Returns an [`CliArg`] string without leading dashes (e.g. `"input"`).
#[macro_export]
macro_rules! undashed {
    ($arg:ident) => {
        $crate::CliArg::$arg.undashed()
    };
}
