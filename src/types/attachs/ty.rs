use strum_macros::EnumIter;

/// Type of media attachment.
#[derive(Copy, Clone, Debug, Default, PartialEq, EnumIter)]
pub enum AttachType {
    #[default]
    Font,
    Other,
}

/*
impl AttachType {
    /// Returns an iterator over all variants of `AttachType`.
    pub(crate) fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    /// Returns the string marker for `AttachType` used in mkvtoolnix tools.
    pub(crate) fn as_str_mkvtoolnix(self) -> &'static str {
        match self {
            Self::Font => "font",
            Self::Other => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut it = AttachType::iter();
        assert_eq!(it.next().unwrap(), AttachType::Font);
        assert_eq!(it.next().unwrap(), AttachType::Other);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_as_str_mkvtoolnix() {
        assert_eq!(AttachType::Font.as_str_mkvtoolnix(), "font");
        assert_eq!(AttachType::Other.as_str_mkvtoolnix(), "");
    }
}
*/
