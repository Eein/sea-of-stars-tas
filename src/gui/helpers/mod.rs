pub mod main_helper;
pub mod nav_helper;
pub mod title_helper;

pub use self::{main_helper::*, nav_helper::*, title_helper::*};

use crate::memory::MemoryManagers;

pub trait GuiHelper {
    fn name(&self) -> String;
    fn draw(&self, managers: &MemoryManagers, ui: &mut egui::Ui, tab: &mut String);
}
