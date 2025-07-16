#[cfg(windows)]
use super::Tools;

use crate::{Msg, MuxError, Tool};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

macro_rules! embed_tool_bin {
    ($var:ident, $path64:expr, $path32:expr) => {
        #[cfg(all(windows, target_pointer_width = "64"))]
        static $var: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), $path64));
        #[cfg(all(windows, target_pointer_width = "32"))]
        static $var: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), $path32));
    };
}

embed_tool_bin!(
    MKVINFO_BUNDLED,
    "/assets/win64/mkvinfo.exe",
    "/assets/win32/mkvinfo.exe"
);
embed_tool_bin!(
    MKVMERGE_BUNDLED,
    "/assets/win64/mkvmerge.exe",
    "/assets/win32/mkvmerge.exe"
);

#[cfg(windows)]
impl Tools {
    pub fn try_upd_tool_path_from_bundled(
        &mut self,
        tool: Tool,
        temp_dir: &Path,
    ) -> Result<(), MuxError> {
        if let None = self.paths[tool] {
            let bytes = match tool {
                Tool::Mkvinfo => MKVINFO_BUNDLED,
                Tool::Mkvmerge => MKVMERGE_BUNDLED,
            };

            let mut tool_path = temp_dir.join(tool.as_ref());
            let _ = tool_path.set_extension("exe");

            std::fs::write(&tool_path, bytes)?;

            if !is_tool_help_success(&tool_path) {
                return Err("Tool help error".into());
            }

            self.paths[tool] = Some(tool_path);
        }
        Ok(())
    }

    pub fn try_upd_tools_paths_from_bundled(
        &mut self,
        tools: impl IntoIterator<Item = Tool>,
        temp_dir: &Path,
    ) -> Result<(), MuxError> {
        for tool in tools {
            self.try_upd_tool_path_from_bundled(tool, temp_dir)?;
        }
        Ok(())
    }
}

#[inline(always)]
pub(super) fn try_get_tool_path(tool: Tool) -> Result<PathBuf, MuxError> {
    let tool_str: &str = tool.as_ref();
    let tool_path = Path::new(tool_str);

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
        is_tool_help_success(tool_path)
            .then(|| tool_path.to_path_buf())
            .ok_or_else(err)
    }

    #[cfg(windows)]
    {
        let mut potential_paths: Vec<PathBuf> = Vec::with_capacity(3);
        potential_paths.push(tool_path.to_path_buf());

        if tool.is_mkvtoolnix() {
            for dir in &[
                r"\\?\C:\Program Files\MkvToolNix",
                r"\\?\C:\Program Files (x86)\MkvToolNix",
            ] {
                let mut path = PathBuf::from(dir);
                path.push(tool_str);
                path.set_extension("exe");
                potential_paths.push(path);
            }
        }

        match potential_paths
            .into_iter()
            .find(|path| is_tool_help_success(&path))
        {
            Some(path) => Ok(path),
            None => Err(err()),
        }
    }
}

#[inline(always)]
fn is_tool_help_success(tool_path: &Path) -> bool {
    matches!(
        Command::new(tool_path).arg("-h").output(),
        Ok(output) if output.status.success()
    )
}
