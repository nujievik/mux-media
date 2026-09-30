use crate::ToTxtConfig;

impl ToTxtConfig for super::ConfigOutput {
    fn append_args(&self, args: &mut Vec<String>) {
        if let Some(s) = self.dir.to_str() {
            args.push(to_args!(Output));
            args.push(s.into());
        }
    }
}
