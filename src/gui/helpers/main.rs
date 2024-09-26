use super::GuiHelper;
use crate::memory::MemoryManagers;

#[derive(Debug)]
pub struct MainHelper {
    name: String
}
impl Default for MainHelper {
    fn default() -> MainHelper {
        Self {
            name: "Main".to_string()
        }
    }

}


impl GuiHelper for MainHelper {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn draw(&self, _ctx: &MemoryManagers, ui: &mut egui::Ui, _tab: &mut String) {
        ui.label("Drawing inside".to_string());
    }
}
