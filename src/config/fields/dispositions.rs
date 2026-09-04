mod get;
mod max;
mod new;
pub(crate) mod ty;

use crate::{IsDefault, Lang, RangeUsize};
use std::collections::HashMap;

/// A dispositions configuraion.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ConfigDispositions {
    pub max_in_auto: Option<usize>,
    pub single_val: Option<bool>,
    pub idxs: Option<HashMap<usize, bool>>,
    pub ranges: Option<Vec<(RangeUsize, bool)>>,
    pub langs: Option<HashMap<Lang, bool>>,
}
