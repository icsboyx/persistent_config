use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{LazyLock, RwLock};

pub use anyhow::{Error, Result};
use quote::ToTokens;

pub static PERSISTENT_CONFIGS: LazyLock<RwLock<PersistentConfigDB>> =
    LazyLock::new(|| RwLock::new(PersistentConfigDB::default()));

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SaveFormat {
    JSON,
    #[default]
    TOML,
    YAML,
}

impl SaveFormat {
    pub fn ext(&self) -> &str {
        match self {
            SaveFormat::JSON => "json",
            SaveFormat::TOML => "toml",
            SaveFormat::YAML => "yaml",
        }
    }
}

impl TryFrom<SaveFormat> for String {
    type Error = &'static str;

    fn try_from(value: SaveFormat) -> Result<Self, Self::Error> {
        match value {
            SaveFormat::JSON => Ok("json".to_string()),
            SaveFormat::TOML => Ok("toml".to_string()),
            SaveFormat::YAML => Ok("yaml".to_string()),
        }
    }
}

impl TryFrom<String> for SaveFormat {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "json" => Ok(SaveFormat::JSON),
            "toml" => Ok(SaveFormat::TOML),
            "yaml" => Ok(SaveFormat::YAML),
            _ => Err("Unsupported format: use 'json', 'toml', or 'yaml'"),
        }
    }
}

impl ToTokens for SaveFormat {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ext = self.ext();
        tokens.extend(quote::quote! {
            #ext
        });
    }
}

#[derive(Debug, Clone)]

pub struct PersistentConfigParameters {
    pub config_dir: String,
    pub file_name: String,
    pub save_format: SaveFormat,
    pub panic_on_error: bool,
}

impl Default for PersistentConfigParameters {
    fn default() -> Self {
        Self {
            config_dir: "./".into(),
            file_name: String::new(),
            save_format: SaveFormat::default(),
            panic_on_error: true,
        }
    }
}
impl PersistentConfigParameters {
    pub fn merge_from(&mut self, other: Self) {
        if other.config_dir != String::new() {
            self.config_dir = other.config_dir;
        }
        if other.file_name != String::new() {
            self.file_name = other.file_name;
        }
        if other.save_format != Default::default() {
            self.save_format = other.save_format;
        }
        self.panic_on_error = other.panic_on_error;
    }
}

impl quote::ToTokens for PersistentConfigParameters {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let config_dir = &self.config_dir;
        let file_name = &self.file_name;
        let save_format = &self.save_format;
        let panic_on_error = &self.panic_on_error;

        tokens.extend(quote::quote! {
            PersistentConfigParameters {
                config_dir: #config_dir,
                file_name: #file_name,
                save_format: #save_format,
                panic_on_error: #panic_on_error,
            }
        });
    }
}

// impl Parse for PersistentConfigParameters {}

#[derive(Debug, Default, Clone)]
pub struct PersistentConfigDB {
    map: HashMap<TypeId, PersistentConfigParameters>,
}

impl PersistentConfigDB {
    pub fn add_config<T: 'static>(&mut self, config: PersistentConfigParameters) {
        let type_id = TypeId::of::<T>();
        self.map.insert(type_id, config);
    }

    pub fn get_config<T: 'static>(&self) -> Option<PersistentConfigParameters> {
        let type_id = TypeId::of::<T>();
        self.map.get(&type_id).cloned()
    }

    pub fn get_config_mut<T: 'static>(&mut self) -> Option<&mut PersistentConfigParameters> {
        let type_id = TypeId::of::<T>();
        self.map.get_mut(&type_id)
    }
}
