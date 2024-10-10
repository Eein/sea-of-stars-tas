mod config;
mod control;
mod gui;
mod memory;
mod seq;
mod state;
mod util;
mod controllers;

use crate::config::{load_config, Config};
use crate::gui::Gui;
use log::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    colog::init();

    // Loads the config.toml if it exists or loads defaults
    let conf = match load_config("./config.toml") {
        Ok(conf) => conf,
        Err(_err) => {
            warn!("No config.toml loaded, using defaults.");
            Config::default()
        }
    };

    Gui::run(conf);
    Ok(())
}
