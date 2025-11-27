pub(crate) mod kind;
mod message;

use crate::{LangCode, Msg, MuxLogger};
use clap::parser::MatchesError;
use kind::MuxErrorKind;
use message::{MuxErrorMessage, MuxErrorMessageLocalized};
use std::fmt;

/// Error type used throughout the crate.
#[derive(Clone, Debug, PartialEq)]
pub struct MuxError {
    message: Option<MuxErrorMessage>,
    pub code: i32,
    pub kind: MuxErrorKind,
}

impl MuxError {
    /// Constructs a new [`MuxError`] with default values.
    pub fn new() -> MuxError {
        Self::default()
    }

    /// Constructs a new [`MuxError`] with code `0` and kind `Ok`.
    pub fn new_ok() -> MuxError {
        Self::new().code(0).kind(MuxErrorKind::Ok)
    }

    /// Sets the error message.
    pub fn message(mut self, msg: impl ToString) -> MuxError {
        self.message = Some(MuxErrorMessage::BoxedStr(msg.to_string().into_boxed_str()));
        self
    }

    /// Sets the error code.
    pub fn code(mut self, code: i32) -> MuxError {
        self.code = code;
        self
    }

    /// Sets the error kind.
    pub fn kind(mut self, kind: MuxErrorKind) -> MuxError {
        self.kind = kind;
        self
    }

    /// Constructs a new [`MuxError`] from any error.
    pub fn from_any<E: std::error::Error>(err: E) -> MuxError {
        Self::new().message(err)
    }

    /// Returns a English string.
    pub fn as_str(&self) -> &str {
        self.message.as_ref().map_or("", |msg| msg.as_str())
    }

    /// Returns a localized string if available; otherwise, returns a English string.
    pub fn as_str_localized(&self) -> &str {
        self.message
            .as_ref()
            .map_or("", |msg| msg.as_str_localized())
    }

    /// Returns `true` if the `code` is non-zero.
    pub fn use_stderr(&self) -> bool {
        self.code != 0
    }

    /// Prints a English message to `stderr` if `code` is non-zero; otherwise, to `stdout`.
    pub fn print(&self) {
        if let Some(msg) = &self.message {
            self.print_in_stderr_or_stdout(msg.as_str())
        }
    }

    /// Prints a localized message if available; otherwise, a English message.
    /// Outputs to `stderr` if `code` is non-zero; otherwise, to `stdout`.
    pub fn print_localized(&self) {
        if let Some(msg) = &self.message {
            self.print_in_stderr_or_stdout(msg.as_str_localized())
        }
    }

    #[inline]
    fn print_in_stderr_or_stdout(&self, msg: &str) {
        if self.use_stderr() {
            let prefix = MuxLogger::color_prefix(log::Level::Error);
            eprintln!("{}{}", prefix, msg);
            eprintln!("\n{}", MuxLogger::try_help());
        } else {
            println!("{}", msg);
        }
    }
}

impl Default for MuxError {
    fn default() -> Self {
        Self {
            message: None,
            code: 1,
            kind: MuxErrorKind::default(),
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

macro_rules! impl_from_any_to_string {
    ($ty:ty) => {
        impl From<$ty> for MuxError {
            fn from(s: $ty) -> Self {
                Self::new().message(s)
            }
        }
    };
}

impl_from_any_to_string!(String);
impl_from_any_to_string!(&str);

impl From<Msg> for MuxError {
    fn from(msg: Msg) -> Self {
        Self {
            message: Some(MuxErrorMessage::Msg(msg)),
            ..Default::default()
        }
    }
}

macro_rules! impl_from_slice_msg_opt {
    ($ty_opt:ty) => {
        impl From<&[(Msg, $ty_opt)]> for MuxError {
            fn from(slice: &[(Msg, $ty_opt)]) -> Self {
                let loc: MuxErrorMessageLocalized = slice.into();
                Self {
                    message: Some(MuxErrorMessage::Localized(loc)),
                    ..Default::default()
                }
            }
        }

        impl From<&[(Msg, $ty_opt)]> for MuxErrorMessageLocalized {
            fn from(slice: &[(Msg, $ty_opt)]) -> Self {
                let build = |is_eng: bool| {
                    slice
                        .iter()
                        .map(|(msg, opt)| {
                            let msg = if is_eng {
                                msg.as_str()
                            } else {
                                msg.as_str_localized()
                            };
                            format!("{}{}", msg, opt)
                        })
                        .collect::<String>()
                        .into_boxed_str()
                };

                let eng = build(true);

                let localized = match Msg::lang() {
                    LangCode::Eng => None,
                    _ => Some(build(false)),
                };

                Self { eng, localized }
            }
        }
    };
}

impl_from_slice_msg_opt!(String);
impl_from_slice_msg_opt!(&str);

impl From<clap::Error> for MuxError {
    fn from(err: clap::Error) -> MuxError {
        if err.use_stderr() {
            let _ = err.print();
            Self::new().code(err.exit_code()).kind(MuxErrorKind::Clap)
        } else {
            Self::new_ok()
        }
    }
}

impl From<MuxError> for clap::Error {
    fn from(err: MuxError) -> Self {
        if !err.use_stderr() {
            return clap::Error::new(clap::error::ErrorKind::DisplayVersion);
        }

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

macro_rules! impl_from_any {
    ($ty:ty) => {
        impl From<$ty> for MuxError {
            fn from(err: $ty) -> Self {
                Self::from_any(err)
            }
        }
    };
}

impl_from_any!(std::io::Error);
impl_from_any!(std::num::ParseIntError);
impl_from_any!(std::num::ParseFloatError);
impl_from_any!(serde_json::Error);
impl_from_any!(rsubs_lib::SRTError);
impl_from_any!(rsubs_lib::SSAError);
impl_from_any!(rsubs_lib::VTTError);
impl_from_any!(crate::ffmpeg::Error);
