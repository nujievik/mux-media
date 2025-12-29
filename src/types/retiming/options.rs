mod new;
mod to_json_args;

use crate::{GlobSetPattern, IsDefault, RetimingChapter};

/// A retiming configuration.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
#[non_exhaustive]
pub struct RetimingOptions {
    pub parts: RetimingOptionsParts,
    pub no_linked: bool,
}

/// A retiming configuration by chapter titles.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct RetimingOptionsParts {
    pub inverse: bool,
    pub pattern: Option<GlobSetPattern>,
}

impl RetimingOptions {
    pub(crate) fn is_save_chapter(&self, chp: &RetimingChapter) -> bool {
        if self.no_linked && chp.uid.is_some() {
            return false;
        }

        if self.parts.is_default() {
            return true;
        }

        if let Some(pat) = self.parts.pattern.as_ref() {
            if chp.title.as_ref().is_some_and(|title| pat.is_match(title)) {
                return !self.parts.inverse;
            }
        }

        self.parts.inverse
    }
}
