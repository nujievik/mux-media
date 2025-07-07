use super::Input;
use crate::{ToJsonArgs, json_arg};

impl ToJsonArgs for Input {
    fn to_json_args(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        if let Some(dir) = self.dir.to_str() {
            args.push(json_arg!(Input));
            args.push(dir.into());
        }

        if let Some(range) = &self.range {
            args.push(json_arg!(Range));
            args.push(range.to_string());
        }

        if let Some(pat) = &self.skip {
            if !pat.raw.is_empty() {
                args.push(json_arg!(Skip));
                args.push(pat.raw.to_string());
            }
        }

        if self.up != Self::DEFAULT_UP {
            args.push(json_arg!(Up));
            args.push(self.up.to_string());
        }

        if self.check != Self::DEFAULT_CHECK {
            args.push(json_arg!(Check));
            args.push(self.check.to_string());
        }

        if self.down != Self::DEFAULT_DOWN {
            args.push(json_arg!(Down));
            args.push(self.down.to_string());
        }

        args
    }
}
