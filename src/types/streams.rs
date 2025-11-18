mod is_save;
mod new;
mod to_json_args;

use crate::{IsDefault, LangCode, RangeUsize};
use std::collections::HashSet;

/// Streams config.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct Streams {
    pub no_flag: bool,
    pub inverse: bool,
    pub idxs: Option<HashSet<usize>>,
    pub ranges: Option<Vec<RangeUsize>>,
    pub langs: Option<HashSet<LangCode>>,
}
