use super::*;
use crate::ToTxtConfig;
use std::ffi::OsString;

impl ToTxtConfig for RetimingOptions {
    fn append_args(&self, args: &mut Vec<OsString>) {
        self.parts.append_args(args);
        to_args!(@push_true, self, args; no_linked, NoLinked);
    }
}

impl ToTxtConfig for RetimingOptionsParts {
    fn append_args(&self, args: &mut Vec<OsString>) {
        let mut arg = String::new();
        if self.inverse {
            arg.push('!');
        }
        if let Some(pat) = self.pattern.as_ref() {
            arg.push_str(&pat.raw);
        }

        if !arg.is_empty() {
            args.push(to_args!(Parts));
            args.push(arg.into());
        }
    }
}
