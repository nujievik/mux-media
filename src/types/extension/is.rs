use super::Extension;

macro_rules! is_any {
    ($self:ident, $( $ext:ident )* ) => {
        matches!($self, $( Extension::$ext )|+)
    }
}

impl Extension {
    pub(crate) fn is_attach(&self) -> bool {
        is_any!(self, Jpeg Jpg Png)
    }

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
}
