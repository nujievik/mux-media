use crate::{MuxError, Tool};
use log::warn;
use std::{fmt, process::Output};

/// Represents the output of an executed [`std::process::Command`].
#[derive(Debug)]
pub struct ToolOutput {
    tool: Tool,
    success: bool,
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl ToolOutput {
    /// Returns the captured stdout as a string slice.
    pub fn as_str_stdout(&self) -> &str {
        &self.stdout
    }

    /// Returns the captured stderr as a string slice.
    pub fn as_str_stderr(&self) -> &str {
        &self.stderr
    }

    /// Returns `Ok(Self)` if the tool succeeded, or `Err(MuxError)` otherwise.
    ///
    /// For `mkvtoolnix` tools, a non-zero exit code is tolerated if no `Error:` is found in stdout.
    pub(super) fn ok_or_err(self) -> Result<Self, MuxError> {
        if self.success {
            return Ok(self);
        }

        if !self.tool.is_mkvtoolnix() {
            return Err(self.into());
        }

        // mkvtoolnix tools output all messages to stdout; errors are prefixed with "Error:"
        match self.stdout.rsplitn(2, "Error:").skip(1).next() {
            Some(_) => Err(self.into()),
            None => Ok(self),
        }
    }

    /// Logs all warning lines in stdout, if tool is from `mkvtoolnix`.
    ///
    /// Warnings are detected by the `Warning: ` prefix.
    pub(crate) fn log_warns(&self) {
        if !self.tool.is_mkvtoolnix() {
            return;
        }

        // Mkvtoolnix always uses stdout and marks warning as `Warning: `
        self.stdout.split("Warning: ").skip(1).for_each(|s| {
            let s = s.split('\n').next().unwrap_or("");
            if !s.is_empty() {
                warn!("{}", s);
            }
        })
    }
}

impl fmt::Display for ToolOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tool.is_mkvtoolnix() {
            return write!(f, "{}", &self.stdout);
        }

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
