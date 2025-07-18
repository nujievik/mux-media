mod as_eng;
mod as_rus;

pub(crate) mod logs;
mod pubs;

/// Localized messages.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Msg {
    ArgsInJson,
    ErrUpdLang,
    ErrWriteJson,
    FileIsAlreadyExists,
    FileTypeNotSup,
    FoundRepeat,
    FromPackage,
    HelpAddCharsets,
    HelpAddDefaults,
    HelpAddEnableds,
    HelpAddForceds,
    HelpAddLangs,
    HelpAddNames,
    HelpAttachs,
    HelpAudio,
    HelpButtons,
    HelpChapters,
    HelpConfig,
    HelpDefaults,
    HelpDepth,
    HelpEnableds,
    HelpExitOnErr,
    HelpFfprobeHelp,
    HelpFonts,
    HelpForceds,
    HelpGlobalOptions,
    HelpHelp,
    HelpIOOptions,
    HelpInput,
    HelpLangs,
    HelpLessRetiming,
    HelpLimDefaults,
    HelpLimEnableds,
    HelpLimForceds,
    HelpListLangs,
    HelpListTargets,
    HelpLocale,
    HelpMkvextractHelp,
    HelpMkvinfoHelp,
    HelpMkvmergeHelp,
    HelpNames,
    HelpNoAttachs,
    HelpNoAudio,
    HelpNoButtons,
    HelpNoChapters,
    HelpNoConfig,
    HelpNoFonts,
    HelpNoLinked,
    HelpNoSubs,
    HelpNoVideo,
    HelpOffOnProOptions,
    HelpOtherOptions,
    HelpOutput,
    HelpPro,
    HelpQuiet,
    HelpRange,
    HelpRetimingOptions,
    HelpRmSegments,
    HelpSkip,
    HelpSortFonts,
    HelpSpecials,
    HelpSubs,
    HelpTargetHelp,
    HelpTargetOptions,
    HelpUserTools,
    HelpVerbosity,
    HelpVersion,
    HelpVideo,
    InstallIt,
    LMedia,
    LangNotSupLog,
    ListTargets,
    MayFailIfCommandLong,
    NoExtMediaFound,
    NoInputDirMedia,
    NoStemMedia,
    NotFound,
    NotMuxedAny,
    NotOutChange,
    NotOutSaveAny,
    NotRecognizedMedia,
    NotSavedTrackOrAttach,
    ReadsJson,
    RunningCommand,
    Skipping,
    SuccessMuxed,
    Using,
}
