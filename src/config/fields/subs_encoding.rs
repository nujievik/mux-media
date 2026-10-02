use crate::{MuxError, Result, ToArgs};
use core::fmt::NumBuffer;
use encoding_rs::Encoding;
use std::{io::Write, str::FromStr};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfigSubsEncoding(Option<&'static Encoding>);

impl ConfigSubsEncoding {
    #[inline]
    pub fn get_encoding(&self) -> Option<&'static Encoding> {
        self.0
    }
}

impl FromStr for ConfigSubsEncoding {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<ConfigSubsEncoding> {
        let enc = Encoding::for_label_no_replacement(s.as_bytes())
            .ok_or_else(|| err!("unrecognized encoding"))?;

        Ok(Self(Some(enc)))
    }
}

impl ToArgs for ConfigSubsEncoding {
    fn write_with_num_buffer<W>(&self, w: &mut W, _: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if let Some(enc) = self.0.as_ref() {
            to_args!(w, SubsEncoding)?;
            to_args!(w, enc.name().as_bytes(), @v)?;
        }
        Ok(())
    }
}
