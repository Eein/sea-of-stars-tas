pub mod title;
pub mod main;
use crate::memory::MemoryManagers;

pub trait GuiHelper {
    fn name(&self) -> String;
    fn draw(&self, managers: &MemoryManagers, ui: &mut egui::Ui, tab: &mut String);
}
