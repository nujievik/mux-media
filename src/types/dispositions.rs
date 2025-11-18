mod get;
mod max;
mod new;
mod to_ffmpeg_args;
mod to_json_args;
pub(crate) mod ty;

use crate::{IsDefault, LangCode, RangeUsize};
use std::collections::HashMap;

/// Config `default` dispositions.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct DefaultDispositions(pub Dispositions);

/// Config `forced` dispositions.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ForcedDispositions(pub Dispositions);

/// Common interface for settings of track flags by type.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct Dispositions {
    pub max_in_auto: Option<usize>,
    pub single_val: Option<bool>,
    pub idxs: Option<HashMap<usize, bool>>,
    pub ranges: Option<Vec<(RangeUsize, bool)>>,
    pub langs: Option<HashMap<LangCode, bool>>,
}

deref_singleton_tuple_struct!(DefaultDispositions, Dispositions, @from_str);
deref_singleton_tuple_struct!(ForcedDispositions, Dispositions, @from_str);
