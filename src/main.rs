mod control;
mod gui;
mod memory;
mod seq;
mod state;

use crate::gui::Gui;

fn main() {
    colog::init();
    Gui::run();
}
