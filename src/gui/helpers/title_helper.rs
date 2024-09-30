use super::GuiHelper;
use crate::memory::MemoryManagers;

pub const NAME: &str = "Title Helper";

#[derive(Debug)]
pub struct TitleHelper;

impl TitleHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for TitleHelper {
    fn draw(&mut self, managers: &MemoryManagers, ui: &mut egui::Ui, _tab: &mut String) {
        ui.label(format!(
            "Menu Item Selected: {:?}",
            managers.title_sequence_manager.data.title_menu.selected
        ));
        ui.label(format!(
            "Pressed Start: {:?}",
            managers.title_sequence_manager.data.pressed_start
        ));
    }
}
