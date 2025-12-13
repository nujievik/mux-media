use enum_map::Enum;

/// A type of file.
#[derive(Copy, Clone, Debug, Enum)]
#[non_exhaustive]
pub enum FileType {
    Font,
    Media,
}
