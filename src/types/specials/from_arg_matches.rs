use super::Specials;
use crate::{CLIArg, cli_args, from_arg_matches};

cli_args!(Specials, SpecialsArg; Specials => "specials");

impl clap::FromArgMatches for Specials {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(from_arg_matches!(
            matches,
            Specials,
            Specials,
            Self::default
        ))
    }
}
