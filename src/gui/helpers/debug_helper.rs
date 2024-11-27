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
pub struct MovementGui {
    coord_fsm: CoordFsm,
    last_game_obj: Vector3<f32>,
    last_world: Vector3<f32>,
    last_boat: Vector3<f32>,
}

impl MovementGui {
    pub fn new() -> Self {
        Self {
            coord_fsm: CoordFsm::GameObj,
            last_game_obj: Vector3::default(),
            last_world: Vector3::default(),
            last_boat: Vector3::default(),
        }
    }

    pub fn draw(&mut self, game_state: &GameState, ui: &mut egui::Ui) {
        let sppmd = &game_state.memory_managers.single_player_plus_manager.data;
        let bmd = &game_state.memory_managers.boat_manager.data;
        if sppmd.players.items.is_empty() {
            return;
        }

        let gameobject_position = sppmd.players.items[0]
            .gameobject_position
            .unwrap_or(Vector3::new(0.0, 0.0, 0.0));
        let position = sppmd.players.items[0]
            .position
            .unwrap_or(Vector3::new(0.0, 0.0, 0.0));

        const EPSILON: f64 = 0.01;

        if !self
            .last_game_obj
            .fuzzy_equal(&gameobject_position, EPSILON)
        {
            self.coord_fsm = CoordFsm::GameObj;
        } else if !self.last_world.fuzzy_equal(&position, EPSILON) {
            self.coord_fsm = CoordFsm::World;
        } else if !self.last_boat.fuzzy_equal(&bmd.position, EPSILON) {
            self.coord_fsm = CoordFsm::Boat;
        }

        ui.label(format!("Coord type: {:?}", self.coord_fsm));
        match self.coord_fsm {
            CoordFsm::GameObj => Self::draw_coord(ui, &gameobject_position),
            CoordFsm::World => Self::draw_coord(ui, &position),
            CoordFsm::Boat => {
                Self::draw_coord(ui, &bmd.position);
                ui.label(format!("Rot (yaw): {:?}", &bmd.rotation.to_yaw()));
                ui.label(format!("speed: {:.3}/{:.3}", bmd.speed, bmd.max_speed));
            }
        }

        self.last_game_obj = gameobject_position;
        self.last_world = position;
        self.last_boat = bmd.position;
    }

    pub fn draw_coord(ui: &mut egui::Ui, pos: &Vector3<f32>) {
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

#[derive(Debug)]
pub struct DebugHelper {
    m_gui: MovementGui,
}

impl DebugHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self {
            m_gui: MovementGui::new(),
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
        let sppmd = &game_state.memory_managers.single_player_plus_manager.data;

        ui.label(format!("Leader: {:?}", ppmd.leader_character));
        ui.label(format!("Movement State: {:?}", ppmd.movement_state));

        self.m_gui.draw(game_state, ui);

        ui.separator();
        ui.label("SPP Manager");
        ui.label("Interacting Characters");
        let interacting_characters = sppmd.interacting_characters.clone();
        if interacting_characters.is_empty() {
            ui.label("None");
        } else {
            for player in sppmd.interacting_characters.clone() {
                ui.label(format!("{:?}", player));
            }
        }

        ui.separator();
        for (idx, player) in sppmd.players.items.iter().enumerate() {
            ui.label(format!("Name: {}", player.name));
            ui.label(format!("Character: {:?}", player.character));
            ui.label(format!("First Player: {:?}", player.first_player));
            ui.label(format!("Index: {:?}", player.index));
            ui.label(format!("Can Join/Leave: {:?}", player.can_join_leave));
            ui.label(format!("Playing: {:?}", player.playing));
            ui.label(format!("State: {}", player.state));
            ui.label(format!(
                "Bubble Fill Amount: {:.3}",
                player.bubble_fill_amount
            ));
            ui.label(format!("In Bubble: {:?}", player.in_bubble));
            egui::CollapsingHeader::new(format!("High Five: {:?}", idx))
                .default_open(false)
                .show(ui, |ui| {
                    ui.label(format!("Has Boost: {:?}", player.has_boost));
                    ui.label(format!("Doing High Five: {:?}", player.is_doing_high_five));
                    ui.label(format!(
                        "Can Replenish Boost: {:?}",
                        player.can_replenish_boost
                    ));
                });
            ui.label("Controller Pos:");
            MovementGui::draw_coord(ui, &player.position.unwrap_or(Vector3::new(0.0, 0.0, 0.0)));
            ui.label("Gameobject Pos:");
            MovementGui::draw_coord(
                ui,
                &player
                    .gameobject_position
                    .unwrap_or(Vector3::new(0.0, 0.0, 0.0)),
            );
            ui.label("");
        }

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
