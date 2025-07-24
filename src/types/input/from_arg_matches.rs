use super::Input;
use crate::{GlobSetPattern, Range, from_arg_matches};
use clap::{ArgMatches, Error, FromArgMatches};
use std::path::PathBuf;

impl FromArgMatches for Input {
    from_arg_matches!(@fn);
    from_arg_matches!(@fn_update);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        let range = from_arg_matches!(matches, Range<u64>, Range, @no_default);

        Ok(Self {
            need_num: range.is_some(),
            dir: from_arg_matches!(matches, PathBuf, Input, Self::try_default_dir, @try_default),
            range,
            skip: from_arg_matches!(matches, GlobSetPattern, Skip, @no_default),
            depth: from_arg_matches!(matches, u8, Depth, || Self::DEFAULT_DEPTH),
            out_need_num: false,
            dirs: Vec::new(),
        })
    }

    fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error> {
        from_arg_matches!(
            @upd, self, matches;
            dir, PathBuf, Input,
            depth, u8, Depth
        );

        from_arg_matches!(
            @upd, self, matches, @opt_field;
            range, Range<u64>, Range,
            skip, GlobSetPattern, Skip
        );

        self.need_num = self.range.is_some();
        self.out_need_num = false;
        self.dirs = Vec::new();

        Ok(())
    }
}
