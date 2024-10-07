mod gui;
mod memory;
mod state;

use crate::gui::Gui;

fn main() {
    colog::init();
    Gui::run();
}
