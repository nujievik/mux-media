macro_rules! impl_msg_as_str {
    ($fn:ident, $( $enum_var:ident => $text:expr ),* $(,)?) => {
        impl $crate::Msg {
            #[inline(always)]
            pub(in crate::i18n) fn $fn(self) -> &'static str {
                match self {
                    $( Self::$enum_var => $text ),*
                }
            }
        }
    };
}

mod as_eng;
mod as_rus;

pub(crate) mod logs;
mod pubs;

/// Localized messages.
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Msg {
    ContainerDoesNotSupport,
    ErrUpdLang,
    FileIsAlreadyExists,
    FileTypeNotSup,
    FoundRepeat,
    FromPackage,
    HelpAttachs,
    HelpAudio,
    HelpAutoCharsets,
    HelpAutoDefaults,
    HelpAutoFlags,
    HelpAutoForceds,
    HelpAutoLangs,
    HelpAutoNames,
    HelpChapters,
    HelpDefaults,
    HelpDepth,
    HelpExitOnErr,
    HelpFfmpegHelp,
    HelpFonts,
    HelpForceds,
    HelpGlobalOptions,
    HelpHelp,
    HelpIOOptions,
    HelpInput,
    HelpJson,
    HelpLangs,
    HelpListContainers,
    HelpListLangs,
    HelpListLangsFull,
    HelpListTargets,
    HelpLocale,
    HelpMaxDefaults,
    HelpMaxForceds,
    HelpNames,
    HelpNoAttachs,
    HelpNoAudio,
    HelpNoChapters,
    HelpNoFonts,
    HelpNoLinked,
    HelpNoStreams,
    HelpNoSubs,
    HelpNoVideo,
    HelpOtherOptions,
    HelpOutput,
    HelpParts,
    HelpPro,
    HelpQuiet,
    HelpRange,
    HelpReencode,
    HelpRetimingOptions,
    HelpSaveConfig,
    HelpSkip,
    HelpSolo,
    HelpSortFonts,
    HelpStreams,
    HelpSubs,
    HelpSys,
    HelpTargetHelp,
    HelpTargetOptions,
    HelpThreads,
    HelpVerbosity,
    HelpVersion,
    HelpVideo,
    InstallIt,
    LMedia,
    LMultipleTracksOrTypeTrack,
    LangNotSupLog,
    LoadingJson,
    MediaNumOutOfRange,
    Muxing,
    NoExtMediaFound,
    NoInputDirMedia,
    NoStemMedia,
    NotFound,
    NotFoundTrack,
    NotMuxedAny,
    NotOutSaveAny,
    NotRecognizedMedia,
    RunCommand,
    RunningCommand,
    Skipping,
    SuccessMuxed,
    UnsupOutContainerExt,
    UnsupRetimingExt,
    Using,
    ListTargets,
    ListContainers,
}
