use super::*;
use dirs::{config_dir, data_dir};
use serde::{Deserialize, Serialize};
use std::fs;

const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub archive_location: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            archive_location: default_archive_location(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let _ = fs::create_dir(config_location());
        let config_str =
            fs::read_to_string(config_location() + CONFIG_FILE_NAME).with_context(|| {
                format!(
                    "Failed to read file from {}/{}, perhaps you have not set up archie yet?",
                    config_location(),
                    CONFIG_FILE_NAME
                )
            })?;

        toml::from_str(&config_str).with_context(|| {
            format!(
                "Failed to parse toml file from {}/{}",
                config_location(),
                CONFIG_FILE_NAME
            )
        })
    }
}

fn config_location() -> String {
    let mut config_dir = config_dir().unwrap();
    config_dir.push("archie/");
    config_dir.display().to_string()
}

fn default_archive_location() -> String {
    let mut data_dir = data_dir().unwrap();
    data_dir.push("archie/");
    data_dir.display().to_string()
}
