use crate::{Msg, MuxError, types::helpers::try_write_args_to_json};
use enum_map::{Enum, EnumMap};
use log::{debug, warn};
use std::{
    ffi::{OsStr, OsString},
    fmt,
    path::{Path, PathBuf},
    process::Command,
};
use strum_macros::{AsRefStr, EnumIter, EnumString};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, AsRefStr, Enum, EnumIter, EnumString)]
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

    fn as_str_package(self) -> &'static str {
        if self.is_mkvtoolnix() {
            "mkvtoolnix"
        } else {
            "ffmpeg"
        }
    }
}

impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Clone, Default)]
pub struct Tools {
    paths: EnumMap<Tool, Option<PathBuf>>,
    json: Option<PathBuf>,
}

impl Tools {
    pub fn try_from(iter: impl IntoIterator<Item = Tool>) -> Result<Self, MuxError> {
        let mut paths: EnumMap<Tool, Option<PathBuf>> = EnumMap::default();
        for tool in iter.into_iter() {
            paths[tool] = Some(get_tool_path(tool)?);
        }

        Ok(Self { paths, json: None })
    }

    pub fn try_upd_tool_path(&mut self, tool: Tool) -> Result<(), MuxError> {
        self.paths[tool] = Some(get_tool_path(tool)?);
        Ok(())
    }

    pub fn try_upd_tool_path_if_none(&mut self, tool: Tool) -> Result<(), MuxError> {
        self.paths[tool]
            .is_none()
            .then(|| self.try_upd_tool_path(tool))
            .unwrap_or(Ok(()))
    }

    pub fn make_json(dir: impl Into<PathBuf>) -> PathBuf {
        let mut json = dir.into();
        json.push(".command_args.json");
        json
    }

    pub fn upd_json(&mut self, json: impl Into<PathBuf>) {
        self.json = Some(json.into());
    }

    pub fn json(mut self, json: impl Into<PathBuf>) -> Self {
        self.upd_json(json);
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
            self.paths[tool]
                .as_ref()
                .map(|p| p.as_path())
                .unwrap_or(Path::new(tool.as_ref())),
        );

        let args_json = match &self.json {
            Some(json) if tool.is_mkvtoolnix() => {
                match try_write_args_to_json(args.clone(), json) {
                    Ok(args) => {
                        let mut json_with_at = OsString::from("@");
                        json_with_at.push(json);
                        command.arg(json_with_at);
                        Some(args)
                    }
                    Err(e) => {
                        warn!(
                            "{}: {}. {} CLI ({})",
                            Msg::ErrWriteJson,
                            e,
                            Msg::Using,
                            Msg::MayFailIfCommandLong
                        );
                        command.args(args);
                        None
                    }
                }
            }

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
            Err(e) => Err(MuxError::from(format!("Running error: {}", e))),
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
