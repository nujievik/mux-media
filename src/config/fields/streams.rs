mod is_save;
mod new;
mod to_args;

use crate::{FxIndexSet, IsDefault, Lang, RangeUsize};

/// A streams configuration.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ConfigStreams {
    pub no_flag: bool,
    pub inverse: bool,
    pub idxs: Option<FxIndexSet<usize>>,
    pub ranges: Option<Vec<RangeUsize>>,
    pub langs: Option<FxIndexSet<Lang>>,
}
