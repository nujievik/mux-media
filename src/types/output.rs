mod to_args;
mod try_finalize_init;
mod try_from;

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use std::{
    ffi::{OsStr, OsString},
    fs,
    path::PathBuf,
};

/// An output configuration.
///
/// # Warning
///
/// This struct is not fully initialized after construction.
/// You **must** call [`Self::try_finalize_init`] before using some methods
/// (e.g. [`Self::temp_dir`]).
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Output {
    pub dir: PathBuf,
    pub temp_dir: PathBuf,
    pub name_begin: OsString,
    pub name_tail: OsString,
    pub ext: OsString,
    pub created_dirs: Vec<PathBuf>,
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
    /// is not empty; otherwise, expected a full [`Path::file_stem`](std::path::Path::file_stem).
    pub fn build_out(&self, name_middle: impl AsRef<OsStr>) -> PathBuf {
        let p = self.dir.join(&self.name_begin);
        let mut p = p.into_os_string();
        p.push(name_middle);
        p.push(&self.name_tail);
        p.push(".");
        p.push(&self.ext);
        p.into()
    }

    /// Returns `true` if a media number is expected in [`Self::build_out`].
    ///
    /// This is the case when either [`Self::name_begin`] or [`Self::name_tail`] is non-empty.
    /// ```
    /// use clap::Parser;
    /// use mux_media::Config;
    ///
    /// let mut o = Config::parse_from::<_, &str>([]).output;
    /// assert!(!o.need_num());
    ///
    /// o.name_begin = "x".into();
    /// assert!(o.need_num());
    /// o.name_begin = Default::default();
    ///
    /// o.name_tail = "x".into();
    /// assert!(o.need_num());
    /// ```
    #[inline]
    pub fn need_num(&self) -> bool {
        !self.name_begin.is_empty() || !self.name_tail.is_empty()
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
