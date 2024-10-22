use super::GuiHelper;
use crate::{game_manager::GameManager, state::GameState};

pub const NAME: &str = "Level Up";

#[derive(Debug)]
pub struct LevelUpHelper;

impl LevelUpHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for LevelUpHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let level_up_manager = &game_state.memory_managers.level_up_manager.data;

        ui.label(format!("Level Up Active: {}", level_up_manager.active));
        ui.label(format!(
            "Current Character: {:?}",
            level_up_manager.current_character
        ));

        ui.label(format!("Active Index: {}", level_up_manager.upgrade_index));

        for upgrade in level_up_manager.current_upgrades.items.iter() {
            let selected = if upgrade.selected { "<<<" } else { "" };

            ui.label(format!("{:?} {}", upgrade.upgrade, selected));
        }
    }
}
