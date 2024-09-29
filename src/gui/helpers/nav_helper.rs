use super::GuiHelper;
use crate::memory::MemoryManagers;
use vec3_rs::Vector3;

#[derive(Debug)]
pub struct NavHelper {
    pub name: String,
    pub precision: f32,
    pub run_enabled: bool,
    pub target_coordinates: Vector3<f32>,
}
impl Default for NavHelper {
    fn default() -> NavHelper {
        Self {
            name: "Nav Helper".to_string(),
            precision: 0.200,
            target_coordinates: Vector3::new(0.0, 0.0, 0.0),
            run_enabled: true,
        }
    }
}

impl GuiHelper for NavHelper {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn draw(&mut self, managers: &MemoryManagers, ui: &mut egui::Ui, _tab: &mut String) {
        ui.label(format!("Movement State: {}", "Not Implemented!"));
        ui.separator();

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
                    self.target_coordinates =
                        Vector3::new(position.get_x(), position.get_y(), position.get_z())
                };
                if ui.button("Copy to clipboard").clicked() {
                    let text = "YOU COPIED THIS TEXT FROM THE CLIPBOARD";
                    ui.output_mut(|o| o.copied_text = String::from(text));
                    // nothing yet
                };
            });

        egui::CollapsingHeader::new("Target Coordinates NOT IMPLEMENTED")
            .default_open(true)
            .show(ui, |ui| {
                let position = self.target_coordinates;
                let player_position = managers.player_party_manager.data.position;
                let pos_x = format!("{:.3}", position.get_x());
                let pos_y = format!("{:.3}", position.get_y());
                let pos_z = format!("{:.3}", position.get_z());
                ui.label(format!("x: {}", pos_x));
                ui.label(format!("y: {}", pos_y));
                ui.label(format!("z: {}", pos_z));

                let diff = position - player_position;
                let distance_to_target = Vector3::magnitude(&diff);
                let distance_to_target_string =
                    format!("Distance to target {:.3}", distance_to_target);
                ui.label(distance_to_target_string);

                if ui.button("Copy to clipboard").clicked() {
                    let text = "YOU COPIED THIS TEXT FROM THE CLIPBOARD";
                    ui.output_mut(|o| o.copied_text = String::from(text));
                };
            });
        ui.separator();
        ui.add(egui::Slider::new(&mut self.precision, 0.0..=100.0).text("Precision"));
        if ui.button("Navigate to target").clicked() {
            // nothing yet
        };
        ui.checkbox(&mut self.run_enabled, "Run");
        if ui.button("Stop (timed)").clicked() {
            // nothing yet
        };
        ui.separator();

        egui::CollapsingHeader::new("Gameobject Coordinates NOT IMPLEMENTED")
            .default_open(true)
            .show(ui, |ui| {
                let position = managers.player_party_manager.data.position;
                let pos_x = format!("{:.3}", position.get_x());
                let pos_y = format!("{:.3}", position.get_y());
                let pos_z = format!("{:.3}", position.get_z());
                ui.label(format!("x: {}", pos_x));
                ui.label(format!("y: {}", pos_y));
                ui.label(format!("z: {}", pos_z));

                if ui.button("Copy to clipboard").clicked() {
                    // nothing yet
                };
            });
        egui::CollapsingHeader::new("Boat Coordinates NOT IMPLEMENTED")
            .default_open(true)
            .show(ui, |ui| {
                let position = managers.player_party_manager.data.position;
                let pos_x = format!("{:.3}", position.get_x());
                let pos_y = format!("{:.3}", position.get_y());
                let pos_z = format!("{:.3}", position.get_z());
                ui.label(format!("x: {}", pos_x));
                ui.label(format!("y: {}", pos_y));
                ui.label(format!("z: {}", pos_z));

                if ui.button("Copy to clipboard").clicked() {
                    // nothing yet
                };
                ui.label(format!("Rot (yaw): {}", pos_z));
                ui.label("speed: not/implemented");
            });
    }
}
