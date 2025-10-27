pub(crate) mod output;
pub(crate) mod paths;
pub(crate) mod tool;

use crate::{
    MuxConfig, Result, Tool, ToolOutput, i18n::logs, types::helpers::try_write_args_to_json,
};
use paths::ToolPaths;
use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
    process::Command,
};

/// Manage execution a [`Tool`] commands.
#[derive(Clone, Debug)]
pub struct Tools<'a> {
    pub paths: &'a ToolPaths,
    pub json: Option<PathBuf>,
}

impl<'a> From<&'a ToolPaths> for Tools<'a> {
    fn from(paths: &'a ToolPaths) -> Tools<'a> {
        Tools { paths, json: None }
    }
}

impl<'a> From<&'a MuxConfig> for Tools<'a> {
    fn from(cfg: &'a MuxConfig) -> Tools<'a> {
        Tools::from(&cfg.tool_paths)
    }
}

impl Tools<'_> {
    /// Runs the specified tool with the given arguments.
    ///
    /// If the tool is from `mkvtoolnix` and a JSON file path is setted,
    /// arguments are written to that file and passed as `@path`.
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
        let mut command = match &self.paths[tool].get() {
            Some(p) => Command::new(p),
            None => Command::new(tool.as_ref()),
        };

        let mut json_args: Option<Vec<String>> = None;

        match self.json.as_ref().filter(|_| tool.is_mkvtoolnix()) {
            Some(json) => match try_write_args_to_json(args.clone(), json) {
                Ok(args) => {
                    let mut json_with_at = OsString::from("@");
                    json_with_at.push(&**json);
                    command.arg(json_with_at);
                    json_args = Some(args);
                }
                Err(e) => {
                    logs::warn_err_write_json(e);
                    command.args(args);
                }
            },
            None => {
                command.args(args);
            }
        }

        logs::debug_running_command(&command, json_args);

        match command.output() {
            Ok(out) => ToolOutput::from((tool, out)).ok_or_err(),
            Err(e) => Err(err!("Running error: {}", e)),
        }
    }
}
