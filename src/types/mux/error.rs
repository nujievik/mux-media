use clap::parser::MatchesError;

#[derive(Debug)]
pub enum MuxErrorKind {
    InvalidValue,
    MatchesErrorDowncast,
    MatchesErrorUnknownArgument,
    Unknown,
}

impl Default for MuxErrorKind {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug)]
pub struct MuxError {
    pub message: Option<String>,
    pub code: i32,
    pub kind: MuxErrorKind,
}

impl std::fmt::Display for MuxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "{}", msg),
            None => write!(f, ""),
        }
    }
}

impl std::error::Error for MuxError {}

impl MuxError {
    pub fn new() -> Self {
        Self {
            message: None,
            code: 1,
            kind: MuxErrorKind::default(),
        }
    }

    pub fn ok() -> Self {
        Self::new().code(0)
    }

    pub fn from_any<E: std::error::Error>(err: E) -> Self {
        Self::new().message(err)
    }

    pub fn message(mut self, msg: impl ToString) -> Self {
        self.message = Some(msg.to_string());
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

    pub fn use_stderr(&self) -> bool {
        self.code != 0
    }

    pub fn print(&self) {
        if let Some(msg) = &self.message {
            if self.use_stderr() {
                eprintln!("{}", msg);
            } else {
                println!("{}", msg);
            }
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
