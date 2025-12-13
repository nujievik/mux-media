use crate::{MuxError, Tool};
use std::{fmt, process::Output};

/// Represents the output of an executed [`Command`](std::process::Command).
#[derive(Debug)]
#[non_exhaustive]
pub struct ToolOutput {
    pub tool: Tool,
    pub success: bool,
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl ToolOutput {
    /// Returns `Ok(Self)` if the tool succeeded, or `Err(MuxError)` otherwise.
    pub(super) fn ok_or_err(self) -> Result<Self, MuxError> {
        if self.success {
            Ok(self)
        } else {
            Err(self.into())
        }
    }
}

impl fmt::Display for ToolOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stdout:\n{}", &self.stdout)?;
        write!(f, "\nStderr:\n{}", &self.stdout)
    }
}

impl From<(Tool, Output)> for ToolOutput {
    fn from(val: (Tool, Output)) -> Self {
        let (tool, out) = val;

        let to_string = |bytes: Vec<u8>| -> String { String::from_utf8_lossy(&bytes).to_string() };

        Self {
            tool,
            success: out.status.success(),
            code: out.status.code(),
            stdout: to_string(out.stdout),
            stderr: to_string(out.stderr),
        }
    }
}

impl From<ToolOutput> for MuxError {
    fn from(out: ToolOutput) -> Self {
        let code = out.code.unwrap_or(1);
        let mut s = out.stdout;
        s.push_str(&out.stderr);
        MuxError::from(s).code(code)
    }
}
