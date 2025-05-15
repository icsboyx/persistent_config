use persistent_config::persistent_config_macros::Persistent;
use persistent_config::{PersistentConfig, PersistentConfigBuilder, SaveFormat};
use serde::{Deserialize, Serialize}; // Import procedural macros

#[derive(Serialize, Deserialize, Default, Debug, Persistent)] // Add configuration for Persistent derive
pub struct MyConfig {
    pub field1: String,
    pub field2: i32,
}

fn main() {
    let my_config = MyConfig {
        field1: "Hello From Derive Example".to_string(),
        field2: 42,
    };

    // Create configuration with the persistent config library
    my_config
        .permanent_config(None, None, SaveFormat::default(), true)
        .unwrap();

    // Load the configuration
    println!("{:=^100}", " Saving configuration ");
    my_config.load().unwrap();

    println!("{:=^100}", " Checking configuration ");
    // println!(
    //     "Check if loaded config is equal to default {}",
    //     my_config.is_default().unwrap()
    // );

    println!("{:=^100}", " Configuration content ");
    println!("Content of the : {:#?}", my_config);

    // Save the configuration
    println!("{:=^100}", " Updating configuration ");
    my_config.save().unwrap();
}
