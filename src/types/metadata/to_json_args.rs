use super::*;

macro_rules! to_json_args_impl {
    ($ty:ty, $arg:ident) => {
        impl $crate::ToJsonArgs for $ty {
            fn append_json_args(&self, args: &mut Vec<String>) {
                if let Some(values) = to_json_args!(@get_values, self) {
                    args.push(to_json_args!($arg));
                    args.push(values);
                }
            }
        }
    };
}

to_json_args_impl!(NameMetadata, Names);
to_json_args_impl!(LangMetadata, Langs);
