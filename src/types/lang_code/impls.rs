use super::{
    LangCode,
    has_duo_code::has_duo_code,
    list_langs::{LIST_LANGS, LIST_LANGS_REMAINDER},
    map_from_str::MAP_FROM_STR,
};
use crate::{IsDefault, MuxError, Result};
use std::{env, fmt, str::FromStr};

impl LangCode {
    /// Returns [`LangCode`] parsed from the system locale if successful;
    /// otherwise, returns [`LangCode::Und`] (undeterminated).
    pub(crate) fn init() -> Self {
        Self::try_from_system_locale().unwrap_or(Self::Und)
    }

    /// Prints the list of supported language codes to stdout.
    pub(crate) fn print_list_langs() {
        println!("{}", LIST_LANGS)
    }

    /// Prints the full list of supported language codes to stdout.
    pub(crate) fn print_list_langs_full() {
        println!("{}", LIST_LANGS);
        println!("{}", LIST_LANGS_REMAINDER)
    }

    pub(crate) fn has_duo_code(self) -> bool {
        has_duo_code(self)
    }
}

impl Default for LangCode {
    fn default() -> Self {
        LangCode::Und
    }
}
impl IsDefault for LangCode {
    fn is_default(&self) -> bool {
        matches!(self, LangCode::Und)
    }
}

impl fmt::Display for LangCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl FromStr for LangCode {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        let mut unpriority: Option<Self> = None;

        for s in str_to_ascii_words(s) {
            if matches!(s.len(), 2 | 3) {
                let s = s.to_ascii_lowercase();
                match MAP_FROM_STR.get(&s).copied() {
                    Some(l) if l.has_duo_code() => return Ok(l),
                    Some(l) => unpriority = Some(l),
                    None => (),
                }
            }
        }

        return unpriority.ok_or_else(|| err!("Not found any language code"));

        fn str_to_ascii_words(s: &str) -> impl Iterator<Item = &str> {
            use lazy_regex::{Lazy, Regex, regex};
            static REGEX_ASCII_WORD: &Lazy<Regex> = regex!(r"[a-zA-Z]+");
            REGEX_ASCII_WORD.find_iter(s).map(|mat| mat.as_str())
        }
    }
}

impl LangCode {
    #[inline]
    fn try_from_system_locale() -> Result<Self> {
        let locale = env::var("LC_ALL")
            .or_else(|_| env::var("LANG"))
            .or_else(|_| env::var("LC_MESSAGES"))
            .or_else(|_| get_system_locale_fallback().ok_or("No locale env or fallback found"))?;

        return locale.parse::<LangCode>();

        fn get_system_locale_fallback() -> Option<String> {
            #[cfg(windows)]
            {
                use std::ffi::OsString;
                use std::os::windows::ffi::OsStringExt;
                use winapi::um::winnls::GetUserDefaultLocaleName;

                const LOCALE_NAME_MAX_LENGTH: usize = 85;
                let mut buffer = [0u16; LOCALE_NAME_MAX_LENGTH];

                let len = unsafe {
                    GetUserDefaultLocaleName(buffer.as_mut_ptr(), LOCALE_NAME_MAX_LENGTH as i32)
                };

                if len > 0 {
                    let os_str = OsString::from_wide(&buffer[..(len as usize - 1)]);
                    os_str.into_string().ok()
                } else {
                    None
                }
            }

            #[cfg(unix)]
            {
                None
            }
        }
    }
}
