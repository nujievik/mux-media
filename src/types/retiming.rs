use crate::{IsDefault, from_arg_matches};
use clap::{ArgMatches, Error, FromArgMatches};
use globset::GlobSet;

#[derive(Clone)]
pub struct Retiming {
    pub rm_segments: Option<GlobSet>,
    pub no_linked: bool,
    pub less: bool,
}

impl IsDefault for Retiming {
    fn is_default(&self) -> bool {
        self.rm_segments.is_none() && !self.no_linked && !self.less
    }
}

impl FromArgMatches for Retiming {
    from_arg_matches!(@fn);
    from_arg_matches!(@fn_update);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        Ok(Self {
            rm_segments: from_arg_matches!(matches, GlobSet, RmSegments, @no_default),
            no_linked: from_arg_matches!(matches, bool, NoLinked, || false),
            less: from_arg_matches!(matches, bool, LessRetiming, || false),
        })
    }

    fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error> {
        from_arg_matches!(@upd, self, matches, @opt_field; rm_segments, GlobSet, RmSegments);

        from_arg_matches!(
            @upd, self, matches;
            no_linked, bool, NoLinked,
            less, bool, LessRetiming
        );

        Ok(())
    }
}
