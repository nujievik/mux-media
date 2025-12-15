use super::Extension;

macro_rules! is_any {
    ($self:ident, $( $ext:ident )* ) => {
        matches!($self, $( Extension::$ext )|+)
    }
}

impl Extension {
    pub(crate) fn is_font(&self) -> bool {
        is_any!(self, Otf Ttf)
    }

    pub(crate) fn is_matroska(&self) -> bool {
        is_any!(self, Mka Mks Mkv Webm)
    }

    pub(crate) fn is_media(&self) -> bool {
        !self.is_font()
    }

    pub(crate) fn is_subs(&self) -> bool {
        is_any!(self, Ass Mks Srt Ssa Sub Sup Vtt)
    }

    pub(crate) fn new_and_is_font(bytes: &[u8]) -> bool {
        Self::new(bytes).is_some_and(|ext| ext.is_font())
    }

    pub(crate) fn new_and_is_matroska(bytes: &[u8]) -> bool {
        Self::new(bytes).is_some_and(|ext| ext.is_matroska())
    }

    pub(crate) fn new_and_is_media(bytes: &[u8]) -> bool {
        Self::new(bytes).is_some_and(|ext| ext.is_media())
    }

    pub(crate) fn new_and_is_subs(bytes: &[u8]) -> bool {
        Self::new(bytes).is_some_and(|ext| ext.is_subs())
    }
}
