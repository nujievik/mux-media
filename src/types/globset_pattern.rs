use crate::MuxError;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::{ops::Deref, str::FromStr};

/// A wrapper around [`GlobSet`] with its raw pattern string.
#[derive(Clone, Debug, Default)]
pub struct GlobSetPattern {
    pub glob_set: GlobSet,
    pub raw: String,
}

impl Deref for GlobSetPattern {
    type Target = GlobSet;

    fn deref(&self) -> &GlobSet {
        &self.glob_set
    }
}

/// Compares strings [`GlobSetPattern::raw`].
impl PartialEq for GlobSetPattern {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl FromStr for GlobSetPattern {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut builder = GlobSetBuilder::new();

        for pattern in s.split(',') {
            let glob =
                Glob::new(pattern).map_err(|e| format!("Invalid pattern '{}': {}", pattern, e))?;
            builder.add(glob);
        }

        let glob_set = builder
            .build()
            .map_err(|e| format!("Failed to build patterns: {}", e))?;

        Ok(Self {
            glob_set,
            raw: s.to_string(),
        })
    }
}
