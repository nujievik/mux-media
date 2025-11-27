pub(crate) mod output;
pub(crate) mod paths;
pub(crate) mod tool;

use crate::{Config, Result, Tool, ToolOutput, i18n::logs};
use paths::ToolPaths;
use std::{ffi::OsStr, process::Command};

#[derive(Clone, Debug)]
pub struct Tools<'a>(pub &'a ToolPaths);

impl<'a> From<&'a ToolPaths> for Tools<'a> {
    fn from(paths: &'a ToolPaths) -> Tools<'a> {
        Tools(paths)
    }
}
impl<'a> From<&'a Config> for Tools<'a> {
    fn from(cfg: &'a Config) -> Tools<'a> {
        Tools::from(&cfg.tool_paths)
    }
}

impl Tools<'_> {
    /// Runs the specified tool with the given arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if run command fails.
    ///
    /// # Logging
    ///
    /// - **Only if** [`log`] is initialized with at least [`LevelFilter::Warn`](
    ///   log::LevelFilter::Warn).
    ///
    /// - Warning: Error write JSON.
    ///
    /// - Debug: Running command.
    pub fn run<I, T>(&self, tool: Tool, args: I) -> Result<ToolOutput>
    where
        I: IntoIterator<Item = T> + Clone,
        T: AsRef<OsStr>,
    {
        let mut command = match &self.0[tool].get() {
            Some(p) => Command::new(p),
            None => Command::new(tool.as_ref()),
        };
        command.args(args);

        logs::debug_running_command(&command);

        match command.output() {
            Ok(out) => ToolOutput::from((tool, out)).ok_or_err(),
            Err(e) => Err(err!("Running error: {}", e)),
        }
    }
}
