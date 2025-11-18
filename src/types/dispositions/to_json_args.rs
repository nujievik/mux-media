use super::*;

macro_rules! to_json_args_impl {
    ($ty:ty, $arg:ident, $max_arg:ident) => {
        impl $crate::ToJsonArgs for $ty {
            fn append_json_args(&self, args: &mut Vec<String>) {
                if let Some(values) = to_json_args!(@get_values, self) {
                    args.push(to_json_args!($arg));
                    args.push(values);
                }
                if let Some(max) = self.max_in_auto {
                    args.push(to_json_args!($max_arg));
                    args.push(max.to_string());
                }
            }
        }
    };
}

to_json_args_impl!(DefaultDispositions, Defaults, MaxDefaults);
to_json_args_impl!(ForcedDispositions, Forceds, MaxForceds);
