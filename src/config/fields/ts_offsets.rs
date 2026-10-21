mod get;
mod new;
mod to_args;

use crate::{FxIndexMap, Lang, RangeUsize};
use is_default::IsDefault;

#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ConfigTsOffsets {
    pub single_val: Option<i32>,
    pub idxs: Option<FxIndexMap<usize, i32>>,
    pub ranges: Option<Vec<(RangeUsize, i32)>>,
    pub langs: Option<FxIndexMap<Lang, i32>>,
}
