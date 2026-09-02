use super::{Input, InputType};
use crate::ToTxtConfig;
use std::ffi::OsString;

impl ToTxtConfig for Input {
    fn append_args(&self, args: &mut Vec<OsString>) {
        match &self.ty {
            InputType::Dir(dir) => {
                args.push(to_args!(Input));
                args.push(dir.into());
            }
            InputType::Files(xs) => {
                for x in xs {
                    args.push(to_args!(Input));
                    args.push(x.into());
                }
            }
        }

        if let Some(range) = &self.range {
            args.push(to_args!(Range));
            args.push(range.to_string().into());
        }

        if let Some(pat) = &self.skip {
            if !pat.raw.is_empty() {
                args.push(to_args!(Skip));
                args.push(OsString::from(&pat.raw));
            }
        }

        if self.depth != Self::DEPTH_DEFAULT {
            args.push(to_args!(Depth));
            args.push(self.depth.to_string().into());
        }

        if self.solo {
            args.push(to_args!(Solo));
        }
    }
}
