pub(crate) mod output;
mod paths;
pub(crate) mod tool;

use crate::{MuxError, Tool, ToolOutput, i18n::logs, types::helpers::try_write_args_to_json};
use enum_map::EnumMap;
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
    #[allow(dead_code)]
    user_tools: bool,
}

impl Tools {
    /// Creates a new [`Tools`], resolving paths to the given tools.
    ///
    /// Returns an error if any tool path cannot be resolved.
    pub fn try_from_tools(tools: impl IntoIterator<Item = Tool>) -> Result<Self, MuxError> {
        let mut new = Self::default();
        new.try_upd_paths(tools)?;
        Ok(new)
    }

    /// Resolves and caches the path to the specified tool if not already set.
    ///
    /// Returns an error if tool path cannot be resolved.
    pub fn try_upd_path(&mut self, tool: Tool) -> Result<(), MuxError> {
        if let None = self.paths[tool] {
            let path = self.try_find_path(tool)?;
            self.paths[tool] = Some(path);
        }
        Ok(())
    }

    /// Resolves and caches paths to the specified tools.
    ///
    /// Returns an error if any tool path cannot be resolved.
    pub fn try_upd_paths(&mut self, tools: impl IntoIterator<Item = Tool>) -> Result<(), MuxError> {
        for tool in tools {
            self.try_upd_path(tool)?;
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
    pub fn set_json(&mut self, json: impl Into<PathBuf>) {
        self.json = Some(json.into());
    }

    /// Sets the priority for user tools.
    ///
    /// In the current implementation, this affects the behavior of path-related methods
    /// only in builds with features `with_embedded_bins` for Windows x86 or x86_64 architectures.
    pub fn set_user_tools(&mut self, user_tools: bool) {
        self.user_tools = user_tools;
    }

    /// Returns a reference to the tool path value if exists.
    pub fn get_path(&self, tool: Tool) -> Option<&PathBuf> {
        self.paths[tool].as_ref()
    }

    /// Runs the specified tool with the given arguments.
    ///
    /// If the tool is from `mkvtoolnix` and a JSON file path is set,
    /// arguments are written to that file and passed as `@<path>`.
    ///
    /// # Logging
    ///
    /// - **Only if** [`log`] is initialized with at least [`LevelFilter::Warn`](
    ///   log::LevelFilter::Warn).
    ///
    /// - Warn: Error write JSON.
    ///
    /// - Debug: Running command.
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
