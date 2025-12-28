use crate::{IsDefault, ToJsonArgs, dashed};

/// A chapters configuration.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
#[non_exhaustive]
pub struct Chapters {
    pub no_flag: bool,
}

impl ToJsonArgs for Chapters {
    fn append_json_args(&self, args: &mut Vec<String>) {
        if self.no_flag {
            args.push(dashed!(NoChapters).into());
        }
    }
}
