use super::Metadata;
use crate::{IsDefault, LangCode};
use std::fmt::{Debug, Display};

impl<T> Metadata<T>
where
    T: Clone + Debug + Display + PartialEq + IsDefault,
{
    pub(crate) fn get(&self, i: &usize, lang: &LangCode) -> Option<&T> {
        if let Some(v) = self.single_val.as_ref() {
            return Some(v);
        }

        if let Some(v) = self.idxs.as_ref().and_then(|xs| xs.get(i)) {
            return Some(v);
        }

        if let Some(v) = self
            .ranges
            .as_ref()
            .and_then(|xs| xs.iter().find_map(|(k, v)| k.contains(i).then_some(v)))
        {
            return Some(v);
        }

        if let Some(v) = self.langs.as_ref().and_then(|xs| xs.get(lang)) {
            return Some(v);
        }

        None
    }
}
