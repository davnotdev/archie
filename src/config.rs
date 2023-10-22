use super::*;
use homedir::get_my_home;
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
            archive_location: default_archive_location().unwrap(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let _ = fs::create_dir(config_location()?);
        let config_str = fs::read_to_string(config_location()? + CONFIG_FILE_NAME).with_context(|| {
            format!(
                "Failed to read file from {}/{}, perhaps you have not set up archie yet?",
                config_location().unwrap(),
                CONFIG_FILE_NAME
            )
        })?;

        toml::from_str(&config_str).with_context(|| {
            format!(
                "Failed to parse toml file from {}/{}",
                config_location().unwrap(),
                CONFIG_FILE_NAME
            )
        })
    }
}

fn config_location() -> Result<String> {
    Ok(get_my_home()?.unwrap().to_str().unwrap().to_owned() + "/.config/archie/")
}

fn default_archive_location() -> Result<String> {
    Ok(get_my_home()?.unwrap().to_str().unwrap().to_owned() + "/.archie/")
}
