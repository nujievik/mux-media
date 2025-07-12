use crate::{MuxError, Tool};
use log::warn;
use std::{fmt, process::Output};

#[derive(Debug)]
pub struct ToolOutput {
    tool: Tool,
    success: bool,
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl ToolOutput {
    pub(crate) fn into_err(self) -> MuxError {
        let code = self.code.unwrap_or(1);
        let mut s = self.stdout;
        s.push_str(&self.stderr);
        MuxError::from(s).code(code)
    }

    pub(super) fn ok_or_err(self) -> Result<Self, MuxError> {
        if self.success {
            return Ok(self);
        }

        if !self.tool.is_mkvtoolnix() {
            return Err(self.into_err());
        }

        // Mkvtoolnix always uses stdout, but prefixes any errors with `Error:`
        // Non-zero code could have returned due to `Warning:`, check for that
        match self.stdout.rsplitn(2, "Error:").skip(1).next() {
            Some(_) => Err(self.into_err()),
            None => Ok(self),
        }
    }

    pub fn as_str_stdout(&self) -> &str {
        &self.stdout
    }

    pub fn as_str_stderr(&self) -> &str {
        &self.stderr
    }

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
