mod get;
mod new;
mod to_args;

use crate::{FxIndexMap, Lang, RangeUsize};
use is_default::IsDefault;

#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ConfigStreamsOrder {
    pub single_val: Option<u8>,
    pub idxs: Option<FxIndexMap<usize, u8>>,
    pub ranges: Option<Vec<(RangeUsize, u8)>>,
    pub langs: Option<FxIndexMap<Lang, u8>>,
}
