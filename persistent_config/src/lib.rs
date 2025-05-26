#![doc = include_str!("../README.md")]

//! Persistent configuration trait and helpers.
//!
//! This module provides traits and helpers for saving and loading configuration
//! structs to disk using various formats (JSON, TOML, YAML). It builds on the
//! core types from `persistent_config_core` and provides a builder pattern for
//! configuring persistence parameters.

use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{Write, read_to_string};
use std::path::PathBuf;

use anyhow::Result;
use persistent_config_core::{PERSISTENT_CONFIGS, PersistentConfigParameters, SaveFormat};
use serde::{Deserialize, Serialize};

/// Prelude for convenient imports.
///
/// This module re-exports the most commonly used items for persistent config.
pub mod prelude {
    pub use persistent_config_core::*;
    #[cfg(feature = "derive")]
    pub use persistent_config_macros::Persistent;

    pub use crate::{PersistentConfig, PersistentConfigBuilder};
}

/// Trait for building persistent configuration parameters for a type.
///
/// This trait provides methods for registering how a type should be saved and loaded.
pub trait PersistentConfigBuilder: Sized + Default + Serialize + for<'de> Deserialize<'de> + 'static + Debug {
    /// Configures persistent storage parameters for a type.
    ///
    /// This function sets up how and where configuration data will be saved and loaded.
    ///
    /// # Parameters
    ///
    /// * `config_dir` - Optional directory path where the config file will be stored. Defaults to `./`.
    /// * `file_name` - Optional name for the config file (without extension). Defaults to the type name.
    /// * `save_format` - Format used for serialization (JSON, TOML, or YAML).
    /// * `panic_on_error` - If true, panics on load/save errors. If false, falls back to defaults.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the configuration was registered successfully
    /// * `Err` if there was a problem registering the configuration
    ///
    /// # Example
    ///
    /// ```
    /// let my_config = MyConfig::default();
    /// my_config.config_builder(
    ///     Some("./some_dir".to_string()),
    ///     Some("some_name".to_string()),
    ///     SaveFormat::TOML, /// Serialization format
    ///     false,            /// Panic on error, false means it will use default values on error
    /// )?;
    /// ```
    fn config_builder(
        &self,
        config_dir: Option<impl AsRef<str>>,
        file_name: Option<impl AsRef<str>>,
        save_format: SaveFormat,
        panic_on_error: bool,
    ) -> Result<()> {
        let config_dir = config_dir.map_or_else(|| "./.config".to_string(), |dir| dir.as_ref().to_string());
        let file_name = file_name.map_or_else(
            || std::any::type_name::<Self>().split("::").last().unwrap().to_owned(),
            |name| name.as_ref().to_string(),
        );

        let config_params = PersistentConfigParameters {
            config_dir,
            file_name,
            save_format,
            panic_on_error,
        };
        _ = PERSISTENT_CONFIGS.add_config::<Self>(config_params);
        Ok(())
    }

    /// Configures persistent storage with default parameters.
    ///
    /// This function provides a simplified way to set up configuration persistence with default values.
    /// It uses the current directory for storage and the type name as the file name, with TOML as the format.
    ///
    /// # Parameters
    ///
    /// * `panic_on_error` - If true, panics on load/save errors. If false, falls back to default values.
    /// In this case, data may be lost if the program exits without saving successfully.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the configuration was registered successfully
    /// * `Err` if there was a problem registering the configuration
    ///
    /// # Example
    ///
    /// ```
    /// let my_config = MyConfig::default();
    /// my_config.default_save_config(false)?;
    /// ```
    fn default_save_config(&self, panic_on_error: bool) -> Result<()> {
        let config_params = PersistentConfigParameters {
            panic_on_error,
            file_name: std::any::type_name::<Self>().split("::").last().unwrap().to_owned(),
            config_dir: "./.config".to_string(),
            save_format: SaveFormat::default(),
        };

        PERSISTENT_CONFIGS.add_config::<Self>(config_params.clone());
        Ok(())
    }
}

/// Trait for saving and loading persistent configuration.
///
/// This trait provides methods for saving to and loading from disk.
pub trait PersistentConfig: PersistentConfigBuilder {
    /// Saves the current configuration to persistent storage.
    ///
    /// This function writes the configuration data to a file according to the parameters
    /// previously registered with `config_builder` or `default_save_config`.
    ///
    /// # Behavior
    ///
    /// - If no configuration parameters have been registered, returns an error
    /// - If saving succeeds, prints a success message
    /// - If saving fails and `panic_on_error` is true, logs the error but returns Ok
    /// - If saving fails and `panic_on_error` is false, returns an error
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the configuration was saved successfully or if `panic_on_error` is true
    /// * `Err` if the configuration could not be saved and `panic_on_error` is false
    ///
    /// # Example
    ///
    /// ```
    /// let my_config = MyConfig::default();
    /// my_config.default_save_config(false)?;
    /// my_config.save()?;
    /// ```
    fn save(&self) -> Result<()> {
        let params = match PERSISTENT_CONFIGS.get_config::<Self>() {
            Some(params) => params,
            None => {
                return Err(anyhow::anyhow!("No persistent config found for this type"));
            }
        };

        match save_file(&params, self) {
            Ok(_) => {
                println!("File saved successfully");
            }

            Err(e) if params.panic_on_error => {
                println!("Error saving file: {:?}", e);
                println!("Using default configuration");
            }

            Err(e) => {
                println!("Error saving file: {:?}", e);
                return Err(anyhow::anyhow!("Failed to save file"));
            }
        }
        Ok(())
    }

    /// Loads configuration from persistent storage into the current instance.
    ///
    /// This function reads configuration data from a file according to parameters
    /// previously registered with `config_builder` or `default_save_config`.
    ///
    /// # Behavior
    ///
    /// - If no configuration parameters have been registered, returns an error
    /// - If loading succeeds, replaces the current instance with the loaded data
    /// - If loading fails and `panic_on_error` is false, logs the error and uses default values
    /// - If loading fails and `panic_on_error` is true, returns an error
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the configuration was loaded successfully or if using defaults due to error with `panic_on_error` false
    /// * `Err` if the configuration could not be loaded and `panic_on_error` is true
    ///
    /// # Example
    ///
    /// ```
    /// let mut my_config = MyConfig::default();
    /// my_config.default_save_config(false)?;
    /// my_config.load()?;
    /// ```
    fn load(&mut self) -> Result<()>
    where
        Self: for<'de> Deserialize<'de>,
    {
        let params = match PERSISTENT_CONFIGS.get_config::<Self>() {
            Some(params) => params,
            None => {
                return Err(anyhow::anyhow!("No persistent config found for this type"));
            }
        };

        match load_file(&params) {
            Ok(content) => {
                *self = content;
                return Ok(());
            }
            Err(e) if !params.panic_on_error => {
                eprintln!("Error loading file: {:?}", e);
                eprintln!("Ephemeral mode selected, Returning default configuration, Attention values may be lost");
                *self = Self::default();
                return Ok(());
            }
            Err(e) => {
                println!("Error loading file: {:?}", e);
                return Err(anyhow::anyhow!("Failed to load file"));
            }
        }
    }
}

/// Loads configuration data from a file according to the given parameters.
///
/// Returns the deserialized configuration struct.
fn load_file<T>(params: &PersistentConfigParameters) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let mut file_path = PathBuf::new();
    file_path.push(&params.config_dir);
    file_path.push(&params.file_name);
    file_path.set_extension(&params.save_format.ext());

    let file = File::open(&file_path)?;
    let ret_val = read_to_string(file)?;

    match params.save_format {
        SaveFormat::JSON => {
            let config: T = serde_json::from_str(&ret_val)?;
            Ok(config)
        }
        SaveFormat::TOML => {
            let config: T = toml::de::from_str(&ret_val)?;
            Ok(config)
        }
        SaveFormat::YAML => {
            let config: T = serde_yaml::from_str(&ret_val)?;
            Ok(config)
        }
    }
}

/// Saves configuration data to a file according to the given parameters.
///
/// Serializes the struct and writes it to disk.
fn save_file<T>(params: &PersistentConfigParameters, data: T) -> Result<()>
where
    T: Serialize,
{
    let mut file_path = PathBuf::new();
    file_path.push(&params.config_dir);
    file_path.push(&params.file_name);
    file_path.set_extension(&params.save_format.ext());

    // Convert the data to the appropriate format
    let data = match params.save_format {
        SaveFormat::JSON => serde_json::to_string(&data)?,
        SaveFormat::TOML => toml::to_string(&data)?,
        SaveFormat::YAML => serde_yaml::to_string(&data)?,
    };

    // Create a config directory if necessary
    if file_path.parent().is_some() && !file_path.parent().unwrap().exists() {
        // println!("Creating config directory: {:?}", file_path.parent().unwrap());
        std::fs::create_dir_all(file_path.parent().unwrap())?
    }

    // Open the file for writing, truncating it if it exists
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .append(false)
        .create(true)
        .open(file_path)?;

    file.write_all(&data.as_bytes())?;

    Ok(())
}

// This trait is implemented for any type that implements PersistentConfigBuilder.
impl<T: PersistentConfigBuilder> PersistentConfig for T {}
