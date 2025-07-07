#[macro_export]
macro_rules! impl_msg_as_localized_str {
    ($as_lang:ident, $( $enum_var:ident => $text:expr ),* $(,)?) => {
        impl $crate::Msg {
            #[inline(always)]
            pub(in crate::i18n) fn $as_lang(self) -> &'static str {
                match self {
                    $( Self::$enum_var => $text ),*
                }
            }
        }
    };
}
