use super::GuiHelper;
use crate::seq::title::SeqTitleScreen;
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
        game_state: &GameState,
        sequencer: &mut Sequencer<GameState>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        ui.label("The Default Main Window".to_string());

        ui.label(format!(
            "Load Konami code: {}",
            &game_state.config.konami_code
        ));

        let running = sequencer.is_running();
        if ui
            .add_enabled(!running, egui::Button::new("Start TAS"))
            .clicked()
        {
            *sequencer = Sequencer::create(SeqList::create(
                "TEMP",
                vec![
                    SeqLog::create("SEQ START"),
                    SeqTitleScreen::create(),
                    SeqLog::create("SEQ DONE"),
                ],
            ));
            sequencer.start();
        }
    }
}
