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
        }

        egui::ComboBox::from_label("Checkpoint")
            .selected_text(
                self.checkpoint
                    .clone()
                    .unwrap_or(String::from("New Game"))
                    .to_string(),
            )
            .show_ui(ui, |ui| {
                for checkpoint in [
                    "New Game",
                    "Mooncradle Intro Cavern",
                    "Zenith Academy Dorms",
                    "Zenith Academy Dorms 2",
                    "Outside Forbidden Cavern",
                    "Before Bosslug",
                    "After Elder Mist Boss-fight",
                ] {
                    ui.selectable_value(
                        &mut self.checkpoint,
                        Some(checkpoint.to_string()),
                        checkpoint,
                    );
                }
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

        if let Some(gm) = game_manager {
            ui.separator();

            let paused = gm.is_paused();
            let text = if paused { "Resume" } else { "Pause" };
            if ui.button(text).clicked() {
                gm.pause(!paused);
            }

            if running {
                ui.label(format!("{}", gm));
            }
        }
    }
}
