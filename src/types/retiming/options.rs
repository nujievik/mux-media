mod new;
mod to_json_args;

use crate::{GlobSetPattern, IsDefault, RetimingChapter};

/// Config of retiming options.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct RetimingOptions {
    pub inverse: bool,
    pub parts: Option<GlobSetPattern>,
    pub no_linked: bool,
}

impl RetimingOptions {
    pub(crate) fn is_has_parts_cfg(&self) -> bool {
        !self.inverse.is_default() || !self.parts.is_default()
    }

    pub(crate) fn is_save_chapter(&self, chp: &RetimingChapter) -> bool {
        if self.no_linked && chp.uid.is_some() {
            return false;
        }

        if !self.is_has_parts_cfg() {
            return true;
        }

        if let Some(pat) = self.parts.as_ref() {
            if chp.title.as_ref().is_some_and(|title| pat.is_match(title)) {
                return !self.inverse;
            }
        }

        self.inverse
    }
}
