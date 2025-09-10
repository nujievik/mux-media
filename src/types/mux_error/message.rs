use crate::Msg;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum MuxErrorMessage {
    Localized(MuxErrorMessageLocalized),
    Msg(Msg),
    BoxedStr(Box<str>),
}

impl MuxErrorMessage {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Localized(loc) => &loc.eng,
            Self::Msg(msg) => msg.as_str(),
            Self::BoxedStr(s) => s,
        }
    }

    pub fn as_str_localized(&self) -> &str {
        match self {
            Self::Localized(loc) => loc.localized.as_ref().unwrap_or_else(|| &loc.eng),
            Self::Msg(msg) => msg.as_str_localized(),
            Self::BoxedStr(s) => s,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MuxErrorMessageLocalized {
    pub eng: Box<str>,
    pub localized: Option<Box<str>>,
}

impl fmt::Display for MuxErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BoxedStr(s) => write!(f, "{}", s),
            Self::Msg(msg) => write!(f, "{}", msg.as_str()),
            Self::Localized(loc) => write!(f, "{}", loc.eng),
        }
    }
}
