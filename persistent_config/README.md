# persistent_config

**persistent_config** is a Rust crate for simple, type-safe, and ergonomic persistent configuration management. It allows you to easily save and load your application's configuration structs to disk using familiar serialization formats (like TOML, JSON, etc.).

## Features

- Derive macro for automatic persistent config support
- Supports TOML, JSON, and other Serde formats
- Minimal boilerplate: just derive and use
- Type-safe and ergonomic API



## Usage

### Simple Usage with implementation
Add to your `Cargo.toml`:

```toml
[dependencies]
persistent_config = "0.1"
serde = { version = "1.0", features = ["derive"] }
```

## Example

```rust
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
```


### Derive Usage with implementation

You need to enable the `derive` feature in your `Cargo.toml`:

```toml
[dependencies]
persistent_config = { version = "0.1", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
```

Then you can use the `Persistent` derive macro to automatically implement the necessary traits for your configuration structs.


```rust
//! Example: Using the `Persistent` derive macro for configuration persistence.
//!
//! This example demonstrates how to use the `Persistent` derive macro to
//! automatically implement persistent save/load functionality for a config struct.
//!
//! Run with: `cargo run --example simple`

use persistent_config::prelude::*;
use serde::{Deserialize, Serialize};

/// Application configuration with persistent storage.
///
/// The `Persistent` derive macro automatically implements the necessary traits
/// for saving and loading this struct to disk.
#[derive(Debug, Serialize, Deserialize, Default, Persistent)]
struct AppConfig {
    /// Username for the application.
    username: String,
    /// Number of times the app has been launched.
    launch_count: u32,
}

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
```