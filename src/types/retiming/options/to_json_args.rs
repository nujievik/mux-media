use super::*;
use crate::ToJsonArgs;

impl ToJsonArgs for RetimingOptions {
    fn append_json_args(&self, args: &mut Vec<String>) {
        self.parts.append_json_args(args);
        to_json_args!(@push_true, self, args; no_linked, NoLinked);
    }
}

impl ToJsonArgs for RetimingOptionsParts {
    fn append_json_args(&self, args: &mut Vec<String>) {
        let mut arg = String::new();
        if self.inverse {
            arg.push('!');
        }
        if let Some(pat) = self.pattern.as_ref() {
            arg.push_str(&pat.raw);
        }

        if !arg.is_empty() {
            args.push(to_json_args!(Parts));
            args.push(arg);
        }
    }
}
