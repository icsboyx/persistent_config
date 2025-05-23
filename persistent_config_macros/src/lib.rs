use persistent_config_core::{self, PersistentConfigParameters, ctor};
use proc_macro::token_stream;
use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt, format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Persistent, attributes(persistent))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let attributes = input.attrs;
    let mut persistent_config = PersistentConfigParameters::default();

    for attribute in attributes {
        if attribute.path().is_ident("persistent") {
            let mut ret_val = proc_macro2::TokenStream::new();
            attribute.meta.to_tokens(&mut ret_val);
            for val in ret_val {
                match val {
                    proc_macro2::TokenTree::Group(group) => {
                        persistent_config.merge_from(parse_key_value_args(group.stream()));
                        println!("############\n{:?}", persistent_config);
                    }
                    _ => {}
                }
            }
        }
    }

    let config_dir = persistent_config.config_dir.clone();
    let file_name = persistent_config.file_name.clone();
    let save_format = persistent_config.save_format.clone();
    let panic_on_error = persistent_config.panic_on_error.clone();

    let register_fn = format_ident!("__register_config_for_{}", name);
    let expanded = quote! {
        impl #impl_generics persistent_config::PersistentConfigBuilder for #name #ty_generics #where_clause {}

        #[persistent_config_core::ctor::ctor]
        fn #register_fn() {
            let config = persistent_config::PersistentConfigParameters {
                config_dir: #config_dir.to_string(),
                file_name: #file_name.to_string(),
                save_format: persistent_config::SaveFormat::try_from(#save_format).unwrap_or(persistent_config::SaveFormat::default()),
                panic_on_error: #panic_on_error,
            };
            persistent_config::PERSISTENT_CONFIGS
                .write()
                .unwrap()
                .add_config::<#name>(config);
        }
    };

    println!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE\n{}", expanded.to_string());

    TokenStream::from(expanded).into()
}

fn parse_key_value_args(tokens: TokenStream) -> PersistentConfigParameters {
    let mut key_values = vec![];
    let mut key = String::new();
    let mut ready_for_value = false;

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
                match ident.to_string() {
                    val if valid_parameters.contains(&val.as_str()) => {
                        key = val;
                    }
                    _ => {}
                };
            }

            TokenTree::Literal(lit) => {
                if ready_for_value {
                    let val = lit.to_string();
                    key_values.push((key.clone(), val.trim_matches('"').to_string()));
                    key.clear();
                    ready_for_value = false;
                    continue;
                }
            }
        };
    }

    for (key, val) in key_values {
        match key.as_str() {
            "panic_on_error" => {
                persistent_config_parameters.panic_on_error = val
                    .parse::<bool>()
                    .unwrap_or_else(|e| panic!("Failed to parse panic_on_error: {}. Error: {}", val, e));
            }
            "file_name" => {
                persistent_config_parameters.file_name = val;
            }
            "config_dir" => {
                persistent_config_parameters.config_dir = val;
            }
            "save_format" => {
                persistent_config_parameters.save_format = val.try_into().unwrap();
            }

            _ => {
                println!("Unknown parameter: {}", key);
            }
        }
    }

    persistent_config_parameters
}
