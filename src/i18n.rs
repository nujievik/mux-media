mod eng;
mod rus;

use crate::{LangCode, MuxError};
use once_cell::sync::Lazy;
use std::fmt;
use std::sync::Mutex;

static LANG_CODE: Lazy<Mutex<LangCode>> = Lazy::new(|| Mutex::new(LangCode::init()));

#[derive(Copy, Clone, Debug)]
pub enum Msg {
    ErrUpdLangCode,
    ErrWriteJson,
    FromPackage,
    InstallTool,
    MayFailIfCommandLong,
    NoInputMedia,
    NotFound,
    RunningCommand,
    Using,
}

impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Msg {
    pub fn to_str_eng(self) -> &'static str {
        eng::eng(self)
    }

    pub fn to_str(self) -> &'static str {
        match Self::get_lang_code() {
            LangCode::Eng => eng::eng(self),
            LangCode::Rus => rus::rus(self),
            _ => eng::eng(self),
        }
    }

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
                Self::ErrUpdLangCode,
                e,
                Self::Using,
                Self::get_lang_code()
            )
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
