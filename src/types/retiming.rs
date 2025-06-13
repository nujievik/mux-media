use crate::{CLIArg, IsDefault, cli_args, from_arg_matches};
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

cli_args!(Retiming, RetimingArg; RmSegments => "rm-segments", NoLinked => "no-linked",
          LessRetiming => "less-retiming");

impl clap::FromArgMatches for Retiming {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(Self {
            rm_segments: from_arg_matches!(matches, GlobSet, RmSegments, @no_default),
            no_linked: from_arg_matches!(matches, bool, NoLinked, || false),
            less: from_arg_matches!(matches, bool, LessRetiming, || false),
        })
    }
}
