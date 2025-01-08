extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput};

#[proc_macro_derive(Readable, attributes(entity, table))]
pub fn readable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    proc_macro::TokenStream::from(impl_readable(input))
}

fn impl_readable(input: DeriveInput) -> TokenStream {
    // Get the struct name (e.g., FooRepository)
    let struct_name = &input.ident;

    // Extract entity and table names from attributes
    let entity_type = "Project".to_string();
    let table_name = "projects".to_string();

    // for attr in &input.attrs {
    //     if let Some(ident) = attr.path().get_ident() {
    //         match ident.to_string().as_str() {
    //             "entity" => {
    //                 entity_type = Some(parse_entity_attribute(attr));
    //             }
    //             "table" => {
    //                 table_name = Some(parse_table_attribute(attr));
    //             }
    //             _ => continue,
    //         }
    //     }
    // }

    // let entity_type = entity_type.unwrap().unwrap();
    // let table_name = table_name.unwrap().unwrap();
    
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

fn parse_entity_attribute(attr: &Attribute) -> syn::Result<String> {
    let meta = attr.parse_args::<syn::Meta>()?;
    println!("{:?}", meta);
    // if let syn::Meta::NameValue(meta) = meta {
    //     if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) = meta.value {
    //         return Ok(lit_str.value());
    //     }
    // }
    Ok(String::new())
}

fn parse_table_attribute(attr: &Attribute) -> syn::Result<String> {
    let meta = attr.parse_args::<syn::Meta>()?;
    println!("{:?}", meta);
    // if let syn::Meta::NameValue(meta) = meta {
    //     if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) = meta.value {
    //         return Ok(lit_str.value());
    //     }
    // }
    Ok(String::new())
}
