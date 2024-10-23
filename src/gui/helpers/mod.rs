mod debug_helper;
mod inventory_helper;
mod main_helper;
mod route_helper_v1;
mod shop_helper;

use std::collections::HashMap;

use self::{
    debug_helper::*, inventory_helper::*, main_helper::*, route_helper_v1::*, shop_helper::*,
};

use crate::{game_manager::GameManager, state::GameState};

pub struct GuiHelpers {
    pub helpers: HashMap<String, Box<dyn GuiHelper>>,
}

impl Default for GuiHelpers {
    fn default() -> Self {
        let mut helpers: HashMap<String, Box<dyn GuiHelper>> = HashMap::new();
        helpers.insert(main_helper::NAME.to_owned(), MainHelper::create());
        helpers.insert(debug_helper::NAME.to_owned(), DebugHelper::create());
        helpers.insert(shop_helper::NAME.to_owned(), ShopHelper::create());
        helpers.insert(inventory_helper::NAME.to_owned(), InventoryHelper::create());
        helpers.insert(route_helper_v1::NAME.to_owned(), RouteHelperV1::create());
        Self { helpers }
    }
}

impl GuiHelpers {
    // Build a vector of all the hashmap keys
    pub fn tree_names(&self) -> Vec<String> {
        self.helpers.keys().cloned().collect()
    }

    pub fn draw(
        &mut self,
        game_state: &mut GameState,
        game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        tab: &mut String,
    ) {
        match self.helpers.get_mut(tab.as_str()) {
            Some(helper) => helper.draw(game_state, game_manager, ui, tab),
            None => {
                let label = format!("Tab: {} has not been initialized. Check gui/mod.rs and state.rs to initialize this tab.", tab.as_str());
                ui.label(label);
            }
        }
    }
}

pub trait GuiHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        tab: &mut String,
    );
}
