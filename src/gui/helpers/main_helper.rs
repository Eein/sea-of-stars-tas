use super::GuiHelper;
use crate::route::tas;
use crate::state::GameState;
use seq::prelude::*;

pub const NAME: &str = "Main Helper";

#[derive(Debug)]
pub struct MainHelper;

impl MainHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for MainHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        sequencer: &mut Sequencer<GameState>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        ui.label("TAS Option".to_string());

        ui.checkbox(&mut game_state.config.konami_code, "Konami Code");

        ui.separator();

        let running = sequencer.is_running();
        if ui
            .add_enabled(!running, egui::Button::new("Start TAS"))
            .clicked()
        {
            *sequencer = tas::create_sequencer();
            sequencer.start();
        }
    }
}
