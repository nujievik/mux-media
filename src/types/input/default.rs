use super::Input;
use crate::{IsDefault, Result};
use std::path::PathBuf;

impl Input {
    pub(crate) const DEPTH_DEFAULT: u8 = 16;

    pub(crate) fn try_default_dir() -> Result<PathBuf> {
        Self::try_canonicalize_and_read(".")
    }
}

impl Default for Input {
    /// Returns a [`Input`] with 16 [`Input::depth`] and all other default fields.
    /// ```
    /// # use mux_media::{Input, IsDefault};
    /// let i = Input::default();
    /// assert!(i.is_default());
    /// assert!(i.dir.is_default());
    /// assert!(i.range.is_default());
    /// assert!(i.skip.is_default());
    /// assert_eq!(i.depth, 16);
    /// assert!(i.solo.is_default());
    /// assert!(i.need_num.is_default());
    /// assert!(i.out_need_num.is_default());
    /// assert!(i.dirs.values().all(|v| v.is_default()));
    /// ```
    fn default() -> Input {
        Input {
            dir: Default::default(),
            range: Default::default(),
            skip: Default::default(),
            depth: Self::DEPTH_DEFAULT,
            solo: Default::default(),
            need_num: Default::default(),
            out_need_num: Default::default(),
            dirs: Default::default(),
        }
    }
}

impl IsDefault for Input {
    /// Returns `true` if all [`Input`] fields eq [`Input::default`].
    /// ```
    /// # use mux_media::{Input, IsDefault};
    /// assert!(Input::default().is_default());
    /// assert!(!Input { dir: "x".into(), ..Default::default() }.is_default());
    /// ```
    fn is_default(&self) -> bool {
        self.dir.is_default()
            && self.range.is_default()
            && self.skip.is_default()
            && self.depth == Self::DEPTH_DEFAULT
            && self.solo.is_default()
            && self.need_num.is_default()
            && self.out_need_num.is_default()
            && self.dirs.values().all(|v| v.is_default())
    }
}
