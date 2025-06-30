use super::logger::get_stderr_color_prefix;
use crate::{LangCode, Msg};
use clap::parser::MatchesError;
use std::fmt;

#[derive(Debug)]
pub struct MuxError {
    message: Option<MuxErrorMessage>,
    pub code: i32,
    pub kind: MuxErrorKind,
}

#[derive(Debug, Default)]
pub enum MuxErrorKind {
    InvalidValue,
    MatchesErrorDowncast,
    MatchesErrorUnknownArgument,
    #[default]
    Unknown,
}

#[derive(Debug)]
enum MuxErrorMessage {
    Localized(MuxErrorMessageLocalized),
    Msg(Msg),
    String(String),
}

impl MuxErrorMessage {
    fn to_str(&self) -> &str {
        match self {
            Self::Localized(loc) => &loc.eng,
            Self::Msg(msg) => msg.to_str_eng(),
            Self::String(s) => s,
        }
    }

    fn to_str_localized(&self) -> &str {
        match self {
            Self::Localized(loc) => loc.localized.as_ref().unwrap_or_else(|| &loc.eng),
            Self::Msg(msg) => msg.to_str(),
            Self::String(s) => s,
        }
    }
}

#[derive(Debug)]
struct MuxErrorMessageLocalized {
    eng: String,
    localized: Option<String>,
}

impl fmt::Display for MuxErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Msg(msg) => write!(f, "{}", msg.to_str_eng()),
            Self::Localized(loc) => write!(f, "{}", loc.eng),
        }
    }
}

impl fmt::Display for MuxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "{}", msg),
            None => write!(f, ""),
        }
    }
}

impl std::error::Error for MuxError {}

impl Default for MuxError {
    fn default() -> Self {
        Self {
            message: None,
            code: 1,
            kind: MuxErrorKind::default(),
        }
    }
}

impl MuxError {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_ok() -> Self {
        Self::new().code(0)
    }

    pub fn from_any<E: std::error::Error>(err: E) -> Self {
        Self::new().message(err)
    }

    pub fn message(mut self, msg: impl ToString) -> Self {
        self.message = Some(MuxErrorMessage::String(msg.to_string()));
        self
    }

    pub fn code(mut self, code: i32) -> Self {
        self.code = code;
        self
    }

    fn kind(mut self, kind: MuxErrorKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn to_str_localized(&self) -> &str {
        self.message
            .as_ref()
            .map_or_else(|| "", |msg| msg.to_str_localized())
    }

    pub fn use_stderr(&self) -> bool {
        self.code != 0
    }

    #[inline(always)]
    fn print_in_stderr_or_stdout(&self, msg: &str) {
        if self.use_stderr() {
            eprintln!("{}{}", get_stderr_color_prefix(log::Level::Error), msg);
        } else {
            println!("{}", msg);
        }
    }

    pub fn print(&self) {
        if let Some(msg) = &self.message {
            self.print_in_stderr_or_stdout(msg.to_str())
        }
    }

    pub fn print_localized(&self) {
        if let Some(msg) = &self.message {
            self.print_in_stderr_or_stdout(msg.to_str_localized())
        }
    }
}

impl From<String> for MuxError {
    fn from(s: String) -> Self {
        Self::new().message(s)
    }
}

impl From<&str> for MuxError {
    fn from(s: &str) -> Self {
        Self::new().message(s)
    }
}

impl From<Msg> for MuxError {
    fn from(msg: Msg) -> Self {
        Self {
            message: Some(MuxErrorMessage::Msg(msg)),
            ..Default::default()
        }
    }
}

macro_rules! from_slice_msg_opt {
    ($ty_opt:ty) => {
        impl From<&[(Msg, $ty_opt)]> for MuxErrorMessageLocalized {
            fn from(slice: &[(Msg, $ty_opt)]) -> Self {
                let build = |is_eng: bool| {
                    slice
                        .iter()
                        .map(|(msg, opt)| {
                            let msg = if is_eng {
                                msg.to_str_eng()
                            } else {
                                msg.to_str()
                            };
                            format!("{}{}", msg, opt)
                        })
                        .collect::<String>()
                };

                let eng = build(true);

                let localized = match Msg::get_lang_code() {
                    LangCode::Eng => None,
                    _ => Some(build(false)),
                };

                Self { eng, localized }
            }
        }

        impl From<&[(Msg, $ty_opt)]> for MuxError {
            fn from(slice: &[(Msg, $ty_opt)]) -> Self {
                let loc: MuxErrorMessageLocalized = slice.into();
                Self {
                    message: Some(MuxErrorMessage::Localized(loc)),
                    ..Default::default()
                }
            }
        }
    };
}

from_slice_msg_opt!(String);
from_slice_msg_opt!(&str);

impl From<clap::Error> for MuxError {
    fn from(err: clap::Error) -> Self {
        let _ = err.print();
        Self::new().code(err.exit_code())
    }
}

impl From<MuxError> for clap::Error {
    fn from(err: MuxError) -> Self {
        let mut msg = err.to_string();
        if !msg.ends_with('\n') {
            msg.push('\n');
        }
        clap::Error::raw(clap::error::ErrorKind::InvalidValue, msg)
    }
}

impl From<MatchesError> for MuxError {
    fn from(err: MatchesError) -> Self {
        Self::new().message(&err).kind(match err {
            MatchesError::Downcast { .. } => MuxErrorKind::MatchesErrorDowncast,
            MatchesError::UnknownArgument { .. } => MuxErrorKind::MatchesErrorUnknownArgument,
            _ => MuxErrorKind::Unknown,
        })
    }
}

impl From<std::io::Error> for MuxError {
    fn from(err: std::io::Error) -> Self {
        Self::from_any(err)
    }
}

impl From<regex::Error> for MuxError {
    fn from(err: regex::Error) -> Self {
        Self::from_any(err)
    }
}
