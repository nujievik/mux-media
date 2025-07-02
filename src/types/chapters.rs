use crate::{
    MuxError, ToMkvmergeArgs, from_arg_matches, mkvmerge_arg, mkvmerge_no_arg, to_mkvmerge_args,
};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Chapters {
    no_flag: bool,
    file: Option<PathBuf>,
}

impl Chapters {
    pub fn try_from_path(path: impl Into<PathBuf>) -> Result<Self, MuxError> {
        let path = std::fs::canonicalize(path.into())?;
        if !path.is_file() {
            return Err("Is not a file".into());
        }
        std::fs::File::open(&path)?;

        Ok(Self {
            file: Some(path),
            ..Default::default()
        })
    }

    fn no_flag(mut self, val: bool) -> Self {
        self.no_flag = val;
        self
    }
}

from_arg_matches!(@impl, Chapters, Chapters, NoChapters);

mkvmerge_arg!(Chapters, "--chapters");
mkvmerge_no_arg!(Chapters, "--no-chapters");

impl ToMkvmergeArgs for Chapters {
    to_mkvmerge_args!(@fn);

    fn to_os_mkvmerge_args(&self, _: &mut crate::MediaInfo, _: &Path) -> Vec<std::ffi::OsString> {
        use crate::{IsDefault, MkvmergeArg, MkvmergeNoArg};

        if self.is_default() {
            Vec::new()
        } else if self.no_flag {
            vec![Self::MKVMERGE_NO_ARG.into()]
        } else if let Some(file) = &self.file {
            let arg = Self::MKVMERGE_ARG;
            let file = file.as_os_str().to_os_string();
            vec![arg.into(), file]
        } else {
            eprintln!("Unexpected None file. Return empty");
            Vec::new()
        }
    }
}
