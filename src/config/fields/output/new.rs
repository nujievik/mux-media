use super::ConfigOutput;
use crate::config::{ConfigInput, InputType};
use crate::{MuxError, Result, TryFinalizeInit};
use std::path::{Path, PathBuf};

impl ConfigOutput {
    /// Tries create new [`ConfigOutput`] from a path.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - path is file
    /// - fails [`std::env::current_dir`]
    pub fn new(path: impl AsRef<Path>) -> Result<ConfigOutput> {
        let dir = new_dir(path.as_ref())?;
        Ok(ConfigOutput {
            temp_dir: new_temp_dir(&dir),
            dir,
            len_created_dir_chain: 0,
        })
    }
}

impl Default for ConfigOutput {
    fn default() -> ConfigOutput {
        let dir = || Path::new(".").join("muxed");
        let dir = new_dir(&dir()).unwrap_or(PathBuf::from(dir()));
        ConfigOutput {
            temp_dir: new_temp_dir(&dir),
            dir,
            len_created_dir_chain: 0,
        }
    }
}

impl TryFrom<&ConfigInput> for ConfigOutput {
    type Error = MuxError;

    fn try_from(input: &ConfigInput) -> Result<ConfigOutput> {
        let i_dir: &Path = match &input.ty {
            InputType::Dir(d) => d,
            InputType::Files(xs) => xs[0].parent().unwrap_or(Path::new(".")),
        };
        Self::new(i_dir.join("muxed"))
    }
}

impl TryFinalizeInit for ConfigOutput {
    /// Calls [`ConfigOutput::create_dirs`].
    fn try_finalize_init(&mut self) -> Result<()> {
        self.create_dirs()
    }
}

fn new_dir(path: &Path) -> Result<PathBuf> {
    if path.is_file() {
        Err(err!("is not a directory"))
    } else {
        let dir: PathBuf = try_absolutize(path.into())?.components().collect();
        Ok(crate::ensure_long_path_prefix(dir))
    }
}

fn new_temp_dir(dir: &Path) -> PathBuf {
    const TEMP_SUBDIRECTORY_NAME: &str = concat!(".temp-", env!("CARGO_PKG_NAME"));
    dir.join(TEMP_SUBDIRECTORY_NAME)
}

fn try_absolutize(path: PathBuf) -> Result<PathBuf> {
    #[cfg(unix)]
    {
        if path.starts_with("~") {
            return Ok(path);
        }
    }

    if path.is_absolute() {
        Ok(path)
    } else {
        let mut new = std::env::current_dir()?;
        new.push(path);
        Ok(new)
    }
}
