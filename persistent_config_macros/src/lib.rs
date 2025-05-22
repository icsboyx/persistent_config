use std::iter;

use anyhow::bail;
use persistent_config_core::PersistentConfigParameters;
use proc_macro::{TokenStream, TokenTree, token_stream};
use quote::{ToTokens, quote};
use serde::Serialize;
use serde_json::Value;
use syn::{DeriveInput, Meta, parse_macro_input};

#[proc_macro_derive(Persistent, attributes(persistent))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let attributes = input.attrs;

    for attribute in attributes {
        if attribute.path().is_ident("persistent") {
            if let Some(path_ident) = attribute.meta.path().get_ident() {
                println!("**************\n {:#?}", attribute.meta.to_token_stream().iter);
                // let mut ret_val = TokenStream::new();
                // attribute.meta.to_tokens(&mut ret_val.into());

                // for val in ret_val {
                //     match val {
                //         TokenTree::Group(group) => {}
                //         TokenTree::Ident(ident) => {}
                //         TokenTree::Punct(punct) => {}
                //         TokenTree::Literal(literal) => {}
                //     }
                // }
                if path_ident.to_string() == "persistent".to_string() {
                    // let ret_val = parse_key_value_args(attribute.meta);
                    // println!("{:#?}", ret_val);
                }
            }
        }
    }
    let expanded = quote! {
        impl #impl_generics persistent_config::PersistentConfigBuilder for #name #ty_generics #where_clause {}

    };

    TokenStream::from(expanded)
}

fn parse_key_value_args(tokens: TokenStream) -> PersistentConfigParameters {
    let mut key_values = vec![];
    let mut key = String::new();
    let mut ready_for_value = false;

    println!("{:#?}", tokens);

    let valid_parameters = ["config_dir", "file_name", "save_format", "panic_on_error"];

    let mut persistent_config_parameters = PersistentConfigParameters::default();

    for token in tokens {
        match token {
            TokenTree::Punct(punct) => {
                if !key.is_empty() && punct.to_string() == "=" {
                    ready_for_value = true;
                    continue;
                }
                key.clear();
            }
            TokenTree::Group(group) => {
                persistent_config_parameters = parse_key_value_args(group.stream());
            }
            TokenTree::Ident(ident) => {
                if ready_for_value {
                    key_values.push((key.clone(), ident.to_string()));
                    key.clear();
                    ready_for_value = false;
                    continue;
                }
                match ident.to_string() {
                    val if valid_parameters.contains(&val.as_str()) => {
                        key = val;
                    }
                    _ => {}
                };
            }

            TokenTree::Literal(_) => {}
        };
    }

    persistent_config_parameters
}
