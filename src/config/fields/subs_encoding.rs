use crate::{MuxError, Result};
use encoding_rs::Encoding;
use std::str::FromStr;

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
