use super::Input;
use crate::{ToJsonArgs, json_arg};

impl ToJsonArgs for Input {
    fn to_json_args(&self) -> Vec<String> {
        let mut args = Vec::<String>::new();

        if let Some(dir) = self.dir.to_str() {
            args.push(json_arg!(Input));
            args.push(dir.to_owned());
        }

        if let Some(range) = &self.range {
            args.push(json_arg!(Range));
            args.push(range.to_string());
        }

        if let Some(pat) = &self.skip {
            if !pat.raw.is_empty() {
                args.push(json_arg!(Skip));
                args.push(pat.raw.clone());
            }
        }

        if self.depth != Self::DEFAULT_DEPTH {
            args.push(json_arg!(Depth));
            args.push(self.depth.to_string());
        }

        args
    }
}
