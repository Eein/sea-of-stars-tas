use super::GuiHelper;
use crate::state::GameState;
use seq::prelude::*;

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
        _sequencer: &mut Sequencer<GameState>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let time_of_day_manager = &game_state.memory_managers.time_of_day_manager.data;
        let level_manager = &game_state.memory_managers.level_manager.data;
        let new_dialog_manager = &game_state.memory_managers.new_dialog_manager.data;
        let cutscene_manager = &game_state.memory_managers.cutscene_manager.data;
        let combat_manager = &game_state.memory_managers.combat_manager.data;
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
    }
}
