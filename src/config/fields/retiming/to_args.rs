use super::*;
use crate::ToTxtConfig;

impl ToTxtConfig for ConfigRetiming {
    fn append_args(&self, args: &mut Vec<String>) {
        self.parts.append_args(args);
        to_args!(@push_true, self, args; no_linked, NoLinked);
    }
}

impl ToTxtConfig for ConfigRetimingParts {
    fn append_args(&self, args: &mut Vec<String>) {
        let mut arg = String::new();
        if self.inverse {
            arg.push('!');
        }
        if let Some(pat) = self.pattern.as_ref() {
            arg.push_str(&pat.raw);
        }

        if !arg.is_empty() {
            args.push(to_args!(Parts));
            args.push(arg);
        }
    }
}
