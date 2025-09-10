use crate::{GlobSetPattern, IsDefault, ToJsonArgs, to_json_args};

#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct RetimingOptions {
    pub rm_segments: Option<GlobSetPattern>,
    pub no_linked: bool,
    pub less_retiming: bool,
}

impl ToJsonArgs for RetimingOptions {
    fn append_json_args(&self, args: &mut Vec<String>) {
        if let Some(pat) = &self.rm_segments {
            if !pat.raw.is_empty() {
                args.push(to_json_args!(RmSegments));
                args.push(pat.raw.clone());
            }
        }

        to_json_args!(@push_true, self, args; no_linked, NoLinked, less_retiming, LessRetiming);
    }
}
