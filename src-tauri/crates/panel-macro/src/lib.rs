extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Readable, attributes(entity, table))]
pub fn readable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    proc_macro::TokenStream::from(impl_readable(input))
}

fn impl_readable(input: DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let entity_type: String = fetch_attr("entity", &input.attrs)
        .expect("Please supply an entity attribute")
        .parse()
        .expect("entity should be a string");
    let table_name: String = fetch_attr("table", &input.attrs)
        .expect("Please supply a table attribute")
        .parse()
        .expect("table should be a string");
    let entity_ident = syn::Ident::new(&entity_type, proc_macro2::Span::call_site());
    let table_ident = syn::Ident::new(&table_name, proc_macro2::Span::call_site());

    quote! {
        impl #struct_name {
            pub fn find(connection: &mut ConnectionType, id: i32) -> Result<#entity_ident, diesel::result::Error> {
                use crate::schema::#table_ident::dsl;
                dsl::#table_ident.find(id).first(connection)
            }
        }
    }
}

/// Fetch an attribute string from the derived struct.
fn fetch_attr(name: &str, attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if let Ok(meta) = attr.parse_args::<syn::Meta>() {
            match meta {
                syn::Meta::NameValue(nv) => {
                    if nv.path.get_ident().map(|i| i.to_string()) == Some(name.to_string()) {
                        match nv.value {
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(lit_str),
                                ..
                            }) => return Some(lit_str.value()),
                            _ => {
                                panic!("attribute {} should be a string", name);
                            }
                        }
                    }
                }
                _ => {
                    panic!("attribute {} should be a string", name);
                }
            }
        }
    }

    None
}
