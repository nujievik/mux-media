use strum_macros::EnumIter;

#[derive(Clone, Copy, Default, EnumIter)]
pub enum AttachType {
    #[default]
    Font,
    Other,
}

impl AttachType {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub fn as_str_mkvtoolnix(self) -> &'static str {
        match self {
            Self::Font => "font",
            Self::Other => "",
        }
    }
}
