use super::{DefaultTFlags, EnabledTFlags, ForcedTFlags};
use crate::{IsDefault, ToJsonArgs, json_arg, to_json_args};

macro_rules! flags_to_json_args {
    ($ty:ty, $arg:ident, $lim_arg:ident) => {
        impl ToJsonArgs for $ty {
            fn to_json_args(&self) -> Vec<String> {
                if self.is_default() {
                    return Vec::new();
                }

                if let Some(unmapped) = self.unmapped {
                    return vec![json_arg!($arg), unmapped.to_string()];
                }

                let mut args: Vec<String> = Vec::new();

                if let Some(lim) = self.lim_for_unset {
                    args.push(json_arg!($lim_arg));
                    args.push(lim.to_string());
                }

                let id_map = to_json_args!(@collect_id_map, self);

                if id_map.is_empty() {
                    return args;
                }

                let id_map = id_map.into_iter().collect::<Vec<String>>().join(",");

                args.push(json_arg!($arg));
                args.push(id_map);

                args
            }
        }
    };
}

flags_to_json_args!(DefaultTFlags, Defaults, LimDefaults);
flags_to_json_args!(ForcedTFlags, Forceds, LimForceds);
flags_to_json_args!(EnabledTFlags, Enableds, LimEnableds);
