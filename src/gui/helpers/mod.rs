pub mod main_helper;
pub mod nav_helper;
pub mod title_helper;

pub use self::{main_helper::*, nav_helper::*, title_helper::*};
use crate::memory::MemoryManagers;

#[derive(Default)]
pub struct GuiHelpers {
    pub nav_helper: NavHelper,
    pub main_helper: MainHelper,
    pub title_helper: TitleHelper,
}

impl GuiHelpers {
    pub fn tree_names(&self) -> Vec<String> {
        vec![
            self.nav_helper.name(),
            self.main_helper.name(),
            self.title_helper.name(),
        ]
    }

    pub fn draw(&mut self, managers: &MemoryManagers, ui: &mut egui::Ui, tab: &mut String) {
        match tab.as_str() {
            nav_helper::NAME => self.nav_helper.draw(managers, ui, tab),
            main_helper::NAME => self.main_helper.draw(managers, ui, tab),
            title_helper::NAME => self.title_helper.draw(managers, ui, tab),
            _ => {
                let label = format!("Tab: {} has not been initialized. Check gui/mod.rs and state.rs to initialize this tab.", tab.as_str());
                ui.label(label);
            }
        }
    }
}

pub trait GuiHelper {
    fn name(&self) -> String;
    fn draw(&mut self, managers: &MemoryManagers, ui: &mut egui::Ui, tab: &mut String);
}
