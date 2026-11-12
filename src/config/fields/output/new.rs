use super::ConfigOutput;
use crate::config::ConfigInput;
use crate::{MuxError, Result, TryFinalizeInit};
use std::path::{Path, PathBuf};

const TEMP_SUBDIR: &str = concat!(".temp-", env!("CARGO_PKG_NAME"));

impl ConfigOutput {
    /// Tries create new [`ConfigOutput`] from a directory path.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - path is file
    /// - fails [`std::path::absolute`]
    pub fn try_from_dir<P>(dir: P) -> Result<ConfigOutput>
    where
        P: Into<PathBuf> + AsRef<Path>,
    {
        let dir = new_dir(dir, false)?;
        Ok(Self::with_dir(dir, false))
    }

    fn with_dir(mut dir: PathBuf, is_from_input: bool) -> ConfigOutput {
        let dir_len = dir.as_os_str().len();
        dir.push(TEMP_SUBDIR);
        ConfigOutput {
            temp_dir: dir.into_boxed_path(),
            dir_len,
            len_created_dir_chain: 0,
            is_from_input,
        }
    }
}

impl TryFrom<&ConfigInput> for ConfigOutput {
    type Error = MuxError;

    fn try_from(input: &ConfigInput) -> Result<ConfigOutput> {
        const OUTPUT_SUBDIR: &str = "muxed";
        let input_dir = input.dir();
        let cap = input_dir.as_os_str().len() + 1 + OUTPUT_SUBDIR.len() + 1 + TEMP_SUBDIR.len();

        let mut dir = PathBuf::with_capacity(cap);
        dir.push(input_dir);
        dir.push(OUTPUT_SUBDIR);

        let dir = new_dir(dir, true)?;
        Ok(Self::with_dir(dir, true))
    }
}

impl TryFinalizeInit for ConfigOutput {
    /// Calls [`ConfigOutput::create_dirs`].
    fn try_finalize_init(&mut self) -> Result<()> {
        self.create_dirs()
    }
}

fn new_dir(p: impl Into<PathBuf> + AsRef<Path>, is_from_input: bool) -> Result<PathBuf> {
    if is_from_input {
        check_is_not_file(p.as_ref())?;
        Ok(p.into())
    } else {
        let p = try_absolutize(p.into())?;
        let p = crate::ensure_long_path_prefix(p);
        check_is_not_file(&p)?;
        Ok(p)
    }
}

fn check_is_not_file(p: &Path) -> Result<()> {
    if p.is_file() {
        Err(err!("is not a directory"))
    } else {
        Ok(())
    }
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
        let path = std::path::absolute(path)?;
        Ok(path)
    }
}
