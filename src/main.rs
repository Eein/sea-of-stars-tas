//! GUI entry point for the Sea of Stars TAS.
//!
//! For the headless, AI/script-friendly runner see `src/bin/tas-cli.rs`.

use log::*;
use sea_of_stars_tas::config::{Config, load_config};
use sea_of_stars_tas::gui::Gui;

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
