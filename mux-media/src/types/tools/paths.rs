use super::Tools;
use crate::{Msg, MuxError, Tool};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

impl Tools {
    #[inline(always)]
    pub(super) fn try_find_path(&self, tool: Tool) -> Result<PathBuf, MuxError> {
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
            is_tool_help_success(Path::new(tool_str))
                .then(|| PathBuf::from(tool_str))
                .ok_or_else(err)
        }

        #[cfg(windows)]
        {
            #[cfg(all(feature = "with_embedded_bins", target_arch = "x86_64"))]
            if let Some(Some(path)) = (!self.user_tools).then(|| self.get_bundled_path(tool)) {
                return Ok(path);
            }

            if is_tool_help_success(Path::new(tool_str)) {
                return Ok(PathBuf::from(tool_str));
            }

            if tool.is_mkvtoolnix() {
                let mkvtoolnix_path = |dir: &str| -> PathBuf {
                    let mut path = PathBuf::from(dir);
                    path.push(tool_str);
                    path.set_extension("exe");
                    path
                };

                let path = mkvtoolnix_path(r"\\?\C:\Program Files\MkvToolNix");
                if is_tool_help_success(&path) {
                    return Ok(path);
                }

                #[cfg(target_pointer_width = "64")]
                {
                    let path = mkvtoolnix_path(r"\\?\C:\Program Files (x86)\MkvToolNix");
                    if is_tool_help_success(&path) {
                        return Ok(path);
                    }
                }
            }

            #[cfg(all(feature = "with_embedded_bins", target_arch = "x86_64"))]
            if let Some(Some(path)) = self.user_tools.then(|| self.get_bundled_path(tool)) {
                return Ok(path);
            }

            Err(err())
        }
    }

    #[cfg(all(feature = "with_embedded_bins", windows, target_arch = "x86_64"))]
    fn get_bundled_path(&self, tool: Tool) -> Option<PathBuf> {
        let path = self
            .json
            .as_ref()
            .and_then(|path| path.parent())
            .map(|path| {
                let mut path = path.join(tool.as_ref());
                path.set_extension("exe");
                path
            })?;

        let bytes = match tool {
            Tool::Ffmpeg => FFMPEG_BUNDLED,
            Tool::Mkvmerge => MKVMERGE_BUNDLED,
        };

        std::fs::write(&path, bytes).ok()?;

        is_tool_help_success(&path).then(|| path)
    }
}

#[inline]
fn is_tool_help_success(tool_path: &Path) -> bool {
    matches!(
        Command::new(tool_path).arg("-h").output(),
        Ok(output) if output.status.success()
    )
}

macro_rules! embed_tool_bin {
    ($var:ident, $path:expr) => {
        #[cfg(all(feature = "with_embedded_bins", windows, target_arch = "x86_64"))]
        static $var: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), $path));
    };
}

embed_tool_bin!(FFMPEG_BUNDLED, "/assets/win64/ffmpeg.exe");
embed_tool_bin!(MKVMERGE_BUNDLED, "/assets/win64/mkvmerge.exe");
