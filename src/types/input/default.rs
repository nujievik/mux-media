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
    /// use mux_media::Input;
    /// use std::path::PathBuf;
    ///
    /// let i = Input::default();
    /// assert_eq!(i.dir, PathBuf::default());
    /// assert_eq!(i.range, None);
    /// assert_eq!(i.skip, None);
    /// assert_eq!(i.depth, 16);
    /// assert_eq!(i.solo, false);
    /// assert_eq!(i.need_num, false);
    /// assert_eq!(i.out_need_num, false);
    /// assert_eq!(i.dirs, Default::default());
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
    /// Returns `true` if all `self` fields eq [`Input::default`] fields.
    /// ```
    /// use mux_media::*;
    /// use enum_map::EnumMap;
    /// use is_default::IsDefault;
    ///
    /// assert!(Input::default().is_default());
    ///
    /// assert!(!Input { dir: "x".into(), ..Default::default() }.is_default());
    /// assert!(!Input { range: Some("1".parse().unwrap()), ..Default::default() }.is_default());
    /// assert!(!Input { skip: Some("x".parse().unwrap()), ..Default::default() }.is_default());
    /// assert!(!Input { depth: 1, ..Default::default() }.is_default());
    /// assert!(!Input { solo: true, ..Default::default() }.is_default());
    /// assert!(!Input { need_num: true, ..Default::default() }.is_default());
    /// assert!(!Input { out_need_num: true, ..Default::default() }.is_default());
    ///
    /// let mut dirs: EnumMap<FileType, Vec<ArcPathBuf>> = Default::default();
    /// dirs[FileType::Media].push("x".into());
    /// assert!(!Input { dirs, ..Default::default() }.is_default());
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
