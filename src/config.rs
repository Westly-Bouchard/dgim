use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub vault_path: String,
    pub exercises: Vec<String>
}

pub fn get_config() -> Config {
    let mut config_path = dirs::config_dir().expect("Couldn't resolve config path!");

    config_path.push("dgim");
    config_path.push("dgim.toml");

    match fs::read_to_string(&config_path) {
        Ok(str) => toml::from_str(&str).expect("Error parsing config file!"),
        Err(error) => {
            println!("Error opening config file!");
            println!("Config should be located at: {:?}", config_path);
            panic!("{:?}", error);
        }
    }
}
