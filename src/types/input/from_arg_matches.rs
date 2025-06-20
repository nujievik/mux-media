use super::Input;
use crate::{Range, cli_args, from_arg_matches};
use globset::GlobSet;
use std::path::PathBuf;

cli_args!(Input, InputArg; Input => "input", Range => "range", Up => "up", Check => "check",
          Down => "down", Skip => "skip");

impl clap::FromArgMatches for Input {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        let dir = from_arg_matches!(matches, PathBuf, Input, Self::try_default_dir, @try_default);
        let range = from_arg_matches!(matches, Range<u32>, Range, @no_default);
        let need_num = range.is_some();

        Ok(Self {
            dir: dir.clone(),
            range,
            skip: from_arg_matches!(matches, GlobSet, Skip, @no_default),
            up: from_arg_matches!(matches, u8, Up, Self::default_up),
            check: from_arg_matches!(matches, u16, Check, Self::default_check),
            down: from_arg_matches!(matches, u8, Down, Self::default_down),
            need_num,
            upmost: dir,
            ..Default::default()
        })
    }
}
