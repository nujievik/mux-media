use super::*;

macro_rules! to_args_impl {
    ($ty:ty, $arg:ident) => {
        impl $crate::ToTxtConfig for $ty {
            fn append_args(&self, args: &mut Vec<std::ffi::OsString>) {
                if let Some(values) = to_args!(@get_values, self) {
                    args.push(to_args!($arg));
                    args.push(values.into());
                }
            }
        }
    };
}

to_args_impl!(ConfigNameMetadata, Names);
to_args_impl!(ConfigLangMetadata, Langs);
