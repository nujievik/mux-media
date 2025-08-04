mod to_args;
mod try_finalize_init;
mod try_from;

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

/// Contains output paths components, related functions and methods.
///
/// # Warning
///
/// This struct is not fully initialized after construction.
/// You **must** call [`Self::try_finalize_init`] before using some methods
/// (e.g. [`Self::temp_dir`]).
#[derive(Clone, Debug)]
pub struct Output {
    dir: PathBuf,
    temp_dir: PathBuf,
    created_dirs: Vec<PathBuf>,
    name_begin: OsString,
    name_tail: OsString,
    ext: OsString,
}

impl Output {
    /// Builds the output path for the current media.
    ///
    /// Takes a middle part of the file name and constructs the full path
    /// by prepending [`Self::dir`] and [`Self::name_begin`],
    /// and appending [`Self::name_tail`] and [`Self::ext`].
    /// ```
    /// # use mux_media::{Output, ensure_long_path_prefix};
    /// # use std::path::Path;
    /// #
    /// let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
    ///     .join("tests")
    ///     .join("test_data");
    /// let dir = ensure_long_path_prefix(dir);
    /// let path = dir.join("begin,tail.ext");
    /// let output = Output::try_from_path(path).unwrap();
    ///
    /// let expected = dir.join("begin middle tail.ext");
    /// assert_eq!(expected, output.build_out(" middle "));
    /// ```
    ///
    /// The `name_middle` is expected to be a number if [`Self::name_begin`] or [`Self::name_tail`]
    /// is not empty; otherwise, expected a full [`Path::file_stem`].
    pub fn build_out(&self, name_middle: impl AsRef<OsStr>) -> PathBuf {
        let mut name = OsString::from(&self.dir);
        name.push(&self.name_begin);
        name.push(name_middle);
        name.push(&self.name_tail);
        name.push(".");
        name.push(&self.ext);
        name.into()
    }

    /// Returns `true` if a media number is expected in [`Self::build_out`].
    ///
    /// This is the case when either [`Self::name_begin`] or [`Self::name_tail`] is non-empty.
    #[inline]
    pub fn need_num(&self) -> bool {
        !self.name_begin.is_empty() || !self.name_tail.is_empty()
    }

    /// Returns the output directory.
    #[inline(always)]
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Returns the temporary directory.
    #[inline(always)]
    pub fn temp_dir(&self) -> &Path {
        &self.temp_dir
    }

    /// Returns the name begin.
    #[inline(always)]
    pub fn name_begin(&self) -> &OsStr {
        &self.name_begin
    }

    /// Returns the name tail.
    #[inline(always)]
    pub fn name_tail(&self) -> &OsStr {
        &self.name_tail
    }

    /// Returns the extension.
    #[inline(always)]
    pub fn ext(&self) -> &OsStr {
        &self.ext
    }

    /// Sets the extension.
    /// ```
    /// # use mux_media::Output;
    /// #
    /// let mut out = Output::try_from_path("").unwrap();
    /// ["avi", "mp4", "mkv"].iter().for_each(|ext| {
    ///     out.set_ext(ext);
    ///     assert_eq!(*ext, out.ext());
    /// })
    /// ```
    #[inline(always)]
    pub fn set_ext(&mut self, ext: impl Into<OsString>) {
        self.ext = ext.into();
    }

    /// Removes the temporary directory and all created empty directories.
    pub fn remove_created_dirs(&self) {
        let _ = fs::remove_dir_all(&self.temp_dir);
        remove_empty_chain_dirs(&self.created_dirs);
    }
}

fn remove_empty_chain_dirs(dirs: &[PathBuf]) {
    let ascending_order =
        (dirs.len() > 1) && (dirs[1].as_os_str().len() > dirs[0].as_os_str().len());

    match ascending_order {
        true => dirs.into_iter().rev().for_each(|dir| {
            let _ = fs::remove_dir(dir);
        }),
        false => dirs.into_iter().for_each(|dir| {
            let _ = fs::remove_dir(dir);
        }),
    }
}
