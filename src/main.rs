mod control;
mod gui;
mod memory;
mod seq;
mod state;
mod config;

use crate::gui::Gui;
use crate::config::load_config;

fn main() {
    colog::init();
    let conf = load_config("config.yaml").ok();
    Gui::run(conf);
}
