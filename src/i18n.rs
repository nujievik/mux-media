macro_rules! impl_msg_as_str {
    ($fn:ident, $( $enum_var:ident => $text:expr ),* $(,)?) => {
        impl $crate::Msg {
            #[inline(always)]
            pub(in crate::i18n) fn $fn(self) -> &'static str {
                #[allow(deprecated)]
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
    HelpIOOptions,
    HelpInput,
    HelpOutput,
    HelpRange,
    HelpSkip,
    HelpDepth,
    HelpSolo,

    HelpGlobalOptions,
    HelpLocale,
    HelpJobs,
    HelpVerbosity,
    HelpQuiet,
    HelpExitOnErr,
    HelpLoad,
    HelpSaveConfig,

    HelpAutoFlags,
    HelpNoAuto,
    HelpAutoDefaults,
    HelpAutoForceds,
    HelpAutoTitles,
    HelpAutoLangs,
    HelpAutoEncs,

    HelpSaveStreams,
    HelpAudio,
    HelpNoAudio,
    HelpSubs,
    HelpNoSubs,
    HelpVideo,
    HelpNoVideo,
    HelpFonts,
    HelpNoFonts,
    HelpAttachs,
    HelpNoAttachs,

    HelpTargetOptions,
    HelpTarget,
    HelpListTargets,
    HelpStreams,
    HelpNoStreams,
    HelpNoChapters,
    HelpDefaults,
    HelpMaxDefaults,
    HelpForceds,
    HelpMaxForceds,
    HelpTitles,
    HelpLangs,

    HelpRetimingOptions,
    HelpParts,
    HelpNoLinked,

    HelpOtherOptions,
    HelpListLangs,
    HelpVersion,
    HelpHelp,

    ConvertingSubtitleEncoding,
    FailSaveConfig,
    FailUpdateLanguage,
    FileAlreadyExists,
    #[deprecated]
    FileIsAlreadyExists,
    FileNotCached,
    FoundRepeat,
    LanguageIsNotSupportedForLogging,
    LoadingTxtConfig,
    Media,
    MediaNumberIsOutOfRange,
    Muxing,
    NoExternalMediaFound,
    NoInputDirMedia,
    NotASubtitleFile,
    NotMuxedAny,
    NotOutSaveAny,
    NotRecognizedMedia,
    Skipping,
    SuccessMuxed,
    UnsupportedFileExtension,
    Using,

    ListTargets,
}
