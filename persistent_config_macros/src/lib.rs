use proc_macro::{TokenStream, TokenTree};
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Persistent, attributes(persistent))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let attributes = input.attrs;
    // println!("Attributes: {:#?}", attributes);

    for attribute in attributes {
        if attribute.path().is_ident("persistent") {
            let token_stream = attribute.to_token_stream();
            // println!("Token {:#?}", token_stream);

            println!("TokenStream {:#?}", parse_key_value_args(token_stream.into()));
        }
    }
    let expanded = quote! {
        impl #impl_generics persistent_config::PersistentConfigBuilder for #name #ty_generics #where_clause {}

    };

    TokenStream::from(expanded)
}

fn parse_key_value_args(tokens: TokenStream) -> Vec<(String, String)> {
    let mut key_values = vec![];
    let mut key = String::new();
    let mut ready_for_value = false;

    for token in tokens {
        match token {
            TokenTree::Punct(punct) => {
                println!("{:#^100}", "");
                println!("Punct: {:#?}", punct);
                if !key.is_empty() && punct.to_string() == "=" {
                    ready_for_value = true;
                    continue;
                }
                key.clear();
            }
            TokenTree::Group(group) => {
                // println!("{:#^100}", "");
                // println!("Group: {:#?}", group);
                parse_key_value_args(group.stream());
            }
            TokenTree::Ident(ident) => {
                println!("{:#^100}", "");
                println!("Ident: {:#?}", ident);
                if ready_for_value {
                    key_values.push((key.clone(), ident.to_string()));
                    key.clear();
                    ready_for_value = false;
                    continue;
                }
                key = ident.to_string();
            }

            TokenTree::Literal(literal) => {
                println!("{:#^100}", "");
                println!("Literal: {:#?}", literal);
            }
        };
    }

    key_values
}
