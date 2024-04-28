use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    vault_path: String,
}

pub fn get_config() -> Config {
    let mut config_path =  dirs::config_dir().expect("Couldn't resolve config path!");

    config_path.push("dgim");
    config_path.push("dgim.toml");

    let config_string = fs::read_to_string(config_path).expect("Error loading config file");

    toml::from_str(&config_string).expect("Error parsing config file!")
}
