use enum_map::Enum;
use strum_macros::{AsRefStr, EnumIter};

#[derive(Copy, Clone, Debug, PartialEq, AsRefStr, Enum, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum DispositionType {
    Default,
    Forced,
}

impl DispositionType {
    pub(crate) fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
