//! # Persistent Config Example: Simple Usage
//!
//! This example demonstrates how to use the `PersistentConfigBuilder` trait
//! to easily persist and restore application configuration using the
//! `persistent_config` crate.
//!
//! ## What does this example do?
//! - Defines a simple `AppConfig` struct for application settings.
//! - Registers the config for persistence (using TOML format, file named after the struct).
//! - Saves the config to disk.
//! - Loads the config from disk into a new instance.
//!
//! ## How to run
//! ```sh
//! cargo run --example simple
//! ```
//!
//! ## Output
//! You will see the configuration printed before and after loading from disk.
//! The config file will be created in the current directory (e.g., `AppConfig.toml`).
//!
//! ## Key API
//! - `default_save_config`: Registers the config for persistence with default settings.
//! - `save`: Saves the struct to disk.
//! - `load`: Loads the struct from disk.

use persistent_config::prelude::*;
use serde::{Deserialize, Serialize};

/// Application configuration with persistent storage.
///
/// The `Persistent` derive macro automatically implements the necessary traits
/// for saving and loading this struct to disk.
#[derive(Debug, Serialize, Deserialize, Default)]
struct AppConfig {
    /// Username for the application.
    username: String,
    /// Number of times the app has been launched.
    launch_count: u32,
}
/// Trait to enable the config for persistence.
impl PersistentConfigBuilder for AppConfig {}

fn main() -> anyhow::Result<()> {
    // Create a default config and register it for persistence.
    let config = AppConfig {
        username: "alice".to_string(),
        launch_count: 1,
    };

    // Register config with default parameters (TOML, current dir, type name as file).
    config.default_save_config(false)?;

    // Save the configuration to disk.
    config.save()?;
    println!("Configuration saved: {:?}", config);

    // Simulate a new session by resetting the struct.
    let mut loaded_config = AppConfig::default();

    // Register config for loading (must match previous registration).
    loaded_config.default_save_config(false)?;

    // Load the configuration from disk.
    loaded_config.load()?;
    println!("Configuration loaded: {:?}", loaded_config);

    Ok(())
}
