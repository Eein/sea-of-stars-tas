use super::GuiHelper;
use crate::{game_manager::GameManager, seq::movement::Move, state::GameState};

pub const NAME: &str = "Route Helper v1";

#[derive(Debug)]
pub struct RouteHelperV1 {
    coords: Vec<Move>,
}

impl RouteHelperV1 {
    pub fn create() -> Box<Self> {
        Box::new(Self { coords: Vec::new() })
    }
}

impl GuiHelper for RouteHelperV1 {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let ppmd = &game_state.memory_managers.player_party_manager.data;

        ui.label(format!("Coords: {}", self.coords.len()));
        if ui.button("Clear").clicked() {
            self.coords.clear();
        }
        ui.separator();

        if let Some(last) = self.coords.last() {
            ui.label(format!("Last: {}", last));
            if ui.button("Undo").clicked() {
                self.coords.pop();
            }
            ui.separator();
        }

        let world_pos = ppmd.position;
        let pos = ppmd.gameobject_position;

        ui.horizontal(|ui| {
            if ui.button("To").clicked() {
                self.coords
                    .push(Move::To(pos.get_x(), pos.get_y(), pos.get_z()));
            }
            if ui.button("ToWorld").clicked() {
                self.coords.push(Move::ToWorld(
                    world_pos.get_x(),
                    world_pos.get_y(),
                    world_pos.get_z(),
                ));
            }
            if ui.button("Climb").clicked() {
                self.coords
                    .push(Move::Climb(pos.get_x(), pos.get_y(), pos.get_z()));
            }
            if ui.button("Interact").clicked() {
                self.coords
                    .push(Move::Interact(pos.get_x(), pos.get_y(), pos.get_z()));
            }
            if ui.button("WaitFor").clicked() {
                self.coords.push(Move::WaitFor(0.0));
            }
            if ui.button("HoldDir").clicked() {
                self.coords.push(Move::HoldDir(
                    [0.0, 0.0],
                    [pos.get_x(), pos.get_y(), pos.get_z()],
                ));
            }
            if ui.button("HoldDirWorld").clicked() {
                self.coords.push(Move::HoldDirWorld(
                    [0.0, 0.0],
                    [world_pos.get_x(), world_pos.get_y(), world_pos.get_z()],
                ));
            }
            if ui.button("Confirm").clicked() {
                self.coords.push(Move::Confirm);
            }
            if ui.button("Log").clicked() {
                self.coords.push(Move::Log("TEMP"));
            }
            if ui.button("ChangeTime").clicked() {
                self.coords.push(Move::ChangeTime(0.0));
            }
        });

        ui.separator();
        if ui.button("Copy to clipboard").clicked() {
            let mut text = String::new();
            text += "SeqMove::create(
    \"NAME\",
    vec![\n";
            for coord in &self.coords {
                text += &format!("        {},\n", coord);
            }
            text += "    ],
),";
            ui.output_mut(|o| o.copied_text = text);
        }
    }
}
