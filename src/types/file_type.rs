use enum_map::Enum;

/// Type of file.
#[derive(Copy, Clone, Debug, Enum)]
pub enum FileType {
    Font,
    Media,
}
