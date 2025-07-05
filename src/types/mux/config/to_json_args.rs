use super::MuxConfig;
use crate::{ToJsonArgs, types::helpers::try_write_args_to_json};

impl MuxConfig {
    pub(crate) fn write_args_to_json_or_log(&self) {
        let args = self.to_json_args();

        if args.is_empty() {
            return;
        }

        let json = self.input.get_dir().join(Self::JSON_NAME);
        match try_write_args_to_json(args, &json) {
            Ok(_) => {}
            Err(e) => log::warn!("Fail save current config to json: {}", e),
        }
    }
}

impl ToJsonArgs for MuxConfig {
    fn to_json_args(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        args.append(&mut self.input.to_json_args());

        args
    }
}
