use crate::{Output, ToTxtConfig};
use std::ffi::OsString;

impl ToTxtConfig for Output {
    fn append_args(&self, args: &mut Vec<OsString>) {
        args.push(to_args!(Output));
        args.push(OsString::from(&self.dir));
    }
}
