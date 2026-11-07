mod new;
mod to_args;

use crate::{GlobSetPattern, IsDefault};

#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
#[non_exhaustive]
pub struct ConfigRetiming {
    pub parts: ConfigRetimingParts,
    pub no_linked: bool,
}

/// A retiming configuration by chapter titles.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ConfigRetimingParts {
    pub inverse: bool,
    pub pattern: Option<GlobSetPattern>,
}
