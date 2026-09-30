use super::{ConfigInput, InputType};
use crate::ToTxtConfig;

impl ToTxtConfig for ConfigInput {
    fn append_args(&self, args: &mut Vec<String>) {
        match &self.ty {
            InputType::Dir(dir) => {
                if let Some(s) = dir.to_str() {
                    args.push(to_args!(Input));
                    args.push(s.into());
                }
            }
            InputType::Files(files) => {
                for f in files {
                    if let Some(s) = f.to_str() {
                        args.push(to_args!(Input));
                        args.push(s.into());
                    }
                }
            }
        }

        if let Some(range) = &self.range {
            args.push(to_args!(Range));
            args.push(range.to_string());
        }

        if let Some(pat) = &self.skip {
            if !pat.raw.is_empty() {
                args.push(to_args!(Skip));
                args.push(String::from(&pat.raw));
            }
        }

        if self.depth != Self::DEPTH_DEFAULT {
            args.push(to_args!(Depth));
            args.push(self.depth.to_string());
        }

        if self.solo {
            args.push(to_args!(Solo));
        }
    }
}
