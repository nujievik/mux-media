use crate::{Msg, MuxError};
use log::{debug, warn};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::process::Command;
use strum_macros::{AsRefStr, EnumIter, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, AsRefStr, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Tool {
    Ffprobe,
    Mkvextract,
    Mkvinfo,
    Mkvmerge,
}

impl Tool {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub fn iter_mkvtoolnix() -> impl Iterator<Item = Self> {
        Self::iter().filter(|tool| tool.is_mkvtoolnix())
    }

    fn is_mkvtoolnix(self) -> bool {
        self != Self::Ffprobe
    }

    fn to_str_package(self) -> &'static str {
        if self.is_mkvtoolnix() {
            "mkvtoolnix"
        } else {
            "ffmpeg"
        }
    }
}

#[derive(Clone, Default)]
pub struct Tools {
    paths: HashMap<Tool, PathBuf>,
    json: Option<PathBuf>,
}

impl Tools {
    pub fn try_from(iter: impl IntoIterator<Item = Tool>) -> Result<Self, MuxError> {
        let mut paths = HashMap::with_capacity(4);
        for tool in iter.into_iter() {
            paths.insert(tool, get_tool_path(tool)?);
        }

        Ok(Self { paths, json: None })
    }

    pub fn try_insert(&mut self, tool: Tool) -> Result<(), MuxError> {
        self.paths.insert(tool, get_tool_path(tool)?);
        Ok(())
    }

    pub fn make_json(dir: impl Into<PathBuf>) -> PathBuf {
        let mut json = dir.into();
        json.push(".command_args.json");
        json
    }

    pub fn update_json(&mut self, json: impl Into<PathBuf>) {
        self.json = Some(json.into());
    }

    pub fn json(mut self, json: impl Into<PathBuf>) -> Self {
        self.update_json(json);
        self
    }

    pub fn run<I, T>(&self, tool: Tool, args: I, msg: Option<&str>) -> Result<String, MuxError>
    where
        I: IntoIterator<Item = T> + Clone,
        T: AsRef<OsStr>,
    {
        if let Some(msg) = msg {
            debug!("{}", msg);
        }

        let mut command = Command::new(
            self.paths
                .get(&tool)
                .map(|p| p.as_path())
                .unwrap_or(Path::new(tool.as_ref())),
        );

        let args_json = match &self.json {
            Some(json) if tool.is_mkvtoolnix() => match write_args_to_json(args.clone(), json) {
                Ok(vec) => {
                    let mut json_with_at = OsString::from("@");
                    json_with_at.push(json);
                    command.arg(json_with_at);
                    Some(vec)
                }
                Err(e) => {
                    warn!("{}", Msg::FailWriteJson { s: &e });
                    command.args(args);
                    None
                }
            },

            _ => {
                command.args(args);
                None
            }
        };

        debug!("{}:\n{}", Msg::RunningCommand, CommandDisplay(&command));
        if let Some(args) = args_json {
            debug!("Args in JSON:\n{:?}", args);
        }

        match command.output() {
            Ok(output) if output.status.success() => {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            }
            Ok(output) => Err(MuxError::from(
                String::from_utf8_lossy(&output.stdout).to_string(),
            )),
            Err(e) => Err(MuxError::from(format!("Execution error: {}", e))),
        }
    }
}

struct CommandDisplay<'a>(&'a Command);

impl<'a> fmt::Display for CommandDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0.get_program().to_string_lossy())?;
        for arg in self.0.get_args() {
            write!(f, " \"{}\"", arg.to_string_lossy())?;
        }
        Ok(())
    }
}

fn get_tool_path(tool: Tool) -> Result<PathBuf, MuxError> {
    let tool_str: &str = tool.as_ref();

    #[cfg(unix)]
    {
        if Command::new(tool_str)
            .arg("-h")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            Ok(PathBuf::from(tool_str))
        } else {
            Err(Msg::FailSetPaths {
                s: tool.as_ref(),
                s1: tool.to_str_package(),
            }
            .to_string()
            .into())
        }
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
                .unwrap_or(false)
        }) {
            Some(valid_path) => Ok(valid_path),
            None => Err(Msg::FailSetPaths {
                s: tool.as_ref(),
                s1: tool.to_str_package(),
            }
            .to_string()
            .into()),
        }
    }
}

fn write_args_to_json<I, T>(args: I, json: &Path) -> Result<Vec<String>, String>
where
    I: IntoIterator<Item = T> + Clone,
    T: AsRef<OsStr>,
{
    let args = args
        .into_iter()
        .map(|arg| {
            arg.as_ref()
                .to_str()
                .ok_or("Unsupported UTF-8 symbol.".to_string())
                .map(|s| s.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let file = File::create(json).map_err(|e| format!("Create error: {}", e))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &args).map_err(|e| format!("JSON write error: {}", e))?;

    Ok(args)
}
