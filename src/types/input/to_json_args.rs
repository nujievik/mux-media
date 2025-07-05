use super::Input;
use crate::ToJsonArgs;

impl ToJsonArgs for Input {
    fn to_json_args(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        if let Some(dir) = self.dir.to_str() {
            args.push("-i".into());
            args.push(dir.into());
        }

        if let Some(range) = &self.range {
            args.push("--range".into());
            args.push(format!("{}-{}", range.start, range.end));
        }

        if self.up != Self::default_up() {
            args.push("--up".into());
            args.push(self.up.to_string());
        }

        if self.check != Self::default_check() {
            args.push("--check".into());
            args.push(self.check.to_string());
        }

        if self.down != Self::default_down() {
            args.push("--down".into());
            args.push(self.down.to_string());
        }

        args
    }
}
