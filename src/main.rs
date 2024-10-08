mod config;
mod control;
mod gui;
mod memory;
mod seq;
mod state;
mod util;

use crate::config::load_config;
use crate::gui::Gui;

fn main() {
    colog::init();
    let conf = load_config("config.yaml").ok();
    Gui::run(conf);
}
