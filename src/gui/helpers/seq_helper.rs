use super::GuiHelper;
use crate::state::GameState;
use seq::prelude::*;

pub const NAME: &str = "Seq Helper";

#[derive(Debug)]
pub struct SeqGuiHelper;

impl SeqGuiHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for SeqGuiHelper {
    fn draw(
        &mut self,
        _game_state: &GameState,
        _sequencer: &mut Sequencer<GameState>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        ui.label("The SeqGuiHelper Window".to_string());
    }
}
