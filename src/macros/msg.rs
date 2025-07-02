#[macro_export]
macro_rules! msg {
    ($as_lang:ident, $( $enum_var:ident => $text:expr ),* $(,)?) => {
        impl $crate::Msg {
            pub(crate) fn $as_lang(self) -> &'static str {
                match self {
                    $( Self::$enum_var => $text ),*
                }
            }
        }
    };

    ($as_lang:ident, @inline, $( $enum_var:ident => $text:expr ),* $(,)?) => {
        impl $crate::Msg {
            #[inline(always)]
            pub(crate) fn $as_lang(self) -> &'static str {
                match self {
                    $( Self::$enum_var => $text ),*
                }
            }
        }
    };
}
