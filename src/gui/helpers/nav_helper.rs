use super::GuiHelper;
use crate::state::GameState;
use quaternion_core::{QuaternionOps, Vector3};
use seq::prelude::*;

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
            target_coordinates: [0.0, 0.0, 0.0],
            run_enabled: true,
        })
    }
}

impl GuiHelper for NavHelper {
    fn draw(
        &mut self,
        game_state: &GameState,
        _sequencer: &mut Sequencer<GameState>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let ppmd = &game_state.memory_managers.player_party_manager.data;
        ui.label(format!("Movement State: {:?}", ppmd.movement_state));
        ui.separator();

        egui::CollapsingHeader::new("Player Coordinates")
            .default_open(true)
            .show(ui, |ui| {
                let position = ppmd.position;
                let pos_x = format!("{:.3}", position[0]);
                let pos_y = format!("{:.3}", position[1]);
                let pos_z = format!("{:.3}", position[2]);
                ui.label(format!("x: {}", pos_x));
                ui.label(format!("y: {}", pos_y));
                ui.label(format!("z: {}", pos_z));

                if ui.button("Set as target").clicked() {
                    self.target_coordinates = [position[0], position[1], position[2]]
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
                let position: Vector3<f32> = self.target_coordinates;
                let player_position = ppmd.position;
                let pos_x = format!("{:.3}", position[0]);
                let pos_y = format!("{:.3}", position[1]);
                let pos_z = format!("{:.3}", position[2]);
                ui.label(format!("x: {}", pos_x));
                ui.label(format!("y: {}", pos_y));
                ui.label(format!("z: {}", pos_z));

                let diff = position.sub(player_position);
                let distance_to_target = crate::util::math::magnitude(&diff);
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
                let pos_x = format!("{:.3}", position[0]);
                let pos_y = format!("{:.3}", position[1]);
                let pos_z = format!("{:.3}", position[2]);
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
                let pos_x = format!("{:.3}", position[0]);
                let pos_y = format!("{:.3}", position[1]);
                let pos_z = format!("{:.3}", position[2]);
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
