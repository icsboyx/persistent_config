use std::any::{TypeId, type_name};
use std::collections::HashMap;
use std::default;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

pub use anyhow::Result;
use serde::{Deserialize, Serialize};
pub(crate) static PERSISTENT_CONFIGS: LazyLock<RwLock<PersistentConfigDB>> =
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
            SaveFormat::JSON => ".json",
            SaveFormat::TOML => ".toml",
            SaveFormat::YAML => ".yaml",
        }
    }
}

#[derive(Debug, Clone)]

pub struct PersistentConfigParameters {
    config_dir: PathBuf,
    file_name: String,
    save_format: SaveFormat,
    default_on_error: bool,
}

// Implement Default manually
impl Default for PersistentConfigParameters {
    fn default() -> Self {
        PersistentConfigParameters {
            config_dir: PathBuf::new(),         // Default empty PathBuf, will fall to app directory
            file_name: String::new(),           // Default empty String, will use type name + format extension
            save_format: SaveFormat::default(), // Default SaveFormat::TOML
            default_on_error: true, // Default behavior on error, if true lib will panic else Struct::default() will be returned with a running in memory message.
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

    pub fn get_config<T: 'static>(&self) -> Option<&PersistentConfigParameters> {
        let type_id = TypeId::of::<T>();
        self.map.get(&type_id)
    }
}
pub trait PersistentConfigBuilder:
    Sized + Default + PartialEq + Serialize + for<'de> Deserialize<'de> + 'static
{
    fn permanent_config(
        &self,
        config_dir: Option<PathBuf>,
        file_name: Option<String>,
        save_format: SaveFormat,
        default_on_error: bool,
    ) -> Result<()> {
        // Create file path combining the directory and file name,
        // if directory is none use current directory
        // if file name is none use name of entity

        let config_dir = config_dir.unwrap_or_else(|| PathBuf::from("./"));
        let file_name = file_name.unwrap_or_else(|| {
            std::any::type_name::<Self>().split("::").last().unwrap().to_owned() + save_format.ext()
        });

        let config_params = PersistentConfigParameters {
            config_dir,
            file_name,
            save_format,
            default_on_error,
        };
        PERSISTENT_CONFIGS.write().unwrap().add_config::<Self>(config_params);
        Ok(())
    }
}

pub trait PersistentConfig: PersistentConfigBuilder {
    fn save(&self) -> Result<()> {
        let ret_val = { PERSISTENT_CONFIGS.read().unwrap().get_config::<Self>().cloned() }; // Cloning to unlock 
        match ret_val {
            Some(params) => {
                println!("Saving config to {:?}", params.config_dir);
                println!("File name: {:?}", params.file_name);
                println!("Save format: {:?}", params.save_format);
                println!("Panic on error: {:?}", params.default_on_error);
            }
            None => {
                return Err(anyhow::anyhow!("No persistent config found for this type"));
            }
        }

        Ok(())
    }

    fn load(&self) -> Result<Self> {
        let ret_val = { PERSISTENT_CONFIGS.read().unwrap().get_config::<Self>().cloned() }; // Cloning to unlock
        match ret_val {
            Some(params) => {
                println!("Loading config from {:?}", params.config_dir);
                println!("File name: {:?}", params.file_name);
                println!("Save format: {:?}", params.save_format);
                println!("Panic on error: {:?}", params.default_on_error);
            }
            None => {
                return Err(anyhow::anyhow!("No persistent config found for this type"));
            }
        }
        println!("Implement load method");
        Ok(Self::default())
    }

    fn is_default(&self) -> Result<bool> {
        match self.load() {
            Ok(loaded_config) => {
                if Self::default() == loaded_config {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(_) => Ok(false),
        }
    }
}

impl<T: PersistentConfigBuilder> PersistentConfig for T {}

#[cfg(test)]
mod tests {

    #[test]
    fn test_persistent_config() {
        use super::*;
        #[derive(PartialEq, Serialize, Deserialize, Debug, Default)]
        struct TestConfig {
            field1: String,
            field2: i32,
        }
        impl PersistentConfigBuilder for TestConfig {}

        let my_struct = TestConfig::default();

        my_struct
            .permanent_config(None, None, SaveFormat::default(), true)
            .unwrap();

        println!("{:=^100}", " Test Saving ");
        my_struct.save().unwrap();

        println!("{:=^100}", " Test Loading ");
        my_struct.load().unwrap();
    }
}
