#[doc(hidden)]
#[macro_export]
macro_rules! parseable_args {
    ($ty:ty, $enum_arg:ident; $( $arg:ident => $long:expr ),* $(,)?) => {
        impl $crate::ParseableArgs for $ty {
            type Arg = $enum_arg;
        }

        #[doc = concat!("[`ParseableArgs`]($crate::ParseableArgs)
        assotiated with the [`", stringify!($ty), "`].")]
        #[derive(Copy, Clone)]
        pub enum $enum_arg {
            $( $arg ),*
        }

        impl $crate::ParseableArg for $enum_arg {
            fn dashed(self) -> &'static str {
                match self {
                    $( Self::$arg => concat!("--", $long) ),*
                }
            }

            fn undashed(self) -> &'static str {
                match self {
                    $( Self::$arg => $long ),*
                }
            }
        }
    };
}
