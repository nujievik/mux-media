use super::LangCode;
use super::list_langs::LIST_LANGS;
use super::map_from_str::MAP_FROM_STR;
use super::set_multiple_priority::SET_MULTIPLE_PRIORITY;
use crate::MuxError;
use std::str::FromStr;

impl LangCode {
    pub fn print_list_langs() {
        println!("{}", LIST_LANGS)
    }
}

impl Default for LangCode {
    fn default() -> Self {
        LangCode::Und // Undefined
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

impl ToString for LangCode {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}

impl LangCode {
    #[inline]
    fn is_multiple_priority(&self) -> bool {
        SET_MULTIPLE_PRIORITY.contains(self)
    }
}

impl TryFrom<&[String]> for LangCode {
    type Error = MuxError;

    fn try_from(slice: &[String]) -> Result<Self, Self::Error> {
        let mut unpriority = None;
        for s in slice {
            let s = s.to_lowercase();
            if let Ok(code) = s.parse::<Self>() {
                if code.is_multiple_priority() {
                    return Ok(code);
                } else if unpriority.is_none() {
                    unpriority = Some(code);
                }
            }
        }
        unpriority.ok_or("Not found any language code".into())
    }
}
