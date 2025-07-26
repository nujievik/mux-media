use super::Output;
#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{Input, MuxError, SEP_BYTES, ensure_long_path_prefix, ensure_trailing_sep};
use std::{
    env::current_dir,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

impl TryFrom<&Input> for Output {
    type Error = MuxError;

    /// Attempts to construct [`Output`] from a subdirectory "muxed" in the input directory.
    ///
    /// Sets [`Self::dir`] only, other components is default.
    ///
    /// ```
    /// # use mux_media::{MuxConfig, Output, markers::MCInput};
    /// # use std::{env::current_dir, path::Path};
    /// #
    /// # let input_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
    /// #     .join("tests")
    /// #     .join("test_data");
    /// # let mux_config = MuxConfig::try_from_args([Path::new("-i"), &input_dir]).unwrap();
    /// let input = mux_config.field::<MCInput>();
    /// let output = Output::try_from(input).unwrap();
    /// let dir = input.dir().join("muxed");
    ///
    /// assert_eq!(&dir, output.dir());
    /// assert_eq!("", output.name_begin());
    /// assert_eq!("", output.name_tail());
    /// assert_eq!("mkv", output.ext());
    /// assert_eq!(dir.join(".mkv"), output.build_out(""));
    /// assert_eq!(dir.join("a.mkv"), output.build_out("a"));
    /// ```
    ///
    /// # Error
    ///
    /// Returns an error if [`current_dir`] fails.
    ///
    /// # Warning
    ///
    /// Does not check whether [`Self::dir`] exists and is writable. To perform the check,
    /// call [`Self::try_finalize_init`] after constructing.
    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let dir = Self::make_dir(input.dir());
        Self::try_from_path(dir)
    }
}

impl Output {
    /// Attempts to construct [`Output`] from a path with pattern:
    /// `[dir][MAIN_SEPARATOR][name_begin][,name_tail][.ext]`.
    ///
    /// Any component is optional.
    ///
    /// # Error
    ///
    /// Returns an error if [`current_dir`] fails.
    ///
    /// # Warning
    ///
    /// Does not check whether [`Self::dir`] exists and is writable. To perform the check,
    /// call [`Self::try_finalize_init`] after constructing.
    ///
    /// # Path Parsing Rules
    ///
    /// 1. Sets [`Self::dir`] as `current_dir().join("muxed")` if:
    ///
    ///    - The pattern is empty.
    ///      ```
    ///      # use mux_media::{Output, ensure_long_path_prefix};
    ///      # use std::env::current_dir;
    ///      #
    ///      let expected = current_dir().unwrap().join("muxed");
    ///      let expected = ensure_long_path_prefix(expected);
    ///      let output = Output::try_from_path("").unwrap();
    ///      assert_eq!(&expected, output.dir());
    ///      ```
    ///
    ///    - The path does not exist as a directory
    ///      and does not contain [`MAIN_SEPARATOR`](std::path::MAIN_SEPARATOR).
    ///      ```
    ///      # use mux_media::{Output, ensure_long_path_prefix};
    ///      # use std::env::current_dir;
    ///      #
    ///      let expected = current_dir().unwrap().join("muxed");
    ///      let expected = ensure_long_path_prefix(expected);
    ///      let output = Output::try_from_path("missing_as_dir.mkv").unwrap();
    ///      assert_eq!(&expected, output.dir());
    ///      ```
    ///
    ///    - [`Path::parent`] is [`None`] or `Some("")`.
    ///      ```
    ///      # use mux_media::{Output, ensure_long_path_prefix};
    ///      # use std::env::current_dir;
    ///      #
    ///      let expected = current_dir().unwrap().join("muxed");
    ///      let expected = ensure_long_path_prefix(expected);
    ///      let output = Output::try_from_path("empty_parent").unwrap();
    ///      assert_eq!(&expected, output.dir());
    ///      ```
    ///
    /// 2. Sets [`Self::dir`] from full `path` if:
    ///
    ///    - The path is an existing directory.
    ///      ```
    ///      # use mux_media::{Output, ensure_long_path_prefix};
    ///      # use std::path::Path;
    ///      #
    ///      let expected = Path::new(env!("CARGO_MANIFEST_DIR"))
    ///          .join("tests")
    ///          .join("test_data");
    ///      let expected = ensure_long_path_prefix(expected);
    ///      let output = Output::try_from_path(&expected).unwrap();
    ///      assert_eq!(&expected, output.dir());
    ///      ```
    ///
    ///    - The path ends with a [`MAIN_SEPARATOR`](std::path::MAIN_SEPARATOR).
    ///      ```
    ///      # use mux_media::{Output, ensure_long_path_prefix};
    ///      # use std::{
    ///      #     ffi::OsString,
    ///      #     path::{Path, MAIN_SEPARATOR},
    ///      # };
    ///      #
    ///      let mut expected = Path::new(env!("CARGO_MANIFEST_DIR"))
    ///          .join("missing_as_dir")
    ///          .into_os_string();
    ///      expected.push(MAIN_SEPARATOR.to_string());
    ///      let expected = ensure_long_path_prefix(expected);
    ///      let output = Output::try_from_path(&expected).unwrap();
    ///      assert_eq!(Path::new(&expected), output.dir());
    ///      ```
    ///
    /// 3. Otherwise, sets [`Self::dir`] from `path.parent()`.
    ///    ```
    ///    # use mux_media::{Output, ensure_long_path_prefix};
    ///    # use std::path::Path;
    ///    #
    ///    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
    ///        .join("tests")
    ///        .join("test_data")
    ///        .join("missing_as_dir.mkv");
    ///    let path = ensure_long_path_prefix(path);
    ///    let output = Output::try_from_path(&path).unwrap();
    ///    assert_eq!(path.parent().unwrap(), output.dir());
    ///    ```
    ///
    /// 4. Ensures [`Self::dir`] is an absolute path. (On Unix, allows `~` start.)
    ///    ```
    ///    # use mux_media::{Output, ensure_long_path_prefix};
    ///    # use std::{
    ///    #     env::current_dir,
    ///    #     ffi::OsString,
    ///    #     path::{Path, MAIN_SEPARATOR},
    ///    # };
    ///    #
    ///    let expected = current_dir().unwrap().join("muxed");
    ///    let expected = ensure_long_path_prefix(expected);
    ///    let mut path = OsString::from("muxed");
    ///    path.push(MAIN_SEPARATOR.to_string());
    ///    let output = Output::try_from_path(path).unwrap();
    ///    assert_eq!(&expected, output.dir());
    ///
    ///    #[cfg(unix)]
    ///    {
    ///        let expected = Path::new("~/");
    ///        let output = Output::try_from_path(expected).unwrap();
    ///        assert_eq!(expected, output.dir());
    ///    }
    ///    ```
    ///
    /// 5. Ensures [`Self::dir`] ends with the [`MAIN_SEPARATOR`](std::path::MAIN_SEPARATOR).
    ///    ```
    ///    # use mux_media::Output;
    ///    # use std::path::MAIN_SEPARATOR;
    ///    #
    ///    let output = Output::try_from_path("").unwrap();
    ///    let end = *output.dir().as_os_str().as_encoded_bytes().last().unwrap();
    ///    assert_eq!(MAIN_SEPARATOR, char::from(end));
    ///    ```
    ///
    /// 6. On Windows, ensures [`Self::dir`] starts with the `\\?\`.
    ///    ```
    ///    # use mux_media::{Output, ensure_long_path_prefix};
    ///    # use std::env::current_dir;
    ///    #
    ///    #[cfg(windows)]
    ///    {
    ///        let path = ensure_long_path_prefix(current_dir().unwrap())
    ///            .to_str()
    ///            .unwrap()
    ///            .strip_prefix(r"\\?\")
    ///            .unwrap()
    ///            .to_owned();
    ///
    ///        let dir_bytes = Output::try_from_path(path)
    ///            .unwrap()
    ///            .dir()
    ///            .to_owned()
    ///            .into_os_string()
    ///            .into_encoded_bytes();
    ///
    ///        assert_eq!(b"\\\\?\\", &dir_bytes[..4]);
    ///    }
    ///    ```
    ///
    /// 7. Parses the `path.file_name()` (if present):
    ///
    ///    - Sets [`Self::name_begin`] from start to first `,` (if present; exclusive)
    ///      or to last `.` (exclusive).
    ///      ```
    ///      # use mux_media::Output;
    ///      #
    ///      ["begin", "begin.mkv"].iter().for_each(|path| {
    ///          let output = Output::try_from_path(path).unwrap();
    ///          assert_eq!("begin", output.name_begin());
    ///          assert_eq!("", output.name_tail());
    ///          assert_eq!("mkv", output.ext());
    ///      })
    ///      ```
    ///
    ///    - Sets [`Self::name_tail`] from first `,` (exclusive) to last `.` (exclusive).
    ///      ```
    ///      # use mux_media::Output;
    ///      #
    ///      [",tail", ",tail.mkv"].iter().for_each(|path| {
    ///          let output = Output::try_from_path(path).unwrap();
    ///          assert_eq!("", output.name_begin());
    ///          assert_eq!("tail", output.name_tail());
    ///          assert_eq!("mkv", output.ext());
    ///      })
    ///      ```
    ///
    ///    - Sets [`Self::ext`] from last `.` (exclusive) to the end,
    ///      if the dot is not the first character.
    ///      ```
    ///      # use mux_media::Output;
    ///      #
    ///      ["mkv", "MKV", "mp4", "avi"].into_iter().for_each(|ext| {
    ///          let path = format!(",.{}", ext);
    ///          let output = Output::try_from_path(path).unwrap();
    ///          assert_eq!("", output.name_begin());
    ///          assert_eq!("", output.name_tail());
    ///          assert_eq!(ext, output.ext());
    ///      })
    ///      ```
    ///
    /// 8. If no name is provided, sets [`Self::name_begin`] and [`Self::name_tail`] to empty.
    ///    ```
    ///    # use mux_media::Output;
    ///    # use std::path::Path;
    ///    #
    ///    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
    ///        .join("tests")
    ///        .join("test_data");
    ///    let output = Output::try_from_path(path).unwrap();
    ///    assert_eq!("", output.name_begin());
    ///    assert_eq!("", output.name_tail());
    ///    ```
    ///
    /// 9. If no extension is provided, sets [`Self::ext`] to `"mkv"`.
    ///    ```
    ///    # use mux_media::Output;
    ///    # use std::path::Path;
    ///    #
    ///    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
    ///        .join("tests")
    ///        .join("test_data")
    ///        .join("file_name");
    ///    let output = Output::try_from_path(path).unwrap();
    ///    assert_eq!("mkv", output.ext());
    ///    ```
    ///
    /// 10. The first `,` and the last `.` in the `path.file_name()` are used as delimiters and
    ///     are not stored. Any remaining commas or dots are retained in [`Self::name_tail`].
    ///     ```
    ///     # use mux_media::Output;
    ///     # use std::path::Path;
    ///     #
    ///     let path = Path::new(env!("CARGO_MANIFEST_DIR"))
    ///        .join("tests")
    ///        .join("test_data")
    ///        .join(",t,.ail.mkv");
    ///     let output = Output::try_from_path(path).unwrap();
    ///     assert_eq!("t,.ail", output.name_tail());
    ///     ```
    pub fn try_from_path(path: impl AsRef<Path>) -> Result<Self, MuxError> {
        let path = path.as_ref();

        let dir = Self::try_extract_dir(path)?;

        let name = match path.file_name() {
            Some(name) if Some(name) != dir.file_name() => name,
            _ => return Ok(Self::empty_with_dir(dir)),
        };

        let (name_begin, name_tail) = Self::split_stem(name);
        let ext = Self::extract_extension(name);

        Ok(Self {
            dir,
            temp_dir: PathBuf::new(),
            created_dirs: Vec::new(),
            name_begin,
            name_tail,
            ext,
        })
    }
}

impl Output {
    #[inline(always)]
    fn empty_with_dir(dir: PathBuf) -> Self {
        Self {
            dir,
            ..Default::default()
        }
    }

    #[inline(always)]
    fn try_extract_dir(path: &Path) -> Result<PathBuf, MuxError> {
        let res = |dir: PathBuf| -> Result<PathBuf, MuxError> {
            let dir: PathBuf = try_absolutize(dir)?.components().collect();
            let dir = ensure_long_path_prefix(dir);
            Ok(ensure_trailing_sep(dir))
        };

        let fallback = || -> Result<PathBuf, MuxError> {
            let dir = Self::make_dir(current_dir()?);
            res(dir)
        };

        if path.as_os_str().is_empty() {
            return fallback();
        }

        if path.is_dir() {
            return res(path.to_path_buf());
        }

        if path.as_os_str().as_encoded_bytes().ends_with(SEP_BYTES) {
            return res(path.to_path_buf());
        }

        if let Some(path) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            return res(path.to_path_buf());
        }

        fallback()
    }

    #[inline(always)]
    fn extract_extension(file_name: &OsStr) -> OsString {
        Path::new(file_name)
            .extension()
            .map(OsString::from)
            .unwrap_or_else(|| Self::default_ext())
    }

    #[inline(always)]
    fn split_stem(file_name: &OsStr) -> (OsString, OsString) {
        let stem = Path::new(file_name).file_stem().unwrap_or(OsStr::new(""));
        let stem_str = stem.to_string_lossy();

        match stem_str.find(',') {
            Some(pos) => {
                let (begin, tail) = stem_str.split_at(pos);
                let tail = &tail[1..];
                (OsString::from(begin), OsString::from(tail))
            }
            None if stem_str.is_empty() => (OsString::new(), OsString::new()),
            None => (OsString::from(stem), OsString::new()),
        }
    }
}

#[inline(always)]
fn try_absolutize(path: PathBuf) -> Result<PathBuf, MuxError> {
    #[cfg(unix)]
    {
        if path.starts_with("~") {
            return Ok(path);
        }
    }

    match path.is_absolute() {
        true => Ok(path),
        false => {
            let mut new = current_dir()?;
            new.push(path);
            Ok(new)
        }
    }
}
