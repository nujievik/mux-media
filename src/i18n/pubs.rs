use super::Msg;

use crate::{LangCode, MuxLogger, Result};
use std::{
    fmt,
    sync::atomic::{AtomicU8, Ordering},
};

static LANG: AtomicU8 = AtomicU8::new(LangCode::Eng as u8);

/// Displays localized string.
/// ```
/// use mux_media::{LangCode, Msg};
///
/// let using = Msg::Using;
/// Msg::try_set_lang(LangCode::Eng).unwrap();
/// assert_eq!(&using.to_string(), "Using");
///
/// Msg::try_set_lang(LangCode::Rus).unwrap();
/// assert_eq!(&using.to_string(), "Используется");
/// ```
impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Msg {
    /// Returns localized string.
    /// ```
    /// use mux_media::{LangCode, Msg};
    ///
    /// let using = Msg::Using;
    /// Msg::try_set_lang(LangCode::Eng).unwrap();
    /// assert_eq!(using.as_str(), "Using");
    ///
    /// Msg::try_set_lang(LangCode::Rus).unwrap();
    /// assert_eq!(using.as_str(), "Используется");
    /// ```
    #[inline]
    pub fn as_str(self) -> &'static str {
        match Self::lang() {
            LangCode::Rus => self.as_rus(),
            _ => self.as_eng(),
        }
    }

    /// Returns english string.
    /// ```
    /// use mux_media::{LangCode, Msg};
    ///
    /// Msg::try_set_lang(LangCode::Rus).unwrap();
    /// assert_eq!(Msg::Using.as_english(), "Using");
    /// ```
    #[inline]
    pub fn as_english(self) -> &'static str {
        self.as_eng()
    }

    pub(crate) fn as_str_localized(self) -> &'static str {
        self.as_str()
    }

    /// Returns current language.
    /// ```
    /// use mux_media::{LangCode, Msg};
    ///
    /// Msg::try_set_lang(LangCode::Eng).unwrap();
    /// assert_eq!(Msg::lang(), LangCode::Eng);
    ///
    /// Msg::try_set_lang(LangCode::Rus).unwrap();
    /// assert_eq!(Msg::lang(), LangCode::Rus);
    /// ```
    #[inline]
    pub fn lang() -> LangCode {
        const RUS_U8: u8 = LangCode::Rus as u8;

        match LANG.load(Ordering::Relaxed) {
            RUS_U8 => LangCode::Rus,
            _ => LangCode::Eng,
        }
    }

    /// Tries set language.
    ///
    /// # Errors
    /// Returns error **only if** language is not supported logs.
    ///
    /// # Examples
    /// ```
    /// use mux_media::{LangCode, Msg};
    ///
    /// Msg::try_set_lang(LangCode::Eng).unwrap();
    /// Msg::try_set_lang(LangCode::Rus).unwrap();
    /// Msg::try_set_lang(LangCode::Und).unwrap_err();
    /// ```
    pub fn try_set_lang(lang: LangCode) -> Result<()> {
        if Self::is_supported_lang(lang) {
            Self::set_lang(lang);
            Ok(())
        } else {
            Err(err!(LanguageIsNotSupportedForLogging))
        }
    }

    /// Tries set language; logs warning to `stderr` on failure.
    pub fn set_lang_or_warn(lang: LangCode) {
        use log::Level::Warn;

        Self::try_set_lang(lang).unwrap_or_else(|e| {
            eprintln!(
                "{}{}{}{}: {}. {} '{}'",
                MuxLogger::prefix_prefix(Warn),
                MuxLogger::prefix_root(Warn),
                MuxLogger::prefix_suffix(Warn),
                Self::FailUpdateLanguage,
                e,
                Self::Using,
                Self::lang()
            );
        })
    }

    fn set_lang(lang: LangCode) {
        LANG.store(lang as u8, Ordering::Relaxed);
    }

    pub(crate) fn set_lang_from_system_locale() {
        if let Some(l) = LangCode::get_from_system_locale() {
            Self::set_lang(l)
        }
    }

    #[inline]
    pub(crate) fn is_supported_lang(lang: LangCode) -> bool {
        match lang {
            LangCode::Eng => true,
            LangCode::Rus => true,
            _ => false,
        }
    }
}
