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
    /// # use mux_media::{Output, IsDefault};
    /// #
    /// let o = Output::default();
    /// assert!(o.is_default());
    /// assert!(o.dir.is_default());
    /// assert!(o.temp_dir.is_default());
    /// assert!(o.name_begin.is_default());
    /// assert!(o.name_tail.is_default());
    /// assert_eq!(&o.ext, "mkv");
    /// assert!(o.created_dirs.is_default());
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
    /// Returns `true` if all [`Output`] fields eq [`Output::default`].
    /// ```
    /// # use mux_media::{Output, IsDefault};
    /// assert!(Output::default().is_default());
    /// assert!(!Output { dir: "x".into(), ..Default::default() }.is_default());
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
