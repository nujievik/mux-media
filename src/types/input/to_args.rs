use crate::{Input, ToJsonArgs};

impl ToJsonArgs for Input {
    fn append_json_args(&self, args: &mut Vec<String>) {
        todo!();

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

        if self.depth != Self::DEPTH_DEFAULT {
            args.push(to_json_args!(Depth));
            args.push(self.depth.to_string());
        }

        if self.solo {
            args.push(to_json_args!(Solo));
        }
    }
}
