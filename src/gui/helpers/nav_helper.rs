use super::GuiHelper;
use crate::memory::MemoryManagers;

#[derive(Debug)]
pub struct NavHelper {
    pub name: String,
    pub precision: f32,
}
impl Default for NavHelper {
    fn default() -> NavHelper {
        Self {
            name: "Nav Helper".to_string(),
            precision: 0.200,
        }
    }
}

impl GuiHelper for NavHelper {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn draw(&mut self, managers: &MemoryManagers, ui: &mut egui::Ui, _tab: &mut String) {
        ui.label(format!("Movement State: {}", "Not Implemented!"));

        egui::CollapsingHeader::new("Player Coordinates NOT IMPLEMENTED")
            .default_open(true)
            .show(ui, |ui| {
                let position = managers.player_party_manager.data.position;
                let pos_x = format!("{:.3}", position.get_x());
                let pos_y = format!("{:.3}", position.get_y());
                let pos_z = format!("{:.3}", position.get_z());
                ui.label(format!("x: {}", pos_x));
                ui.label(format!("y: {}", pos_y));
                ui.label(format!("z: {}", pos_z));

                if ui.button("Set as target").clicked() {
                    // nothing yet
                };
                if ui.button("Copy to clipboard").clicked() {
                    // nothing yet
                };
            });

        egui::CollapsingHeader::new("Target Coordinates NOT IMPLEMENTED")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::TextEdit::singleline(&mut "0.00"));
                ui.add(egui::TextEdit::singleline(&mut "0.00"));
                ui.add(egui::TextEdit::singleline(&mut "0.00"));

                ui.label("Distance to target: 0.000");

                if ui.button("Copy to clipboard").clicked() {
                    // nothing yet
                };
            });

        ui.add(egui::Slider::new(&mut self.precision, 0.0..=100.0).text("My value"));
    }
}
