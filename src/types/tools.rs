pub(crate) mod output;
mod paths;
pub(crate) mod tool;

use crate::{MuxError, Tool, ToolOutput, i18n::logs, types::helpers::try_write_args_to_json};
use enum_map::EnumMap;
use paths::try_get_tool_path;
use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
    process::Command,
};

/// Manages paths to external binary tools and handles their execution.
#[derive(Clone, Default)]
pub struct Tools {
    paths: EnumMap<Tool, Option<PathBuf>>,
    json: Option<PathBuf>,
}

impl Tools {
    /// Creates a new `Tools`, resolving paths to the given tools.
    ///
    /// Returns an error if any tool path cannot be resolved.
    pub fn try_from_tools(tools: impl IntoIterator<Item = Tool>) -> Result<Self, MuxError> {
        let mut new = Self::default();
        new.try_upd_tools_paths(tools)?;
        Ok(new)
    }

    /// Resolves and caches the path to the specified tool if not already set.
    ///
    /// Returns an error if tool path cannot be resolved.
    pub fn try_upd_tool_path(&mut self, tool: Tool) -> Result<(), MuxError> {
        if let None = self.paths[tool] {
            let path = try_get_tool_path(tool)?;
            self.paths[tool] = Some(path);
        }
        Ok(())
    }

    /// Resolves and caches paths to the specified tools.
    ///
    /// Returns an error if any tool path cannot be resolved.
    pub fn try_upd_tools_paths(
        &mut self,
        tools: impl IntoIterator<Item = Tool>,
    ) -> Result<(), MuxError> {
        for tool in tools {
            self.try_upd_tool_path(tool)?;
        }
        Ok(())
    }

    /// Returns the default JSON file path for command-line arguments inside the given directory.
    ///
    /// File name is `.command-args.json`.
    pub fn make_json(dir: impl Into<PathBuf>) -> PathBuf {
        let mut json = dir.into();
        json.push(".command-args.json");
        json
    }

    /// Sets the JSON file path used to store arguments for `mkvtoolnix` tools.
    pub fn upd_json(&mut self, json: impl Into<PathBuf>) {
        self.json = Some(json.into());
    }

    /// Sets the JSON file path and returns the updated [`Tools`] instance.
    pub fn json(mut self, json: impl Into<PathBuf>) -> Self {
        self.upd_json(json);
        self
    }

    /// Runs the specified tool with the given arguments.
    ///
    /// If the tool is from `mkvtoolnix` and a JSON file path is set,
    /// arguments are written to that file and passed as `@<path>`.
    ///
    /// Logs errors and the executed command if logging is enabled.
    pub fn run<I, T>(&self, tool: Tool, args: I) -> Result<ToolOutput, MuxError>
    where
        I: IntoIterator<Item = T> + Clone,
        T: AsRef<OsStr>,
    {
        let mut command = match self.paths[tool].as_ref() {
            Some(p) => Command::new(p),
            None => Command::new(tool.as_ref()),
        };

        let mut json_args: Option<Vec<String>> = None;

        match self.json.as_ref().filter(|_| tool.is_mkvtoolnix()) {
            Some(json) => match try_write_args_to_json(args.clone(), json) {
                Ok(args) => {
                    let mut json_with_at = OsString::from("@");
                    json_with_at.push(json);
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
            Err(e) => Err(format!("Running error: {}", e).into()),
        }
    }
}
