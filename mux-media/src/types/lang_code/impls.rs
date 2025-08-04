use super::{
    LangCode, list_langs::LIST_LANGS, map_from_str::MAP_FROM_STR,
    set_multiple_priority::SET_MULTIPLE_PRIORITY,
};
use crate::{IsDefault, MuxError, TrackID};
use std::{env, fmt, str::FromStr};

impl LangCode {
    /// Returns [`LangCode`] parsed from the system locale if successful;
    /// otherwise, returns [`LangCode::Und`] (undeterminated).
    pub fn init() -> Self {
        Self::try_from_system_locale().unwrap_or(Self::Und)
    }

    /// Prints the list of supported language codes to stdout.
    pub fn print_list_langs() {
        println!("{}", LIST_LANGS)
    }

    pub(crate) fn try_priority(slice: &[String]) -> Result<Self, MuxError> {
        slice
            .iter()
            .find_map(|s| {
                let s = s.to_lowercase();
                s.parse::<Self>()
                    .ok()
                    .filter(|lang| SET_MULTIPLE_PRIORITY.contains(&lang))
            })
            .ok_or_else(|| "Not found a priority language".into())
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

    fn from_str(s: &str) -> Result<Self, <LangCode as FromStr>::Err> {
        MAP_FROM_STR
            .get(s)
            .copied()
            .ok_or_else(|| format!("Invalid language code: '{}'", s).into())
    }
}

impl From<&TrackID> for LangCode {
    fn from(tid: &TrackID) -> LangCode {
        match tid {
            TrackID::Lang(lang) => *lang,
            _ => Self::Und,
        }
    }
}

impl TryFrom<&[String]> for LangCode {
    type Error = MuxError;

    fn try_from(slice: &[String]) -> Result<Self, Self::Error> {
        let mut unpriority = None;

        for s in slice {
            let s = s.to_lowercase();

            if let Ok(code) = s.parse::<Self>() {
                if SET_MULTIPLE_PRIORITY.contains(&code) {
                    return Ok(code);
                }

                unpriority = Some(code);
            }
        }

        unpriority.ok_or("Not found any language code".into())
    }
}

impl LangCode {
    #[inline]
    fn try_from_system_locale() -> Result<Self, MuxError> {
        let locale = env::var("LC_ALL")
            .or_else(|_| env::var("LANG"))
            .or_else(|_| env::var("LC_MESSAGES"))
            .or_else(|_| get_system_locale_fallback().ok_or("No locale env or fallback found"))?;

        let locale_parts: Vec<String> = locale
            .split(&['-', '_', '.'])
            .map(|x| x.to_string())
            .collect();

        Self::try_from(locale_parts.as_slice())
    }
}

#[inline]
fn get_system_locale_fallback() -> Option<String> {
    #[cfg(windows)]
    {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use winapi::um::winnls::GetUserDefaultLocaleName;

        const LOCALE_NAME_MAX_LENGTH: usize = 85;
        let mut buffer = [0u16; LOCALE_NAME_MAX_LENGTH];

        unsafe {
            let len = GetUserDefaultLocaleName(buffer.as_mut_ptr(), LOCALE_NAME_MAX_LENGTH as i32);
            if len > 0 {
                let os_str = OsString::from_wide(&buffer[..(len as usize - 1)]);
                os_str.into_string().ok()
            } else {
                None
            }
        }
    }

    #[cfg(unix)]
    {
        None
    }
}
