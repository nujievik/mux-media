use crate::MuxError;
use globset::{Glob, GlobSet, GlobSetBuilder};

/// A wrapper for [`GlobSet`] with its raw pattern string.
#[derive(Clone)]
pub struct GlobSetPattern {
    pub glob_set: GlobSet,
    pub raw: String,
}

impl std::str::FromStr for GlobSetPattern {
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
