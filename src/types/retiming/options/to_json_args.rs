use super::*;

impl crate::ToJsonArgs for RetimingOptions {
    fn append_json_args(&self, args: &mut Vec<String>) {
        let mut parts = String::new();
        if self.inverse {
            parts.push('!');
        }
        if let Some(pat) = self.parts.as_ref() {
            parts.push_str(&pat.raw);
        }

        if !parts.is_empty() {
            args.push(to_json_args!(Parts));
            args.push(parts);
        }

        to_json_args!(@push_true, self, args; no_linked, NoLinked);
    }
}
