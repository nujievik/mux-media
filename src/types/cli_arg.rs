macro_rules! enum_cli_arg {
    ($( $var:ident => $s:expr ),* $(,)?) => {
        /// A CLI argument.
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
    Overwrite => "overwrite",
    Range => "range",
    Skip => "skip",
    Depth => "depth",
    OrphansToo => "orphans-too",

    Locale => "locale",
    LocaleStreams => "locale-streams",
    LocaleLog => "locale-log",
    Verbose => "verbose",
    Quiet => "quiet",
    Jobs => "jobs",
    ExitOnErr => "exit-on-err",
    Load => "load",
    SaveConfig => "save-config",

    NoAuto => "no-auto",
    HelpAutoDefaults => "auto-defaults / --no-auto-defaults",
    AutoDefaults => "auto-defaults",
    NoAutoDefaults => "no-auto-defaults",
    HelpAutoForceds => "auto-forceds / --no-auto-forceds",
    AutoForceds => "auto-forceds",
    NoAutoForceds => "no-auto-forceds",
    HelpAutoTitles => "auto-titles / --no-auto-titles",
    AutoTitles => "auto-titles",
    NoAutoTitles => "no-auto-titles",
    HelpAutoLangs => "auto-langs / --no-auto-langs",
    AutoLangs => "auto-langs",
    NoAutoLangs => "no-auto-langs",
    HelpAutoEncs => "auto-encs / --no-auto-encs",
    AutoEncs => "auto-encs",
    NoAutoEncs => "no-auto-encs",
    HelpAutoTsOffsets => "auto-ts-offsets / --no-auto-ts-offsets",
    AutoTsOffsets => "auto-ts-offsets",
    NoAutoTsOffsets => "no-auto-ts-offsets",

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
    CleanFlags => "clean-flags",
    CleanMeta => "clean-meta",

    Target => "target",
    ListTargets => "list-targets",
    Streams => "streams",
    NoStreams => "no-streams",
    Chapters => "chapters",
    NoChapters => "no-chapters",
    Order => "order",
    Defaults => "defaults",
    MaxDefaults => "max-defaults",
    Forceds => "forceds",
    MaxForceds => "max-forceds",
    Titles => "titles",
    Langs => "langs",
    SubsEncoding => "subs-encoding",
    TsOffsets => "ts-offsets",

    Parts => "parts",
    NoLinked => "no-linked",

    ListLangs => "list-langs",
    Version => "version",
    Help => "help",
}

/// Returns a [`CliArg`] string with leading dashes (e.g. `"--input"`).
#[macro_export]
macro_rules! dashed {
    ($arg:ident) => {
        $crate::CliArg::$arg.dashed()
    };
}

/// Returns a [`CliArg`] string without leading dashes (e.g. `"input"`).
#[macro_export]
macro_rules! undashed {
    ($arg:ident) => {
        $crate::CliArg::$arg.undashed()
    };
}
