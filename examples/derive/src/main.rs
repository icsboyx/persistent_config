use persistent_config::prelude::*;
use serde::{Deserialize, Serialize}; // Import procedural macros

#[derive(Serialize, Deserialize, Debug, Persistent)]
#[persistent(
    panic_on_error = "true",
    file_name = "custom_patata",
    config_dir = "/temp",
    save_format = "yaml"
)]
pub struct MyConfig {
    pub field1: String,
    pub field2: i32,
}

// Impl custom default for the example configuration
impl Default for MyConfig {
    fn default() -> Self {
        Self {
            field1: "DEFAULT: Hello From Derive Example".to_string(),
            field2: 69,
        }
    }
}

fn main() {
    println!("{:=^100}", "Running Derive Example");
    let mut my_config = MyConfig::default();

    // Set the configuration
    // Default config:
    //      file_name, is the name of the struct type.
    //      save_format, is toml.
    //      panic_on_error, is true.
    //
    // my_config.default_save_config(false).unwrap();
    // my_config.config_builder(None, None, SaveFormat::TOML, false).unwrap();

    my_config.load().unwrap();

    println!("{:=^100}", "Loaded Values");
    println!("Struct value: {:?}", my_config);

    println!("{:=^100}", "Changing Values");
    let my_config = MyConfig {
        field1: "Hello From Derive Example".to_string(),
        field2: 42,
    };
    println!("{:=^100}", "New Values");
    println!("Struct value: {:?}", my_config);

    println!("{:=^100}", " Updating configuration ");
    my_config.save().unwrap();
}
