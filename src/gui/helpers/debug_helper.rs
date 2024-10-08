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
        ui.label("This is where we dump everything".to_string());
        ui.label(format!(
            "Time of day: {:.3}",
            &game_state
                .memory_managers
                .time_of_day_manager
                .data
                .current_time
        ));
    }
}
