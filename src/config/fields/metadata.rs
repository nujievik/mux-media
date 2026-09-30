mod get;
mod new;
mod to_args;

use crate::{IndexMap, IsDefault, Lang, RangeUsize};
use std::fmt::{Debug, Display};

/// A `title` metadata configuration.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ConfigTitleMetadata(pub ConfigMetadata<String>);

/// A `language` metadata configuration.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ConfigLangMetadata(pub ConfigMetadata<Lang>);

/// A metadata configuration.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct ConfigMetadata<T>
where
    T: Clone + Debug + Display + PartialEq + IsDefault,
{
    pub single_val: Option<T>,
    pub idxs: Option<IndexMap<usize, T>>,
    pub ranges: Option<Vec<(RangeUsize, T)>>,
    pub langs: Option<IndexMap<Lang, T>>,
}

deref_singleton_tuple_struct!(ConfigTitleMetadata, ConfigMetadata<String>);
deref_singleton_tuple_struct!(ConfigLangMetadata, ConfigMetadata<Lang>);
