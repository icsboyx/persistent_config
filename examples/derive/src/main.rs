use persistent_config::prelude::*;
use serde::{Deserialize, Serialize}; // Import procedural macros

#[derive(Serialize, Deserialize, Debug, Persistent)] // Add configuration for Persistent derive
#[persistent(panic_on_error = false, non_serve = true)]
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

    my_config.load().unwrap();

    println!("{:=^100}", "Loaded Values");
    println!("Struct value: {:?}", my_config);

    let my_config = MyConfig {
        field1: "Hello From Derive Example".to_string(),
        field2: 42,
    };

    // println!("{:=^100}", " Checking configuration ");
    // // println!(
    // //     "Check if loaded config is equal to default {}",
    // //     my_config.is_default().unwrap()
    // // );

    // println!("{:=^100}", " Configuration content ");
    // println!("Content of the : {:#?}", my_config);

    // Save the configuration
    println!("{:=^100}", " Updating configuration ");
    my_config.save().unwrap();
}
