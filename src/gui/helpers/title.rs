use super::GuiHelper;
use crate::memory::MemoryManagers;

#[derive(Debug)]
pub struct TitleHelper {
    name: String
}
impl Default for TitleHelper {
    fn default() -> TitleHelper {
        Self {
            name: "Title Helper".to_string()
        }
    }

}


impl GuiHelper for TitleHelper {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn draw(&self, _ctx: &MemoryManagers, _ui: &mut egui::Ui, _tab: &mut String) {
    }
}
