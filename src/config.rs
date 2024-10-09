use log::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub konami_code: bool,
}

pub fn load_config(filename: &str) -> Result<Config, Box<dyn Error>> {
    match fs::read_to_string(filename) {
        Ok(contents) => match toml::from_str(&contents) {
            Ok(config) => Ok(config),
            Err(err) => {
                error!("Issues parsing config.toml file:");
                error!("{}", err.message());
                Err(Box::new(err))
            }
        },
        Err(err) => {
            warn!("config.toml file could not be found or read.");
            Err(Box::new(err))
        }
    }
}
