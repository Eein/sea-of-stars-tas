pub mod main_helper;
pub mod nav_helper;
pub mod title_helper;

use std::collections::HashMap;

pub use self::{main_helper::*, nav_helper::*, title_helper::*};
use crate::state::GameState;
use seq::prelude::*;

pub struct GuiHelpers {
    pub helpers: HashMap<String, Box<dyn GuiHelper>>,
}

impl Default for GuiHelpers {
    fn default() -> Self {
        let mut helpers: HashMap<String, Box<dyn GuiHelper>> = HashMap::new();
        helpers.insert(nav_helper::NAME.to_owned(), NavHelper::create());
        helpers.insert(main_helper::NAME.to_owned(), MainHelper::create());
        helpers.insert(title_helper::NAME.to_owned(), TitleHelper::create());
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
        game_state: &GameState,
        sequencer: &Sequencer<GameState>,
        ui: &mut egui::Ui,
        tab: &mut String,
    ) {
        match self.helpers.get_mut(tab.as_str()) {
            Some(helper) => helper.draw(game_state, sequencer, ui, tab),
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
        game_state: &GameState,
        sequencer: &Sequencer<GameState>,
        ui: &mut egui::Ui,
        tab: &mut String,
    );
}
