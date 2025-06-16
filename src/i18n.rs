mod eng;
mod rus;

use crate::{LangCode, MuxError};
use once_cell::sync::Lazy;
use std::fmt;
use std::sync::Mutex;

static LANG_CODE: Lazy<Mutex<LangCode>> = Lazy::new(|| Mutex::new(LangCode::init()));

pub enum Msg<'a> {
    ErrUpdLangCode,
    FailSetPaths { s: &'a str, s1: &'a str },
    FailWriteJson { s: &'a str },
    NoInputFiles,
    RunningCommand,
    Using,
}

impl fmt::Display for Msg<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match Self::get_lang_code() {
            LangCode::Eng => eng::eng(self),
            LangCode::Rus => rus::rus(self),
            _ => eng::eng(self),
        };

        write!(f, "{}", msg)
    }
}

impl<'a> Msg<'a> {
    pub fn get_lang_code() -> LangCode {
        LANG_CODE
            .lock()
            .map(|guard| *guard)
            .unwrap_or(LangCode::Eng)
    }

    pub fn try_upd_lang_code(lang: LangCode) -> Result<(), MuxError> {
        if Self::is_supported_lang(lang) {
            let mut code = LANG_CODE
                .lock()
                .map_err(|_| MuxError::from("Fail LANG_CODE.lock()"))?;
            *code = lang;
            Ok(())
        } else {
            Err(format!("Language '{}' is not supported for logging", lang).into())
        }
    }

    pub fn upd_lang_code_or_log_warn(lang: LangCode) {
        Self::try_upd_lang_code(lang).unwrap_or_else(|e| {
            log::warn!(
                "{}: {}. {} '{}'",
                Msg::ErrUpdLangCode,
                e,
                Msg::Using,
                Msg::get_lang_code()
            )
        })
    }

    #[inline]
    fn is_supported_lang(lang: LangCode) -> bool {
        match lang {
            LangCode::Eng => true,
            LangCode::Rus => true,
            _ => false,
        }
    }
}
