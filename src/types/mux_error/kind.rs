/// Kind of [`MuxError`](crate::MuxError).
#[derive(Clone, Default, Debug, PartialEq)]
#[non_exhaustive]
pub enum MuxErrorKind {
    Clap,
    InvalidValue,
    MatchesErrorDowncast,
    MatchesErrorUnknownArgument,
    Ok,
    #[default]
    Unknown,
}
