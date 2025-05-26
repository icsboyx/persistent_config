//! Core types and utilities for persistent configuration management.
//!
//! This module provides the [`PersistentConfigDB`] for storing configuration parameters
//! for different types, as well as the [`SaveFormat`] enum and related helpers.

use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{LazyLock, RwLock};

/// Re-exported error and result types from `anyhow`.
pub use anyhow::{Error, Result};

/// Global static database for persistent configuration parameters.
pub static PERSISTENT_CONFIGS: LazyLock<PersistentConfigDB> = LazyLock::new(|| PersistentConfigDB::default());

/// Supported formats for saving configuration files.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SaveFormat {
    /// JSON format (`.json`)
    JSON,
    /// TOML format (`.toml`)
    #[default]
    TOML,
    /// YAML format (`.yaml`)
    YAML,
}

impl SaveFormat {
    /// Returns the file extension associated with this format.
    pub fn ext(&self) -> &str {
        match self {
            SaveFormat::JSON => "json",
            SaveFormat::TOML => "toml",
            SaveFormat::YAML => "yaml",
        }
    }
}

/// Converts a [`SaveFormat`] to its string representation.
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

/// Parses a [`SaveFormat`] from a string slice.
impl TryFrom<&'_ str> for SaveFormat {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "json" => Ok(SaveFormat::JSON),
            "toml" => Ok(SaveFormat::TOML),
            "yaml" => Ok(SaveFormat::YAML),
            _ => Err("Unsupported format: use 'json', 'toml', or 'yaml'"),
        }
    }
}

/// Parses a [`SaveFormat`] from a [`String`].
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

/// Parameters for a persistent configuration instance.
///
/// # Default Values
/// - `config_dir`: `"./config"`
/// - `file_name`: `""` (empty string)
/// - `save_format`: [`SaveFormat::TOML`] (default format)
/// - `panic_on_error`: `true`
///
/// Use [`PersistentConfigParameters::default()`] to get these defaults.
///
/// # Example
/// ```
/// let params = PersistentConfigParameters::default();
/// assert_eq!(params.config_dir, "./config");
/// assert_eq!(params.file_name, "");
/// assert_eq!(params.save_format, SaveFormat::TOML);
/// assert!(params.panic_on_error);
/// ```
#[derive(Debug, Clone)]
pub struct PersistentConfigParameters {
    /// Directory where the config file is stored.
    pub config_dir: String,
    /// Name of the config file.
    pub file_name: String,
    /// Format used to save the config file.
    pub save_format: SaveFormat,
    /// Whether to panic on error.
    pub panic_on_error: bool,
}

impl Default for PersistentConfigParameters {
    /// Returns the default parameters:
    /// - `config_dir`: `"./config"`
    /// - `file_name`: `""`
    /// - `save_format`: [`SaveFormat::TOML`]
    /// - `panic_on_error`: `true`
    fn default() -> Self {
        Self {
            config_dir: String::new(),
            file_name: String::new(),
            save_format: SaveFormat::default(),
            panic_on_error: true,
        }
    }
}

/// Database for storing persistent configuration parameters for different types.
#[derive(Debug, Default)]
pub struct PersistentConfigDB {
    /// Internal map from type ID to configuration parameters.
    map: RwLock<HashMap<TypeId, PersistentConfigParameters>>,
}

impl PersistentConfigDB {
    /// Add configuration parameters for a type.
    ///
    /// # Type Parameters
    /// * `T`: The type for which to store the configuration.
    pub fn add_config<T: 'static>(&self, config: PersistentConfigParameters) {
        let type_id = TypeId::of::<T>();
        self.map
            .write()
            .expect("Unable to lock, for adding config.")
            .insert(type_id, config);
    }

    /// Get configuration parameters for a type.
    ///
    /// # Type Parameters
    /// * `T`: The type for which to retrieve the configuration.
    pub fn get_config<T: 'static>(&self) -> Option<PersistentConfigParameters> {
        let type_id = TypeId::of::<T>();
        self.map
            .write()
            .expect("Unable to lock, for getting config.")
            .get(&type_id)
            .cloned()
    }
}
