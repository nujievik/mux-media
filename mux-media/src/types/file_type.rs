use enum_map::{Enum, EnumMap};

/// Type of file.
#[derive(Copy, Clone, Debug, Enum)]
pub enum FileType {
    Font,
    Media,
}

impl FileType {
    /// Returns a new [`EnumMap<FileType, T>`] with default values.
    pub fn map<T>() -> EnumMap<Self, T>
    where
        T: Default,
    {
        EnumMap::default()
    }
}
