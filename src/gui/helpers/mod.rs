mod combat_helper;
mod debug_helper;
mod inventory_helper;
mod level_up_helper;
mod main_helper;
mod nav_helper;
mod route_helper_v1;
mod shop_helper;
mod title_helper;

use std::collections::HashMap;

use self::{
    combat_helper::*, debug_helper::*, inventory_helper::*, level_up_helper::*, main_helper::*,
    nav_helper::*, route_helper_v1::*, shop_helper::*, title_helper::*,
};

use crate::{game_manager::GameManager, state::GameState};

pub struct GuiHelpers {
    pub helpers: HashMap<String, Box<dyn GuiHelper>>,
}

impl Default for GuiHelpers {
    fn default() -> Self {
        let mut helpers: HashMap<String, Box<dyn GuiHelper>> = HashMap::new();
        helpers.insert(nav_helper::NAME.to_owned(), NavHelper::create());
        helpers.insert(main_helper::NAME.to_owned(), MainHelper::create());
        helpers.insert(title_helper::NAME.to_owned(), TitleHelper::create());
        helpers.insert(debug_helper::NAME.to_owned(), DebugHelper::create());
        helpers.insert(shop_helper::NAME.to_owned(), ShopHelper::create());
        helpers.insert(inventory_helper::NAME.to_owned(), InventoryHelper::create());
        helpers.insert(combat_helper::NAME.to_owned(), CombatHelper::create());
        helpers.insert(route_helper_v1::NAME.to_owned(), RouteHelperV1::create());
        helpers.insert(level_up_helper::NAME.to_owned(), LevelUpHelper::create());
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
