use super::GuiHelper;
use crate::memory::MemoryManagers;

#[derive(Debug)]
pub struct NavHelper {
    name: String,
}
impl Default for NavHelper {
    fn default() -> NavHelper {
        Self {
            name: "Nav Helper".to_string(),
        }
    }
}

impl GuiHelper for NavHelper {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn draw(&self, managers: &MemoryManagers, ui: &mut egui::Ui, _tab: &mut String) {
        let position = managers.player_party_manager.data.position;
        let pos_x = format!("{:.5}", position.get_x());
        let pos_y = format!("{:.5}", position.get_y());
        let pos_z = format!("{:.5}", position.get_z());
        ui.label(format!("X {}", pos_x));
        ui.label(format!("Y {}", pos_y));
        ui.label(format!("Z {}", pos_z));
    }
}
