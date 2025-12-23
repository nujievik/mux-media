mod get;
mod max;
mod new;
mod to_json_args;
pub(crate) mod ty;

use crate::{IsDefault, Lang, RangeUsize};
use std::collections::HashMap;

/// A `default` dispositions configuraion.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct DefaultDispositions(pub Dispositions);

/// A `forced` dispositions configuraion.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ForcedDispositions(pub Dispositions);

/// A dispositions configuraion.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct Dispositions {
    pub max_in_auto: Option<usize>,
    pub single_val: Option<bool>,
    pub idxs: Option<HashMap<usize, bool>>,
    pub ranges: Option<Vec<(RangeUsize, bool)>>,
    pub langs: Option<HashMap<Lang, bool>>,
}

deref_singleton_tuple_struct!(DefaultDispositions, Dispositions, @from_str);
deref_singleton_tuple_struct!(ForcedDispositions, Dispositions, @from_str);
