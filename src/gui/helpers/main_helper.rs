use super::GuiHelper;
use crate::route::tas;
use crate::{game_manager::GameManager, state::GameState};

pub const NAME: &str = "Main Helper";

#[derive(Debug)]
pub struct MainHelper {
    checkpoint: Option<String>,
}

impl MainHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self { checkpoint: None })
    }
}

impl GuiHelper for MainHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        ui.label("TAS Option".to_string());

        ui.checkbox(&mut game_state.config.konami_code, "Konami Code");

        ui.separator();

        let mut running = false;
        if let Some(gm) = game_manager {
            running = gm.is_running();
            if running {
                ui.label(format!("FSM: {}", gm));
            }

            let paused = gm.is_paused();
            let text = if paused { "Resume" } else { "Pause" };
            if ui.button(text).clicked() {
                gm.pause(!paused);
            }
        }

        egui::ComboBox::from_label("Checkpoint")
            .selected_text(format!("{:?}", self.checkpoint))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.checkpoint, None, "New Game");
                ui.selectable_value(
                    &mut self.checkpoint,
                    Some("intro_mooncradle".to_owned()),
                    "Mooncradle Intro Cavern",
                );
                ui.selectable_value(
                    &mut self.checkpoint,
                    Some("intro_dorms".to_owned()),
                    "Zenith Academy Dorms",
                );
                ui.selectable_value(
                    &mut self.checkpoint,
                    Some("intro_dorms2".to_owned()),
                    "Zenith Academy Dorms 2",
                );
                ui.selectable_value(
                    &mut self.checkpoint,
                    Some("forbidden_cave".to_owned()),
                    "Outside Forbidden Cavern",
                );
                ui.selectable_value(
                    &mut self.checkpoint,
                    Some("forbidden_cave2".to_owned()),
                    "Before Bosslug",
                );
                // TODO: More checkpoints
                ui.selectable_value(
                    &mut self.checkpoint,
                    Some("elder_mist_boss2".to_owned()),
                    "After Elder Mist Boss-fight",
                );
            });

        if ui
            .add_enabled(!running, egui::Button::new("Start TAS"))
            .clicked()
        {
            let mut gm = tas::create_tas();
            if let Some(checkpoint) = &self.checkpoint {
                gm.advance_to_checkpoint(game_state, checkpoint);
            }
            *game_manager = Some(gm);
        }
    }
}
