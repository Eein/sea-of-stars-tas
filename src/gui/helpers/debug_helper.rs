use super::GuiHelper;
use crate::{game_manager::GameManager, memory::speedrun_manager::SpeedrunTimer, state::GameState};

use vec3_rs::Vector3;

pub const NAME: &str = "Debug Menu";

#[derive(Debug)]
enum CoordFsm {
    GameObj,
    World,
    Boat,
}

#[derive(Debug)]
pub struct DebugHelper {
    coord_fsm: CoordFsm,
    last_game_obj: Vector3<f32>,
    last_world: Vector3<f32>,
    last_boat: Vector3<f32>,
}

impl DebugHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self {
            coord_fsm: CoordFsm::GameObj,
            last_game_obj: Vector3::default(),
            last_world: Vector3::default(),
            last_boat: Vector3::default(),
        })
    }

    fn draw_timer(&self, ui: &mut egui::Ui, timer: &SpeedrunTimer) {
        ui.label(format!("Is Started: {}", timer.is_started));
        ui.label(format!("Is Paused: {}", timer.is_paused));
        ui.label(format!("Timer In Second: {}", timer.timer_in_second));
        ui.label(format!(
            "Realtime Delta Time: {}",
            timer.realtime_delta_time
        ));
    }

    fn draw_coord(&self, ui: &mut egui::Ui, pos: &Vector3<f32>) {
        ui.label(format!(
            "{:.3}, {:.3}, {:.3}",
            pos.get_x(),
            pos.get_y(),
            pos.get_z()
        ));
        if ui.button("Copy to clipboard").clicked() {
            ui.output_mut(|o| {
                o.copied_text =
                    format!("{:.3}, {:.3}, {:.3}", pos.get_x(), pos.get_y(), pos.get_z())
            });
        }
    }
}

impl GuiHelper for DebugHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let todm = &game_state.memory_managers.time_of_day_manager.data;
        let level_manager = &game_state.memory_managers.level_manager.data;
        let new_dialog_manager = &game_state.memory_managers.new_dialog_manager.data;
        let cutscene_manager = &game_state.memory_managers.cutscene_manager.data;
        let speedrun_manager = &game_state.memory_managers.speedrun_manager.data;
        let ppmd = &game_state.memory_managers.player_party_manager.data;
        let bmd = &game_state.memory_managers.boat_manager.data;

        const EPSILON: f64 = 0.01;

        if !self
            .last_game_obj
            .fuzzy_equal(&ppmd.gameobject_position, EPSILON)
        {
            self.coord_fsm = CoordFsm::GameObj;
        } else if !self.last_world.fuzzy_equal(&ppmd.position, EPSILON) {
            self.coord_fsm = CoordFsm::World;
        } else if !self.last_boat.fuzzy_equal(&bmd.position, EPSILON) {
            self.coord_fsm = CoordFsm::Boat;
        }

        ui.label(format!("Leader: {:?}", ppmd.leader_character));
        ui.label(format!("Movement State: {:?}", ppmd.movement_state));
        ui.label(format!("Coord type: {:?}", self.coord_fsm));

        match self.coord_fsm {
            CoordFsm::GameObj => self.draw_coord(ui, &ppmd.gameobject_position),
            CoordFsm::World => self.draw_coord(ui, &ppmd.position),
            CoordFsm::Boat => {
                self.draw_coord(ui, &bmd.position);
                ui.label(format!("Rot (yaw): {:?}", &bmd.rotation.to_yaw()));
                ui.label(format!("speed: {:.3}/{:.3}", bmd.speed, bmd.max_speed));
            }
        }

        self.last_game_obj = ppmd.gameobject_position;
        self.last_world = ppmd.position;
        self.last_boat = bmd.position;

        ui.separator();

        egui::CollapsingHeader::new("Level Info")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Level Info".to_string());
                ui.label(format!("Scene Name: {}", level_manager.scene_name));
                ui.label(format!("Scene GUID: {}", level_manager.scene_guid));
                ui.label(format!("Loading: {}", level_manager.loading));
                ui.label(format!("Time of day: {:.3}", todm.current_time));
            });
        ui.separator();

        egui::CollapsingHeader::new("Cutscenes")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!(
                    "Dialog Open: {}",
                    new_dialog_manager.dialog_visible
                ));
                ui.separator();
                ui.label(format!(
                    "Is In Cutscene: {}",
                    cutscene_manager.is_in_cutscene
                ));
                ui.label(format!(
                    "Is Skipping Cutscene: {}",
                    cutscene_manager.is_skipping_cutscene
                ));
                ui.label(format!(
                    "Skip Cutscene Locked: {}",
                    cutscene_manager.skip_cutscene_locked
                ));
                ui.label(format!(
                    "Skip Cutscene Transition Screen PTR: 0x{:x}",
                    cutscene_manager.skip_cutscene_transition_screen
                ));
            });

        ui.separator();
        egui::CollapsingHeader::new("Speedrun Manager")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!(
                    "Is Speedrunning: {}",
                    speedrun_manager.is_speedrunning
                ));
                ui.label(format!(
                    "Speedrun Timer Pause Lock: {}",
                    speedrun_manager.speedrun_timer_pause_lock
                ));

                egui::CollapsingHeader::new("Speedrun Timer")
                    .default_open(true)
                    .show(ui, |ui| {
                        self.draw_timer(ui, &speedrun_manager.speedrun_timer);
                    });

                egui::CollapsingHeader::new("Pause Timer")
                    .default_open(true)
                    .show(ui, |ui| {
                        self.draw_timer(ui, &speedrun_manager.pause_timer);
                    });
            });
    }
}
