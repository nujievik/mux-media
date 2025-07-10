use crate::{from_arg_matches, json_arg};
use log::LevelFilter;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Verbosity {
    Quiet,
    #[default]
    Normal,
    Verbose,
    Debug,
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
    pub fn to_level_filter(&self) -> LevelFilter {
        match self {
            Verbosity::Quiet => LevelFilter::Error,
            Verbosity::Normal => LevelFilter::Info,
            Verbosity::Verbose => LevelFilter::Debug,
            Verbosity::Debug => LevelFilter::Trace,
        }
    }
}

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

impl crate::ToJsonArgs for Verbosity {
    fn to_json_args(&self) -> Vec<String> {
        match self {
            Self::Quiet => vec![json_arg!(Quiet)],
            Self::Normal => Vec::new(),
            Self::Verbose => vec!["-v".to_string()],
            Self::Debug => vec!["-vv".to_string()],
        }
    }
}
