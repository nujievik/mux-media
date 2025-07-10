mod displays;
pub(crate) mod output;
pub(crate) mod tool;

use crate::{Msg, MuxError, Tool, ToolOutput, types::helpers::try_write_args_to_json};
use displays::{debug_running_command, warn_err_write_json};
use enum_map::EnumMap;
use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
    process::Command,
};

#[derive(Clone, Default)]
pub struct Tools {
    paths: EnumMap<Tool, Option<PathBuf>>,
    json: Option<PathBuf>,
}

impl Tools {
    pub fn try_from_tools(tools: impl IntoIterator<Item = Tool>) -> Result<Self, MuxError> {
        let mut new = Self::default();
        new.try_upd_tools_paths(tools)?;
        Ok(new)
    }

    pub fn try_upd_tool_path(&mut self, tool: Tool) -> Result<(), MuxError> {
        if let None = self.paths[tool] {
            let path = try_get_tool_path(tool)?;
            self.paths[tool] = Some(path);
        }
        Ok(())
    }

    pub fn try_upd_tools_paths(
        &mut self,
        tools: impl IntoIterator<Item = Tool>,
    ) -> Result<(), MuxError> {
        for tool in tools {
            self.try_upd_tool_path(tool)?;
        }
        Ok(())
    }

    pub fn make_json(dir: impl Into<PathBuf>) -> PathBuf {
        let mut json = dir.into();
        json.push(".command-args.json");
        json
    }

    pub fn upd_json(&mut self, json: impl Into<PathBuf>) {
        self.json = Some(json.into());
    }

    pub fn json(mut self, json: impl Into<PathBuf>) -> Self {
        self.upd_json(json);
        self
    }

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
                    warn_err_write_json(e);
                    command.args(args);
                }
            },
            None => {
                command.args(args);
            }
        }

        debug_running_command(&command, json_args);

        match command.output() {
            Ok(out) => ToolOutput::from((tool, out)).ok_or_err(),
            Err(e) => Err(format!("Running error: {}", e).into()),
        }
    }
}

#[inline(always)]
fn try_get_tool_path(tool: Tool) -> Result<PathBuf, MuxError> {
    let tool_str: &str = tool.as_ref();

    let err = || -> MuxError {
        [
            (Msg::NotFound, format!(" '{}' (", tool_str)),
            (Msg::FromPackage, format!(" '{}'). ", tool.as_str_package())),
            (Msg::InstallIt, String::new()),
        ]
        .as_slice()
        .into()
    };

    #[cfg(unix)]
    {
        Command::new(tool_str)
            .arg("-h")
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|_| PathBuf::from(tool_str))
            .ok_or_else(err)
    }

    #[cfg(windows)]
    {
        let mut potential_paths: Vec<PathBuf> = Vec::with_capacity(3);
        potential_paths.push(PathBuf::from(tool_str));

        if tool.is_mkvtoolnix() {
            for dir in &[
                "C:\\Program Files\\MkvToolNix",
                "C:\\Program Files (x86)\\MkvToolNix",
            ] {
                let mut path = PathBuf::from(dir);
                path.push(tool_str);
                path.set_extension("exe");
                potential_paths.push(path);
            }
        }

        match potential_paths.into_iter().find(|path| {
            Command::new(path)
                .arg("-h")
                .output()
                .map(|o| o.status.success())
                .unwrap_or_default()
        }) {
            Some(valid_path) => Ok(valid_path),
            None => Err(err()),
        }
    }
}
