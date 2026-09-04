use super::*;
use crate::{Result, helpers};

impl ConfigRetiming {
    pub(crate) fn from_str_parts(s: &str) -> Result<Self> {
        let s = s.trim();
        let (inverse, s) = helpers::parse_inverse_str(s);
        let pat = s.parse::<GlobSetPattern>()?;

        let parts = ConfigRetimingParts {
            inverse,
            pattern: Some(pat),
        };

        Ok(Self {
            parts,
            no_linked: false,
        })
    }
}
