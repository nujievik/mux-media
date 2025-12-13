mod get;
mod new;
mod to_ffmpeg_args;
mod to_json_args;

use crate::{IsDefault, Lang, RangeUsize};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

/// Names configuration.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct NameMetadata(pub Metadata<String>);

/// Langs configuration.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct LangMetadata(pub Metadata<Lang>);

#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct Metadata<T>
where
    T: Clone + Debug + Display + PartialEq + IsDefault,
{
    pub single_val: Option<T>,
    pub idxs: Option<HashMap<usize, T>>,
    pub ranges: Option<Vec<(RangeUsize, T)>>,
    pub langs: Option<HashMap<Lang, T>>,
}

deref_singleton_tuple_struct!(NameMetadata, Metadata<String>);
deref_singleton_tuple_struct!(LangMetadata, Metadata<Lang>);
