use strum_macros::EnumIter;

/// Type of media attachment.
#[derive(Copy, Clone, Default, EnumIter)]
pub enum AttachType {
    #[default]
    Font,
    Other,
}

impl AttachType {
    /// Returns an iterator over all variants of `AttachType`.
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    /// Returns the string marker for `AttachType` used in mkvtoolnix tools.
    pub fn as_str_mkvtoolnix(self) -> &'static str {
        match self {
            Self::Font => "font",
            Self::Other => "",
        }
    }
}
