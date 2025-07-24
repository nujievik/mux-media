use super::LangCode;
use once_cell::sync::Lazy;
use std::collections::HashSet;

pub(crate) static SET_MULTIPLE_PRIORITY: Lazy<HashSet<LangCode>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert(LangCode::Chi);
    set.insert(LangCode::Eng);
    set.insert(LangCode::Jpn);
    set.insert(LangCode::Rus);
    set.insert(LangCode::Spa);
    set
});
