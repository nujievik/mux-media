use crate::{CLIArg, cli_args, from_arg_matches};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
    Debug,
}

impl Default for Verbosity {
    fn default() -> Self {
        Verbosity::Normal
    }
}

impl From<u8> for Verbosity {
    fn from(count: u8) -> Self {
        match count {
            0 => Verbosity::default(),
            1 => Verbosity::Verbose,
            _ => Verbosity::Debug,
        }
    }
}

impl Verbosity {
    pub fn to_level_filter(&self) -> log::LevelFilter {
        match self {
            Verbosity::Quiet => log::LevelFilter::Error,
            Verbosity::Normal => log::LevelFilter::Info,
            Verbosity::Verbose => log::LevelFilter::Debug,
            Verbosity::Debug => log::LevelFilter::Trace,
        }
    }
}

cli_args!(Verbosity, VerbosityArg; Verbose => "verbose", Quiet => "quiet");

impl clap::FromArgMatches for Verbosity {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(
            if from_arg_matches!(matches, bool, Quiet, @no_default).unwrap_or(false) {
                Self::Quiet
            } else if let Some(cnt) = from_arg_matches!(matches, u8, Verbose, @no_default) {
                cnt.into()
            } else {
                Self::default()
            },
        )
    }
}
