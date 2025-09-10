/// Kind of [`MuxError`](crate::MuxError).
#[derive(Clone, Default, Debug, PartialEq)]
pub enum MuxErrorKind {
    Clap,
    InvalidValue,
    MatchesErrorDowncast,
    MatchesErrorUnknownArgument,
    Ok,
    #[default]
    Unknown,
}
