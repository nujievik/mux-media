#[doc(hidden)]
#[macro_export]
macro_rules! deref_singleton_tuple_struct {
    ($wrapper:ty, $inner:ty) => {
        impl std::ops::Deref for $wrapper {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    ($wrapper:ty, @default) => {
        impl Default for $wrapper {
            fn default() -> Self {
                Self(Default::default())
            }
        }
    };

    ($wrapper:ty, $inner:ty, @from_str) => {
        impl std::str::FromStr for $wrapper {
            type Err = $crate::MuxError;

            fn from_str(s: &str) -> $crate::Result<Self> {
                s.parse::<$inner>().map(Self).map_err(Into::into)
            }
        }
    };

    ($wrapper:ty, $inner:ty, @all) => {
        $crate::deref_singleton_tuple_struct!($wrapper, $inner);
        $crate::deref_singleton_tuple_struct!($wrapper, @default);
        $crate::deref_singleton_tuple_struct!($wrapper, $inner, @from_str);
    };
}
