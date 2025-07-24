use super::Specials;
use crate::from_arg_matches;

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
