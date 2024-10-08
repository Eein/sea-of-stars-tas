use super::GuiHelper;
use crate::state::GameState;

pub const NAME: &str = "Main Helper";

#[derive(Debug)]
pub struct MainHelper;

impl MainHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for MainHelper {
    fn draw(&mut self, _game_state: &GameState, ui: &mut egui::Ui, _tab: &mut String) {
        ui.label("The Default Main Window".to_string());
    }
}
