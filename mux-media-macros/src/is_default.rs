use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[inline(always)]
pub fn body_derive_is_default(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let crate_path = match crate_name("mux-media") {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(found)) => {
            let ident = syn::Ident::new(&found, name.span());
            quote!(::#ident)
        }
        Err(_) => quote!(::mux_media),
    };

    let body = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => {
                let checks = fields_named.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #crate_path::IsDefault::is_default(&self.#name)
                    }
                });
                quote! {
                    #(#checks)&&*
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let checks = fields_unnamed.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! {
                        #crate_path::IsDefault::is_default(&self.#index)
                    }
                });
                quote! {
                    #(#checks)&&*
                }
            }
            Fields::Unit => {
                quote!(true)
            }
        },

        Data::Enum(enum_data) => {
            let mut default_variant_ident = None;

            for variant in &enum_data.variants {
                let has_default_attr = variant.attrs.iter().any(|attr| {
                    let path = attr.path();
                    path.is_ident("is_default") || path.is_ident("default")
                });

                if has_default_attr {
                    if !matches!(variant.fields, Fields::Unit) {
                        return syn::Error::new_spanned(
                            &variant.ident,
                            "the #[default] or #[is_default] attribute may only be used on unit enum variants",
                        )
                        .to_compile_error()
                        .into();
                    }

                    default_variant_ident = Some(&variant.ident);
                    break;
                }
            }

            match default_variant_ident {
                Some(ident) => quote!(matches!(self, #name::#ident)),
                None => quote!(self == &Default::default()),
            }
        }

        Data::Union(_) => {
            return syn::Error::new_spanned(name, "IsDefault cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        impl #impl_generics #crate_path::IsDefault for #name #ty_generics #where_clause {
            fn is_default(&self) -> bool {
                #body
            }
        }
    };

    TokenStream::from(expanded)
}
