use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{Write, read_to_string};
use std::path::PathBuf;

pub use anyhow::{Error, Result};
pub use persistent_config_core::{PERSISTENT_CONFIGS, PersistentConfigParameters, SaveFormat};
use serde::{Deserialize, Serialize};

pub mod prelude {
    pub use persistent_config_core::*;
    #[cfg(feature = "derive")]
    pub use persistent_config_macros::Persistent;

    pub use crate::{PersistentConfig, PersistentConfigBuilder};
}

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
    ///     Some("./config".to_string()),
    ///     Some("settings".to_string()),
    ///     SaveFormat::TOML,
    ///     false,
    /// )?;
    /// ```
    fn config_builder(
        &self,
        config_dir: Option<String>,
        file_name: Option<String>,
        save_format: SaveFormat,
        panic_on_error: bool,
    ) -> Result<()> {
        let config_dir = config_dir.unwrap_or_else(|| "./".into());
        let file_name =
            file_name.unwrap_or_else(|| std::any::type_name::<Self>().split("::").last().unwrap().to_owned());

        let config_params = PersistentConfigParameters {
            config_dir,
            file_name,
            save_format,
            panic_on_error,
        };
        _ = PERSISTENT_CONFIGS.write().unwrap().add_config::<Self>(config_params);
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
            config_dir: "./".into(),
            file_name: std::any::type_name::<Self>().split("::").last().unwrap().into(),
            save_format: SaveFormat::default(),
            panic_on_error,
        };

        PERSISTENT_CONFIGS
            .write()
            .unwrap()
            .add_config::<Self>(config_params.clone());
        Ok(())
    }
}

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
        let params = match PERSISTENT_CONFIGS.read().unwrap().get_config::<Self>() {
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
        let params = match PERSISTENT_CONFIGS.read().unwrap().get_config::<Self>() {
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
                println!("Error loading file: {:?}", e);
                println!("Ephimeral mode seleted, Returning default configuration, Attention values may be lost");
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

fn load_file<T>(params: &PersistentConfigParameters) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    println!("Loading file with parameters: {:?}", params);
    let mut file_path = PathBuf::from(&params.config_dir);
    file_path.set_file_name(&params.file_name);
    file_path.set_extension(&params.save_format.ext());
    println!("Loading file from {:?}", file_path);

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

fn save_file<T>(params: &PersistentConfigParameters, data: T) -> Result<()>
where
    T: Serialize,
{
    let mut file_path = PathBuf::from(&params.config_dir);
    file_path.set_file_name(&params.file_name);
    file_path.set_extension(&params.save_format.ext());
    println!("Saving file to {:?}", file_path);

    // Convert the data to the appropriate format
    let data = match params.save_format {
        SaveFormat::JSON => serde_json::to_string(&data)?,
        SaveFormat::TOML => toml::to_string(&data)?,
        SaveFormat::YAML => serde_yaml::to_string(&data)?,
    };

    //create a config directory if necessary
    if file_path.parent().is_some() && !file_path.parent().unwrap().exists() {
        println!("Creating config directory: {:?}", file_path.parent().unwrap());
        std::fs::create_dir_all(file_path.parent().unwrap())?
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .append(false)
        .create(true)
        .open(file_path)?;

    file.write_all(&data.as_bytes())?;

    Ok(())
}

impl<T: PersistentConfigBuilder> PersistentConfig for T {}
