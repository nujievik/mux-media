use super::Specials;
use crate::{ToJsonArgs, json_arg};

impl ToJsonArgs for Specials {
    fn to_json_args(&self) -> Vec<String> {
        match &self.0 {
            Some(args) if !args.is_empty() => {
                let spl = args.join(" ");
                vec![json_arg!(Specials), spl]
            }
            _ => Vec::new(),
        }
    }
}
