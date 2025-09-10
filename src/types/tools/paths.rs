use crate::{IsDefault, Msg, MuxError, Result, Tool};
use enum_map::EnumMap;
use rayon::prelude::*;
use std::{
    ops::Deref,
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
};

/// A wrapper around [`EnumMap<Tool, OnceLock<PathBuf>>`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToolPaths {
    pub map: EnumMap<Tool, OnceLock<PathBuf>>,
    pub user_tools: bool,
}

impl Deref for ToolPaths {
    type Target = EnumMap<Tool, OnceLock<PathBuf>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl IsDefault for ToolPaths {
    fn is_default(&self) -> bool {
        self.map.values().all(|v| v.is_default())
    }
}

impl ToolPaths {
    /// Tries resolve and cache paths to the specified tools if not already cached.
    ///
    /// # Errors
    ///
    /// Returns an error if any tool path cannot be resolved.
    pub fn try_resolve_many(
        &self,
        tools: impl IntoParallelIterator<Item = Tool>,
        temp_dir: impl AsRef<Path>,
    ) -> Result<()> {
        let temp_dir = temp_dir.as_ref();
        tools
            .into_par_iter()
            .try_for_each(|t| self.try_resolve(t, temp_dir))
    }

    /// Tries resolve and cache path to the specified tool if not already cached.
    ///
    /// # Errors
    ///
    /// Returns an error if tool path cannot be resolved.
    pub fn try_resolve(&self, tool: Tool, temp_dir: impl AsRef<Path>) -> Result<()> {
        if self[tool].get().is_some() {
            return Ok(());
        }
        let p = resolve(tool, temp_dir.as_ref(), self.user_tools)?;
        self[tool].set(p).unwrap();

        Ok(())
    }
}

// unused_variables allowed for cross-system impl.
#[allow(unused_variables)]
fn resolve(tool: Tool, temp_dir: &Path, user_tools: bool) -> Result<PathBuf> {
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
        if let Some(Some(path)) = !user_tools.then(|| get_bundled_path(tool, temp_dir)) {
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
        if let Some(Some(path)) = user_tools.then(|| get_bundled_path(tool, temp_dir)) {
            return Ok(path);
        }

        Err(err())
    }
}

#[inline]
fn is_tool_help_success(tool_path: &Path) -> bool {
    matches!(
        Command::new(tool_path).arg("-h").output(),
        Ok(output) if output.status.success()
    )
}

#[cfg(all(feature = "with_embedded_bins", windows, target_arch = "x86_64"))]
fn get_bundled_path(tool: Tool, temp_dir: &Path) -> Option<PathBuf> {
    let mut path = temp_dir.join(tool.as_ref());
    path.set_extension("exe");

    let bytes = match tool {
        Tool::Ffmpeg => FFMPEG_BUNDLED,
        Tool::Ffprobe => FFPROBE_BUNDLED,
        Tool::Mkvmerge => MKVMERGE_BUNDLED,
    };

    std::fs::write(&path, bytes).ok()?;

    is_tool_help_success(&path).then(|| path)
}

macro_rules! embed_tool_bin {
    ($var:ident, $path:expr) => {
        #[cfg(all(feature = "with_embedded_bins", windows, target_arch = "x86_64"))]
        static $var: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), $path));
    };
}

embed_tool_bin!(FFMPEG_BUNDLED, "/assets/win64/ffmpeg.exe");
embed_tool_bin!(FFPROBE_BUNDLED, "/assets/win64/ffprobe.exe");
embed_tool_bin!(MKVMERGE_BUNDLED, "/assets/win64/mkvmerge.exe");
