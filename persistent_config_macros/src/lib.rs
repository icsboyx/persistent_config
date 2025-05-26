//! # persistent_config_macros
//!
//! This crate provides the `Persistent` derive macro for the
//! [`persistent_config`](https://docs.rs/persistent_config) crate.
//!
//! ## Usage
//!
//! Add `#[derive(Persistent)]` to your struct to automatically implement
//! the `PersistentConfigBuilder` trait, enabling persistent save/load functionality.
//!
//! ```rust
//! use persistent_config::prelude::*;
//! use persistent_config_macros::Persistent;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, Persistent)]
//! struct MyConfig {/* ... */}
//! ```
//!
//! You can also use the `#[persistent(...)]` attribute for future customization.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derive macro for [`PersistentConfigBuilder`](persistent_config::PersistentConfigBuilder).
///
/// This macro automatically implements the trait for your struct, enabling
/// persistent configuration save/load functionality.
///
/// # Example
/// ```rust
/// use persistent_config::prelude::*;
/// use persistent_config_macros::Persistent;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Persistent)]
/// struct MyConfig {/* ... */}
/// ```
#[proc_macro_derive(Persistent, attributes(persistent))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics persistent_config::PersistentConfigBuilder for #name #ty_generics #where_clause {}
    };
    TokenStream::from(expanded).into()
}
