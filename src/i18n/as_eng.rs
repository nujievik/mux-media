crate::impl_msg_as_localized_str!(
    as_eng,
    ArgsInJson => "Args in JSON",
    ErrUpdLang => "Update language failed",
    ErrWriteJson => "Write command to JSON failed",
    FileIsAlreadyExists => "File is already exists",
    FileTypeNotSup => "File type is not supported",
    FoundRepeat => "Found repeat",
    FromPackage => "From package",
    HelpAddCharsets => "On/Off auto set sub-charsets",
    HelpAddDefaults => "On/Off auto set default-track-flags",
    HelpAddEnableds => "On/Off auto set track-enabled-flags",
    HelpAddForceds => "On/Off auto set forced-display-flags",
    HelpAddLangs => "On/Off auto set track-languages",
    HelpAddNames => "On/Off auto set track-names",
    HelpAttachs => "[!]Copy other attachments n,m etc.",
    HelpAudio => "[!]Copy audio tracks n,m etc.",
    HelpButtons => "[!]Copy button tracks n,m etc.",
    HelpChapters => "Chapters info from chp file",
    HelpConfig => "Apply settings from json file",
    HelpDefaults => "Bool default-track-flags",
    HelpDepth => "Scan subdirectories up to this depth",
    HelpEnableds => "Bool track-enabled-flags",
    HelpExitOnErr => "Skip mux for next files if err",
    HelpFfprobeHelp => "Run ffprobe",
    HelpFonts => "[!]Copy font attachments n,m etc.",
    HelpForceds => "Bool forced-display-flags",
    HelpGlobalOptions => "Global options",
    HelpHelp => "Show help",
    HelpIOOptions => "I/O options",
    HelpInput => "File search start directory",
    HelpLangs => "Track languages",
    HelpLessRetiming => "No retiming if linked segments outside main",
    HelpLimDefaults => "Max true default-track-flags in auto",
    HelpLimEnableds => "Max true track-enabled-flags in auto",
    HelpLimForceds => "Max true forced-display-flags in auto",
    HelpListLangs => "Show supported language codes",
    HelpListTargets => "Show supported targets",
    HelpLocale => "Locale language (on logging and sort)",
    HelpMkvextractHelp => "Run mkvextract",
    HelpMkvinfoHelp => "Run mkvinfo",
    HelpMkvmergeHelp => "Run mkvmerge",
    HelpNames => "Track names",
    HelpNoAttachs => "Don't copy any other attachment",
    HelpNoAudio => "Don't copy any audio track",
    HelpNoButtons => "Don't copy any button track",
    HelpNoChapters => "Don't copy chapters",
    HelpNoConfig => "Not save and apply cfg file",
    HelpNoFonts => "Don't copy any font attachment",
    HelpNoLinked => "Remove linked segments",
    HelpNoSubs => "Don't copy any subtitle track",
    HelpNoVideo => "Don't copy any video track",
    HelpOffOnProOptions => "Off on Pro options",
    HelpOtherOptions => "Other options",
    HelpOutput => "Output paths pattern: out{num}[put]",
    HelpPro => "Off all auto 'Off on Pro options'",
    HelpQuiet => "Suppress logging",
    HelpRange => "Number range of files",
    HelpRetimingOptions => "Retiming options",
    HelpRmSegments => "Remove segments with name patterns",
    HelpSkip => "Patters of skip files",
    HelpSortFonts => "On/Off sort in-files fonts",
    HelpSpecials => "Set unpresented mkvmerge options",
    HelpSubs => "[!]Copy subtitle tracks n,m etc.",
    HelpTargetHelp => "Set next options for target",
    HelpTargetOptions => "Target options",
    HelpUserTools => "Priority use of user-installed tools",
    HelpVerbosity => "Increase logging",
    HelpVersion => "Show version",
    HelpVideo => "[!]Copy video tracks n,m etc.",
    InstallIt => "Please install it, add to system PATH and re-run",
    LMedia => "media",
    LangNotSupLog => "Language is not supported for logging",
    MayFailIfCommandLong => "May fail if command long",
    NoExtMediaFound => "No external media found",
    NoInputDirMedia => "No media found in the input directory",
    NoStemMedia => "No media found for stem",
    NotFound => "Not found",
    NotMuxedAny => "Not muxed any media",
    NotOutChange => "Not found any change for output",
    NotOutSaveAny => "Not found any save Track or Attach for output",
    NotRecognizedMedia => "Not recognized media file",
    NotSavedTrackOrAttach => "Not found any save Track or Attach",
    ReadsJson => "Reads JSON",
    RunningCommand => "Running command",
    Skipping => "Skipping",
    SuccessMuxed => "Success muxed",
    Using => "Using",
    ListTargets => r#"Supported targets (in order of priority):
 1. Path to a file
 2. Path to the parent directory of the file
 3. File group: video, audio, subs (file must contain the corresponding track type),
    or global (default)"#,
);
