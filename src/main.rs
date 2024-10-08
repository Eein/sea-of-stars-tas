mod config;
mod control;
mod gui;
mod memory;
mod seq;
mod state;

use crate::config::load_config;
use crate::gui::Gui;

fn main() {
    colog::init();
    let conf = load_config("config.yaml").ok();
    Gui::run(conf);
}
