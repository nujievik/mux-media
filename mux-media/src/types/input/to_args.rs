use crate::{Input, ToJsonArgs, to_json_args};

impl ToJsonArgs for Input {
    fn append_json_args(&self, args: &mut Vec<String>) {
        if let Some(dir) = self.dir.to_str() {
            args.push(to_json_args!(Input));
            args.push(dir.to_owned());
        }

        if let Some(range) = &self.range {
            args.push(to_json_args!(Range));
            args.push(range.to_string());
        }

        if let Some(pat) = &self.skip {
            if !pat.raw.is_empty() {
                args.push(to_json_args!(Skip));
                args.push(pat.raw.clone());
            }
        }

        if self.depth != Self::DEFAULT_DEPTH {
            args.push(to_json_args!(Depth));
            args.push(self.depth.to_string());
        }
    }
}
