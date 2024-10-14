use super::GuiHelper;
use crate::{game_manager::GameManager, state::GameState};

pub const NAME: &str = "Shop Helper";

#[derive(Debug)]
pub struct ShopHelper;

impl ShopHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for ShopHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let shop_manager = &game_state.memory_managers.shop_manager.data;

        ui.label(format!("Shop (Total: {})", shop_manager.items.count));
        for item in shop_manager.items.items.iter() {
            ui.label(&item.guid);
        }
    }
}
