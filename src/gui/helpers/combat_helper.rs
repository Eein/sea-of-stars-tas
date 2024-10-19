use super::GuiHelper;
use crate::{game_manager::GameManager, state::GameState};

pub const NAME: &str = "Combat";

#[derive(Debug)]
pub struct CombatHelper;

impl CombatHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for CombatHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let combat_manager = &game_state.memory_managers.combat_manager.data;

        ui.label(format!(
            "Encounter Active: {}",
            combat_manager.encounter_active
        ));
        if combat_manager.encounter_active {
            ui.label(format!(
                "Combat Controller: {:?}",
                combat_manager.combat_controller_type
            ));
            ui.label(format!(
                "Live Mana: Small: {} | Big: {}",
                combat_manager.live_mana.small, combat_manager.live_mana.big
            ));
            ui.label(format!(
                "Combo: {}/3000 | Filled: {}/3 | Ultimate Progress: {:.3}%",
                combat_manager.combo_point_progress,
                combat_manager.combo_points,
                combat_manager.ultimate_progress
            ));
        }
    }
}
