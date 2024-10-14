use super::GuiHelper;
use crate::{game_manager::GameManager, state::GameState};

pub const NAME: &str = "Inventory Helper";

#[derive(Debug)]
pub struct InventoryHelper;

impl InventoryHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for InventoryHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let currency_manager = &game_state.memory_managers.currency_manager.data;

        ui.label(format!("Money: {}", currency_manager.money));
        ui.separator();
        ui.label("Inventory: NOT YET IMPLEMENTED");
    }
}
