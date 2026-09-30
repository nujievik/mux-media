use super::{ConfigLangMetadata, ConfigTitleMetadata};
use crate::{IsDefault, Result, ToArgs};
use core::fmt::NumBuffer;
use std::io::Write;

macro_rules! meta_to_args_impl {
    ($ty:ty, $arg:ident) => {
        impl ToArgs for $ty {
            fn write_with_num_buffer<W>(&self, w: &mut W, buf: &mut NumBuffer<usize>) -> Result<()>
            where
                W: Write + ?Sized,
            {
                if self.is_default() {
                    return Ok(());
                }

                to_args!(w, $arg)?;
                to_args!(w, self, @write_map, buf);

                Ok(())
            }
        }
    };
}

meta_to_args_impl!(ConfigTitleMetadata, Titles);
meta_to_args_impl!(ConfigLangMetadata, Langs);
