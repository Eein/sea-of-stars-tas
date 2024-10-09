use log::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub konami_code: bool,
}

#[derive(Deserialize, Debug)]
pub enum ConfigError {
    FileOpen,
}

pub fn load_config(filename: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    match toml::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(err) => {
            error!("Issues parsing config.toml file:");
            error!("{}", err.message());
            Err(Box::new(err))
        }
    }
}
