use super::GuiHelper;
use crate::memory::MemoryManagers;

pub const NAME: &str = "Title Helper";

#[derive(Debug)]
pub struct TitleHelper {
    name: String,
}
impl Default for TitleHelper {
    fn default() -> TitleHelper {
        Self {
            name: NAME.to_string(),
        }
    }
}

impl GuiHelper for TitleHelper {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn draw(&mut self, managers: &MemoryManagers, ui: &mut egui::Ui, _tab: &mut String) {
        ui.label(format!(
            "Menu Item Selected {:?}",
            managers.title_sequence_manager.data.title_menu.selected
        ));
    }
}
