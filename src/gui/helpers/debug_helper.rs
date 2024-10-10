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
        game_state: &GameState,
        _sequencer: &mut Sequencer<GameState>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let time_of_day_manager = &game_state.memory_managers.time_of_day_manager.data;
        let level_manager = &game_state.memory_managers.level_manager.data;

        ui.label("Level Info".to_string());
        ui.label(format!("Scene Name: {}", level_manager.scene_name));
        ui.label(format!("Scene GUID: {}", level_manager.scene_guid));
        ui.label(format!("Loading: {}", level_manager.loading));
        ui.label(format!(
            "Time of day: {:.3}",
            time_of_day_manager.current_time
        ));
        ui.separator();
    }
}
