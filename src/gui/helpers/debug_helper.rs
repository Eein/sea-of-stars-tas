use super::GuiHelper;
use crate::{game_manager::GameManager, state::GameState};

pub const NAME: &str = "Debug Menu";

#[derive(Debug)]
pub struct DebugHelper;

impl DebugHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for DebugHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut GameManager,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let time_of_day_manager = &game_state.memory_managers.time_of_day_manager.data;
        let level_manager = &game_state.memory_managers.level_manager.data;
        let new_dialog_manager = &game_state.memory_managers.new_dialog_manager.data;
        let cutscene_manager = &game_state.memory_managers.cutscene_manager.data;
        let combat_manager = &game_state.memory_managers.combat_manager.data;
        let speedrun_manager = &game_state.memory_managers.speedrun_manager.data;

        ui.label("Encounter".to_string());
        ui.label(format!(
            "Encounter Active: {}",
            combat_manager.encounter_active
        ));
        ui.label("Level Info".to_string());
        ui.label(format!("Scene Name: {}", level_manager.scene_name));
        ui.label(format!("Scene GUID: {}", level_manager.scene_guid));
        ui.label(format!("Loading: {}", level_manager.loading));
        ui.label(format!(
            "Time of day: {:.3}",
            time_of_day_manager.current_time
        ));
        ui.separator();
        ui.label(format!(
            "Dialog Open: {}",
            new_dialog_manager.dialog_visible
        ));
        ui.separator();
        ui.label(format!(
            "Is In Cutscene: : {}",
            cutscene_manager.is_in_cutscene
        ));
        ui.label(format!(
            "Is Skipping Cutscene: : {}",
            cutscene_manager.is_skipping_cutscene
        ));
        ui.label(format!(
            "Skip Cutscene Locked: : {}",
            cutscene_manager.skip_cutscene_locked
        ));
        ui.label(format!(
            "Skip Cutscene Transition Screen PTR : 0x{:x}",
            cutscene_manager.skip_cutscene_transition_screen
        ));
        ui.separator();
        ui.label("Speedrun Manager".to_string());

        ui.label(format!(
            "Is Speedrunning: {}",
            speedrun_manager.is_speedrunning
        ));
        ui.label(format!(
            "Speedrun Timer Pause Lock: {}",
            speedrun_manager.speedrun_timer_pause_lock
        ));
        ui.label("".to_string());

        ui.label("# Speedrun Timer".to_string());
        ui.label(format!(
            "Is Started: {}",
            speedrun_manager.speedrun_timer.is_started
        ));
        ui.label(format!(
            "Is Paused: {}",
            speedrun_manager.speedrun_timer.is_paused
        ));
        ui.label(format!(
            "Timer In Second: {}",
            speedrun_manager.speedrun_timer.timer_in_second
        ));
        ui.label(format!(
            "Realtime Delta Time: {}",
            speedrun_manager.speedrun_timer.realtime_delta_time
        ));

        ui.label("".to_string());
        ui.label("# PauseTimer".to_string());
        ui.label(format!(
            "Is Started: {}",
            speedrun_manager.pause_timer.is_started
        ));
        ui.label(format!(
            "Is Paused: {}",
            speedrun_manager.pause_timer.is_paused
        ));
        ui.label(format!(
            "Timer In Second: {}",
            speedrun_manager.pause_timer.timer_in_second
        ));
        ui.label(format!(
            "Realtime Delta Time: {}",
            speedrun_manager.pause_timer.realtime_delta_time
        ));
    }
}
