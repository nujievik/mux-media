use super::Msg;

use crate::{LangCode, MuxError, MuxLogger};
use once_cell::sync::Lazy;
use std::fmt;
use std::sync::Mutex;

static LANG_CODE: Lazy<Mutex<LangCode>> = Lazy::new(|| Mutex::new(LangCode::init()));

impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str_localized())
    }
}

impl Msg {
    pub fn to_str(self) -> &'static str {
        self.as_eng()
    }

    pub fn to_str_localized(self) -> &'static str {
        match Self::get_lang() {
            LangCode::Eng => self.to_str(),
            LangCode::Rus => self.as_rus(),
            _ => self.to_str(),
        }
    }

    pub fn get_lang() -> LangCode {
        LANG_CODE
            .lock()
            .map(|guard| *guard)
            .unwrap_or(LangCode::Eng)
    }

    pub fn try_upd_lang(lang: LangCode) -> Result<(), MuxError> {
        if lang == Self::get_lang() {
            return Ok(());
        }

        if !Self::is_supported_lang(lang) {
            return Err([(Self::LangNotSupLog, format!(": '{}'", lang))]
                .as_slice()
                .into());
        }

        let mut code = LANG_CODE
            .lock()
            .map_err(|_| MuxError::from("Fail LANG_CODE.lock()"))?;

        *code = lang;

        Ok(())
    }

    pub fn upd_lang_or_warn(lang: LangCode) {
        Self::try_upd_lang(lang).unwrap_or_else(|e| {
            eprintln!(
                "{}{}: {}. {} '{}'",
                MuxLogger::get_color_prefix(log::Level::Warn),
                Self::ErrUpdLang,
                e.to_str_localized(),
                Self::Using,
                Self::get_lang()
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
