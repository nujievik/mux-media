use super::Output;
use crate::IsDefault;
use std::ffi::OsString;

impl Output {
    pub(crate) const DEFAULT_EXT: &str = "mkv";

    pub(crate) fn default_ext() -> OsString {
        Self::DEFAULT_EXT.into()
    }
}

impl Default for Output {
    /// Returns a [`Output`] with "mkv" extension and all other default fields.
    /// ```
    /// use mux_media::Output;
    /// use std::path::PathBuf;
    ///
    /// let o = Output::default();
    /// assert_eq!(o.dir, PathBuf::default());
    /// assert_eq!(o.temp_dir, PathBuf::default());
    /// assert_eq!(o.name_begin, "");
    /// assert_eq!(o.name_tail, "");
    /// assert_eq!(o.ext, "mkv");
    /// assert!(o.created_dirs.is_empty());
    /// ```
    fn default() -> Output {
        Output {
            dir: Default::default(),
            temp_dir: Default::default(),
            name_begin: Default::default(),
            name_tail: Default::default(),
            ext: Self::default_ext(),
            created_dirs: Default::default(),
        }
    }
}

impl IsDefault for Output {
    /// Returns `true` if all [`Output`] fields eq [`Output::default`] fields.
    /// ```
    /// use mux_media::Output;
    /// use is_default::IsDefault;
    ///
    /// assert!(Output::default().is_default());
    /// assert!(!Output { dir: "x".into(), ..Default::default() }.is_default());
    /// assert!(!Output { temp_dir: "x".into(), ..Default::default() }.is_default());
    /// assert!(!Output { name_begin: "x".into(), ..Default::default() }.is_default());
    /// assert!(!Output { name_tail: "x".into(), ..Default::default() }.is_default());
    /// assert!(!Output { ext: "avi".into(), ..Default::default() }.is_default());
    /// assert!(!Output { created_dirs: vec!["x".into()], ..Default::default() }.is_default());
    /// ```
    fn is_default(&self) -> bool {
        self.dir.is_default()
            && self.temp_dir.is_default()
            && self.name_begin.is_default()
            && self.name_tail.is_default()
            && self.ext == Self::DEFAULT_EXT
            && self.created_dirs.is_default()
    }
}
