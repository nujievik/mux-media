#[doc(hidden)]
#[macro_export]
macro_rules! deref_singleton_tuple_fields {
    ($wrapper:ty, $inner:ty) => {
        impl std::ops::Deref for $wrapper {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $wrapper {
            pub fn inner(&self) -> &$inner {
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

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<$inner>().map(Self).map_err(Into::into)
            }
        }
    };

    ($wrapper:ty, @builders, $( $field:ident : $ty:ty ),* $(,)?) => {
        impl $wrapper { $(
            pub fn $field(mut self, val: $ty) -> Self {
                self.0.$field = val;
                self
            }
        )* }
    };

    ($wrapper:ty, $inner:ty, @all) => {
        $crate::deref_singleton_tuple_fields!($wrapper, $inner);
        $crate::deref_singleton_tuple_fields!($wrapper, @default);
        $crate::deref_singleton_tuple_fields!($wrapper, $inner, @from_str);
    };

    ($wrapper:ty, $inner:ty, @all, $( $field:ident : $ty:ty ),* $(,)?) => {
        $crate::deref_singleton_tuple_fields!($wrapper, $inner, @all);
        $crate::deref_singleton_tuple_fields!($wrapper, @builders, $( $field : $ty ),* );
    };
}
