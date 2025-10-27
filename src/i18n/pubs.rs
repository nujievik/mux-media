use super::Msg;

use crate::{LangCode, MuxLogger, Result};
use std::{
    fmt,
    sync::{LazyLock, RwLock},
};

static LANG: LazyLock<RwLock<LangCode>> = LazyLock::new(|| RwLock::new(LangCode::init()));

/// Uses the localized string.
/// ```
/// # use mux_media::{LangCode, Msg};
/// let msg = Msg::Using;
/// Msg::try_upd_lang(LangCode::Eng).unwrap();
/// assert_eq!(&format!("{}", msg), "Using");
/// Msg::try_upd_lang(LangCode::Rus).unwrap();
/// assert_eq!(&format!("{}", msg), "Используется");
/// ```
impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str_localized())
    }
}

impl Msg {
    /// Returns the English string.
    /// ```
    /// # use mux_media::{LangCode, Msg};
    /// Msg::try_upd_lang(LangCode::Rus).unwrap();
    /// assert_eq!(Msg::Using.as_str(), "Using");
    /// ```
    pub fn as_str(self) -> &'static str {
        self.as_eng()
    }

    /// Returns the localized string.
    /// ```
    /// # use mux_media::{LangCode, Msg};
    /// Msg::try_upd_lang(LangCode::Eng).unwrap();
    /// assert_eq!(Msg::Using.as_str_localized(), "Using");
    /// Msg::try_upd_lang(LangCode::Rus).unwrap();
    /// assert_eq!(Msg::Using.as_str_localized(), "Используется");
    /// ```
    pub fn as_str_localized(self) -> &'static str {
        match Self::lang() {
            LangCode::Rus => self.as_rus(),
            _ => self.as_str(),
        }
    }

    /// Returns the current language.
    /// ```
    /// # use mux_media::{LangCode, Msg};
    /// Msg::try_upd_lang(LangCode::Eng).unwrap();
    /// assert_eq!(Msg::lang(), LangCode::Eng);
    /// Msg::try_upd_lang(LangCode::Rus).unwrap();
    /// assert_eq!(Msg::lang(), LangCode::Rus);
    /// ```
    pub fn lang() -> LangCode {
        LANG.read().map(|guard| *guard).unwrap_or(LangCode::Eng)
    }

    /// Tries update the current language.
    ///
    /// # Errors
    ///
    /// - Language is not supported logs
    ///
    /// - Failed to write [`RwLock<LangCode>`]
    ///
    /// # Examples
    /// ```
    /// # use mux_media::{LangCode, Msg};
    /// #
    /// Msg::try_upd_lang(LangCode::Eng).unwrap();
    /// Msg::try_upd_lang(LangCode::Rus).unwrap();
    /// Msg::try_upd_lang(LangCode::Und).unwrap_err();
    /// ```
    pub fn try_upd_lang(lang: LangCode) -> Result<()> {
        if lang == Self::lang() {
            return Ok(());
        }

        if !Self::is_supported_lang(lang) {
            return Err([(Self::LangNotSupLog, format!(": '{}'", lang))]
                .as_slice()
                .into());
        }

        let mut l = LANG.write().map_err(|_| err!("Fail LANG_CODE.write()"))?;

        *l = lang;

        Ok(())
    }

    /// Tries update the current language; logs a warning to `stderr` on failure.
    pub fn upd_lang_or_warn(lang: LangCode) {
        Self::try_upd_lang(lang).unwrap_or_else(|e| {
            eprintln!(
                "{}{}: {}. {} '{}'",
                MuxLogger::color_prefix(log::Level::Warn),
                Self::ErrUpdLang,
                e.as_str_localized(),
                Self::Using,
                Self::lang()
            );
        })
    }

    #[inline(always)]
    fn is_supported_lang(lang: LangCode) -> bool {
        match lang {
            LangCode::Eng => true,
            LangCode::Rus => true,
            _ => false,
        }
    }
}
