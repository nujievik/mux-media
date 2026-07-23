use super::*;

macro_rules! to_args_impl {
    ($ty:ty, $arg:ident, $max_arg:ident) => {
        impl $crate::ToTxtConfig for $ty {
            fn append_args(&self, args: &mut Vec<std::ffi::OsString>) {
                if let Some(values) = to_args!(@get_values, self) {
                    args.push(to_args!($arg));
                    args.push(values.into());
                }
                if let Some(max) = self.max_in_auto {
                    args.push(to_args!($max_arg));
                    args.push(max.to_string().into());
                }
            }
        }
    };
}

to_args_impl!(DefaultDispositions, Defaults, MaxDefaults);
to_args_impl!(ForcedDispositions, Forceds, MaxForceds);
