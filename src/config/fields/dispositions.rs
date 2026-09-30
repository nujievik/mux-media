mod get;
mod max;
mod new;
pub(crate) mod ty;

use crate::{Bool, IndexMap, IsDefault, Lang, RangeUsize};

/// A dispositions configuraion.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ConfigDispositions {
    pub max_in_auto: Option<usize>,
    pub single_val: Option<Bool>,
    pub idxs: Option<IndexMap<usize, Bool>>,
    pub ranges: Option<Vec<(RangeUsize, Bool)>>,
    pub langs: Option<IndexMap<Lang, Bool>>,
}
