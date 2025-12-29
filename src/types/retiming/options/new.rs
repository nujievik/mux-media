use super::*;
use crate::{Result, helpers};

impl RetimingOptions {
    pub(crate) fn from_str_parts(s: &str) -> Result<Self> {
        let s = s.trim();
        let (inverse, s) = helpers::parse_inverse_str(s);
        let pat = s.parse::<GlobSetPattern>()?;

        let parts = RetimingOptionsParts {
            inverse,
            pattern: Some(pat),
        };

        Ok(Self {
            parts,
            no_linked: false,
        })
    }
}
