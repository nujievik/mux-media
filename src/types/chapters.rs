use crate::{CLIArg, MuxError, ToMkvmergeArgs, cli_args, from_arg_matches, to_mkvmerge_args};
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

cli_args!(Chapters, ChaptersArg; Chapters => "chapters", "--chapters",
          NoChapters => "no-chapters", "--no-chapters");

impl clap::FromArgMatches for Chapters {
    from_arg_matches!(@unrealized_fns);
    from_arg_matches!(@fn_mut, Chapters, NoChapters);
}

impl ToMkvmergeArgs for Chapters {
    to_mkvmerge_args!(@fn);

    fn to_os_mkvmerge_args(&self, _: &mut crate::MediaInfo, _: &Path) -> Vec<std::ffi::OsString> {
        use crate::traits::IsDefault;

        if self.is_default() {
            Vec::new()
        } else if self.no_flag {
            let arg = to_mkvmerge_args!(@cli_arg, NoChapters);
            vec![arg.into()]
        } else if let Some(file) = &self.file {
            let arg = to_mkvmerge_args!(@cli_arg, Chapters);
            let file = file.as_os_str().to_os_string();
            vec![arg.into(), file]
        } else {
            eprintln!("Unexpected None file. Return empty");
            Vec::new()
        }
    }
}
