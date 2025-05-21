use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{LazyLock, RwLock};

pub use anyhow::{Error, Result};

pub static PERSISTENT_CONFIGS: LazyLock<RwLock<PersistentConfigDB>> =
    LazyLock::new(|| RwLock::new(PersistentConfigDB::default()));

#[derive(Debug, Clone, Copy, Default)]
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

impl TryInto<SaveFormat> for &str {
    type Error = &'static str;

    fn try_into(self) -> Result<SaveFormat, Self::Error> {
        match self.to_lowercase().as_str() {
            "json" => Ok(SaveFormat::JSON),
            "toml" => Ok(SaveFormat::TOML),
            "yaml" => Ok(SaveFormat::YAML),
            _ => Err("Unsupported format: use 'json', 'toml', or 'yaml'"),
        }
    }
}

impl std::str::FromStr for SaveFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(SaveFormat::JSON),
            "toml" => Ok(SaveFormat::TOML),
            "yaml" => Ok(SaveFormat::YAML),
            _ => Err("Unsupported format: use 'json', 'toml', or 'yaml'"),
        }
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
            config_dir: String::new(),
            file_name: String::new(),
            save_format: SaveFormat::default(),
            panic_on_error: true,
        }
    }
}

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
