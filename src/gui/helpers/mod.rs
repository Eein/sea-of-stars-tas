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

pub trait GuiHelper {
    fn name(&self) -> String;
    fn draw(&mut self, managers: &MemoryManagers, ui: &mut egui::Ui, tab: &mut String);
}
