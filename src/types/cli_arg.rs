macro_rules! enum_cli_arg {
    ($( $var:ident => $s:expr ),* $(,)?) => {
        /// An enum of CLI arguments.
        #[derive(Copy, Clone, Debug)]
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
    // visible in help
    Input => "input",
    Output => "output",
    Range => "range",
    Skip => "skip",
    Depth => "depth",
    Solo => "solo",
    Locale => "locale",
    Verbose => "verbose",
    Quiet => "quiet",
    ExitOnErr => "exit-on-err",
    Json => "json",
    SaveConfig => "save-config",
    Reencode => "reencode",
    Threads => "threads",
    Pro => "pro",
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
    HelpAutoCharsets => "auto-charsets / --no-auto-charsets",
    AutoCharsets => "auto-charsets",
    NoAutoCharsets => "no-auto-charsets",
    Target => "target",
    ListTargets => "list-targets",
    Audio => "audio",
    NoAudio => "no-audio",
    Subs => "subs",
    NoSubs => "no-subs",
    Video => "video",
    NoVideo => "no-video",
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
    Raws => "raws",
    RmSegments => "rm-segments",
    NoLinked => "no-linked",
    ListContainers => "list-containers",
    ListLangs => "list-langs",
    ListLangsFull => "list-langs-full",
    UserTools => "user-tools",
    Ffmpeg => "ffmpeg",
    Version => "version",
    Help => "help",

    // mkvmerge
    Attachments => "attachments",
    AudioTracks => "audio-tracks",
    DefaultTrackFlag => "default-track-flag",
    ForcedDisplayFlag => "forced-display-flag",
    Language => "language",
    ListLanguages => "list-languages",
    NoAttachments => "no-attachments",
    NoGlobalTags => "no-global-tags",
    NoSubtitles => "no-subtitles",
    SubCharset => "sub-charset",
    SubtitleTracks => "subtitle-tracks",
    TrackEnabledFlag => "track-enabled-flag",
    TrackName => "track-name",
    TrackOrder => "track-order",
    VideoTracks => "video-tracks",

    // ffmpeg
    Metadata => "metadata",
    Title => "title",
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
