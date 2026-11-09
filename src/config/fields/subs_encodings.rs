mod get;
mod new;
mod to_args;

use crate::{FxIndexMap, IsDefault, Lang, RangeUsize};
use encoding_rs::Encoding;

#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ConfigSubsEncodings {
    pub single_val: Option<&'static Encoding>,
    pub idxs: Option<FxIndexMap<usize, &'static Encoding>>,
    pub ranges: Option<Vec<(RangeUsize, &'static Encoding)>>,
    pub langs: Option<FxIndexMap<Lang, &'static Encoding>>,
}
