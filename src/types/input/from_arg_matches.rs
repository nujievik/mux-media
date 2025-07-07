use super::Input;
use crate::{GlobSetPattern, Range, from_arg_matches};
use clap::{ArgMatches, Error, FromArgMatches};
use std::path::PathBuf;

impl FromArgMatches for Input {
    from_arg_matches!(@fn);
    from_arg_matches!(@fn_update);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        let dir = from_arg_matches!(matches, PathBuf, Input, Self::try_default_dir, @try_default);
        let range = from_arg_matches!(matches, Range<u64>, Range, @no_default);
        let need_num = range.is_some();

        Ok(Self {
            dir: dir.clone(),
            range,
            skip: from_arg_matches!(matches, GlobSetPattern, Skip, @no_default),
            up: from_arg_matches!(matches, u8, Up, || Self::DEFAULT_UP),
            check: from_arg_matches!(matches, u16, Check, || Self::DEFAULT_CHECK),
            down: from_arg_matches!(matches, u8, Down, || Self::DEFAULT_DOWN),
            need_num,
            upmost: dir,
            ..Default::default()
        })
    }

    fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error> {
        from_arg_matches!(
            @upd, self, matches;
            dir, PathBuf, Input,
            up, u8, Up,
            check, u16, Check,
            down, u8, Down
        );

        from_arg_matches!(
            @upd, self, matches, @opt_field;
            range, Range<u64>, Range,
            skip, GlobSetPattern, Skip
        );

        self.need_num = self.range.is_some();
        self.out_need_num = false;
        self.is_upmost_higher = false;

        if !(self.upmost == self.dir) {
            self.upmost = self.dir.clone();
        }

        self.dirs = Vec::new();

        Ok(())
    }
}
