mod is_default;

use proc_macro::TokenStream;

/// Derive macro generating an impl of the trait `IsDefault`.
#[proc_macro_derive(IsDefault, attributes(is_default, default))]
pub fn derive_is_default(input: TokenStream) -> TokenStream {
    is_default::body_derive_is_default(input)
}
