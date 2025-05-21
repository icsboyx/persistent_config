use persistent_config::{PersistentConfig, PersistentConfigBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct MyConfig {
    field1: String,
    field2: i32,
}

impl PersistentConfigBuilder for MyConfig {}

fn main() {
    let mut my_config = MyConfig {
        field1: "Hello From Simple impl Example".to_string(),
        field2: 42,
    };

    // Create configuration with the persistent config library
    my_config.default_save_config(false).unwrap();

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
