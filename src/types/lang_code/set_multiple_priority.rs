use super::LangCode;
use std::{collections::HashSet, sync::LazyLock};

pub(crate) static SET_MULTIPLE_PRIORITY: LazyLock<HashSet<LangCode>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert(LangCode::Chi);
    set.insert(LangCode::Eng);
    set.insert(LangCode::Jpn);
    set.insert(LangCode::Rus);
    set.insert(LangCode::Spa);
    set
});
