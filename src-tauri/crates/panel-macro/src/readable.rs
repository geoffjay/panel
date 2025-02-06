use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Meta};

#[proc_macro_derive(ReadableNext, attributes(entity, table))]
pub fn readable_derive_next(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    proc_macro::TokenStream::from(impl_readable(input))
}

fn impl_readable(input: DeriveInput) -> TokenStream {
    let mut entity = None;
    let mut table = None;

    for attr in input.attrs {
        match &attr {
            Meta::List(list) if list.path.is_ident("entity") => {
                list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                    .map_err(|_| {
                        syn::Error::new(
                            list.span(),
                            indoc! {r#"
                                The `entity` attribute expects a string literal

                                = help: use `#[entity("Project")]`
                            "#}
                        )
                    })?;
            }
            Meta::List(list) if list.path.is_ident("table") => {
                list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                    .map_err(|_| {
                        syn::Error::new(
                            list.span(),
                            indoc! {r#"
                                The `table` attribute expects a string literal

                                = help: use `#[table("projects")]`
                            "#}
                        )
                    })?;
            }
            meta => {
                // returning a syn::Error would help with the compiler diagnostics and guide your macro users to get it right
                return Err(syn::Error::new(
                    meta.span(),
                    indoc! {r#"
                        The `entity` attribute is the only supported argument

                        = help: use `#[entity("Project")]`
                    "#})
                );
            }
        }
    }
}
