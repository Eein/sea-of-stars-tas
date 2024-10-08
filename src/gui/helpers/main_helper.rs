use super::GuiHelper;
use crate::seq::SeqConfirm;
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

        let mut konami_code = false;
        if let Some(config) = &game_state.config {
            let doc = &config[0];
            konami_code = doc["konami"].as_bool().unwrap_or_default();
        }
        ui.label(format!("Load Konami code: {}", konami_code));

        let running = sequencer.is_running();
        if ui
            .add_enabled(!running, egui::Button::new("Start TAS"))
            .clicked()
        {
            *sequencer = Sequencer::create(SeqList::create(
                "TEMP",
                vec![
                    SeqLog::create("SEQ START"),
                    SeqWait::create("Wait for focus", 5.0),
                    SeqConfirm::create(0.5),
                    SeqLog::create("SEQ (1)"),
                    SeqWait::create("Wait for joystick", 0.5),
                    SeqConfirm::create(0.5),
                    SeqLog::create("SEQ (2)"),
                    SeqWait::create("Wait for joystick", 0.5),
                    SeqConfirm::create(0.5),
                    SeqLog::create("SEQ (3)"),
                    SeqWait::create("Wait for joystick", 0.5),
                    SeqConfirm::create(0.5),
                    SeqLog::create("SEQ (4)"),
                    SeqWait::create("Wait for joystick", 0.5),
                    SeqConfirm::create(0.5),
                    SeqLog::create("SEQ (5)"),
                    SeqLog::create("SEQ DONE"),
                ],
            ));
            sequencer.start();
        }
    }
}
