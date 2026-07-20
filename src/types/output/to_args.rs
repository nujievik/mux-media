use crate::{Output, ToJsonArgs};

impl ToJsonArgs for Output {
    fn append_json_args(&self, args: &mut Vec<String>) {
        let path = if let Some(s) = self.dir.as_os_str().to_str() {
            String::from(s)
        } else {
            return;
        };

        args.push(to_json_args!(Output));
        args.push(path);
    }
}
