use log::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

/*
pub const RELIC_NAMES: [&str; 24] = [
    "Amulet of Storytelling",
    "Tourist's Provision",
    "Volitional Spirit",
    "Signifying Glass",
    "Guardian Aura",
    "Sequent Flare",
    "Truestrike Pendant",
    "Tome of Knowledge",
    "Falcon-eyed Parrot",
    "Salient Sails",
    "Gold Tooth",
    "Chef's Knife",
    "Sixth Sense",
    "Adamant Shard",
    "Hidden Pockets",
    "Mithreel Rod",
    "Bearing Reel",
    "Stereofilament Line",
    "Tactician's Mettle",
    "Artful Gambit",
    "Double Edge",
    "Dubious Dare",
    "Solstice Doctorate",
    "Mark of the Speedrunner",
];
*/

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub persist_tab_state: bool,
    pub konami_code: bool,
    pub solstice_diploma: bool,
    /// Shell command used to launch the game when it isn't already running.
    /// Optional; when unset the game must be started manually.
    #[serde(default)]
    pub game_start_command: Option<String>,
    pub relics: HashMap<String, bool>,
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
