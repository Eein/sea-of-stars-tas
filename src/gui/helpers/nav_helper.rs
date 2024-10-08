use super::GuiHelper;
use crate::state::GameState;
use vec3_rs::Vector3;

pub const NAME: &str = "Nav Helper";

#[derive(Debug)]
pub struct NavHelper {
    pub precision: f32,
    pub run_enabled: bool,
    pub target_coordinates: Vector3<f32>,
}

impl NavHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self {
            precision: 0.200,
            target_coordinates: Vector3::new(0.0, 0.0, 0.0),
            run_enabled: true,
        })
    }
}

impl GuiHelper for NavHelper {
    fn draw(&mut self, game_state: &GameState, ui: &mut egui::Ui, _tab: &mut String) {
        let ppmd = &game_state.memory_managers.player_party_manager.data;
        ui.label(format!("Movement State: {:?}", ppmd.movement_state));
        ui.separator();

        egui::CollapsingHeader::new("Player Coordinates")
            .default_open(true)
            .show(ui, |ui| {
                let position = ppmd.position;
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
                if ui.button("Copy to clipboard NOT IMPLEMENTED").clicked() {
                    let text = "YOU COPIED THIS TEXT FROM THE CLIPBOARD";
                    ui.output_mut(|o| o.copied_text = String::from(text));
                    // nothing yet
                };
            });

        egui::CollapsingHeader::new("Target Coordinates")
            .default_open(true)
            .show(ui, |ui| {
                let position = self.target_coordinates;
                let player_position = ppmd.position;
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

                if ui.button("Copy to clipboard NOT IMPLEMENTED").clicked() {
                    let text = "YOU COPIED THIS TEXT FROM THE CLIPBOARD";
                    ui.output_mut(|o| o.copied_text = String::from(text));
                };
            });
        ui.separator();
        ui.label("NOT IMPLEMENTED");
        ui.add(egui::Slider::new(&mut self.precision, 0.0..=100.0).text("Precision"));
        if ui.button("Navigate to target").clicked() {
            // nothing yet
        };
        ui.checkbox(&mut self.run_enabled, "Run");
        if ui.button("Stop (timed)").clicked() {
            // nothing yet
        };
        ui.separator();

        egui::CollapsingHeader::new("Gameobject Coordinates")
            .default_open(true)
            .show(ui, |ui| {
                let position = ppmd.gameobject_position;
                let pos_x = format!("{:.3}", position.get_x());
                let pos_y = format!("{:.3}", position.get_y());
                let pos_z = format!("{:.3}", position.get_z());
                ui.label(format!("x: {}", pos_x));
                ui.label(format!("y: {}", pos_y));
                ui.label(format!("z: {}", pos_z));

                if ui.button("Copy to clipboard NOT IMPLEMENTED").clicked() {
                    let text = "YOU COPIED THIS TEXT FROM THE CLIPBOARD";
                    ui.output_mut(|o| o.copied_text = String::from(text));
                    // nothing yet
                };
            });
        egui::CollapsingHeader::new("Boat Coordinates NOT IMPLEMENTED")
            .default_open(true)
            .show(ui, |ui| {
                let position = ppmd.position;
                let pos_x = format!("{:.3}", position.get_x());
                let pos_y = format!("{:.3}", position.get_y());
                let pos_z = format!("{:.3}", position.get_z());
                ui.label(format!("x: {}", pos_x));
                ui.label(format!("y: {}", pos_y));
                ui.label(format!("z: {}", pos_z));

                if ui.button("Copy to clipboard").clicked() {
                    let text = "YOU COPIED THIS TEXT FROM THE CLIPBOARD";
                    ui.output_mut(|o| o.copied_text = String::from(text));
                    // nothing yet
                };
                ui.label(format!("Rot (yaw): {}", pos_z));
                ui.label("speed: not/implemented");
            });
    }
}
