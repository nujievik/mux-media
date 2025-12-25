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

/// A message with localized methods.
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
    HelpAutoDefaults,
    HelpAutoEncs,
    HelpAutoFlags,
    HelpAutoForceds,
    HelpAutoLangs,
    HelpAutoNames,
    HelpChapters,
    HelpDefaults,
    HelpDepth,
    HelpExitOnErr,
    HelpFonts,
    HelpForceds,
    HelpGlobalOptions,
    HelpHelp,
    HelpIOOptions,
    HelpInput,
    HelpJobs,
    HelpLangs,
    HelpListContainers,
    HelpListLangs,
    HelpListTargets,
    HelpLoad,
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
    HelpTargetHelp,
    HelpTargetOptions,
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
