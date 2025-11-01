use super::Raws;
use crate::{ToJsonArgs, dashed};

impl ToJsonArgs for Raws {
    fn append_json_args(&self, args: &mut Vec<String>) {
        if let Some(spls) = self.0.as_ref().filter(|spls| !spls.is_empty()) {
            args.push(dashed!(Raws).into());
            args.push(spls.join(" "));
        }
    }
}
