use crate::{LangCode, Result, ToArgs};
use core::fmt::NumBuffer;
use std::io::Write;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigLocales {
    pub(crate) locale: LangCode,
    pub(crate) locale_streams: Option<LangCode>,
    pub(crate) locale_log: Option<LangCode>,
}

impl ConfigLocales {
    #[inline]
    pub const fn locale(&self) -> LangCode {
        self.locale
    }

    #[inline]
    pub const fn locale_streams(&self) -> LangCode {
        match self.locale_streams {
            Some(l) => l,
            None => self.locale,
        }
    }

    #[inline]
    pub const fn locale_log(&self) -> LangCode {
        match self.locale_log {
            Some(l) => l,
            None => self.locale,
        }
    }

    #[inline]
    pub const fn get_locale_streams(&self) -> Option<LangCode> {
        self.locale_streams
    }

    #[inline]
    pub const fn get_locale_log(&self) -> Option<LangCode> {
        self.locale_log
    }
}

impl ToArgs for ConfigLocales {
    fn write_with_num_buffer<W>(&self, w: &mut W, _: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        to_args!(w, Locale)?;
        to_args!(w, self.locale.as_ref().as_bytes(), @v)?;

        if let Some(l) = self.locale_streams {
            to_args!(w, LocaleStreams)?;
            to_args!(w, l.as_ref().as_bytes(), @v)?;
        }

        if let Some(l) = self.locale_log {
            to_args!(w, LocaleLog)?;
            to_args!(w, l.as_ref().as_bytes(), @v)?;
        }

        Ok(())
    }
}
