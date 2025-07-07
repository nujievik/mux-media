use mux_media::*;

macro_rules! cli_args_list {
    ($macro:ident) => {
        $macro! {
            Input => "input",
            Output => "output",
            Range => "range",
            Skip => "skip",
            Up => "up",
            Check => "check",
            Down => "down",
            Verbose => "verbose",
            Quiet => "quiet",
            Locale => "locale",
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
        }
    };
}

macro_rules! gen_cli_args_test {
    ( $( $arg:ident => $long:expr ),* ) => {
        #[test]
        fn test_cli_args() {
            $(
                assert_eq!($long, <MuxConfig as CLIArgs>::Arg::$arg.as_long());
            )*
        }
    };
}

macro_rules! gen_json_args_test {
    ( $( $arg:ident => $json_arg:expr ),* ) => {
        #[test]
        fn test_json_args() {
            $(
                assert_eq!(format!("--{}", $json_arg), json_arg!($arg));
            )*
        }
    };
}

cli_args_list!(gen_cli_args_test);
cli_args_list!(gen_json_args_test);
