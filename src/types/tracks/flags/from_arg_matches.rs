use super::{DefaultTFlags, EnabledTFlags, ForcedTFlags};
use crate::from_arg_matches;
use clap::{ArgMatches, Error, FromArgMatches};

macro_rules! flags_from_arg_matches {
    ($type:ident, $arg:ident, $lim_arg:ident) => {
        impl FromArgMatches for $type {
            from_arg_matches!(@unrealized_fns);

            fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
                let lim = from_arg_matches!(matches, u64, $lim_arg, @no_default);
                Ok(from_arg_matches!(matches, $type, $arg, Self::default).lim_for_unset(lim))
            }
        }
    };
}

flags_from_arg_matches!(DefaultTFlags, Defaults, LimDefaults);
flags_from_arg_matches!(ForcedTFlags, Forceds, LimForceds);
flags_from_arg_matches!(EnabledTFlags, Enableds, LimEnableds);
