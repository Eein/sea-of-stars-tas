use super::GuiHelper;
use crate::route::tas;
use crate::{game_manager::GameManager, state::GameState};

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
        }

        if ui
            .add_enabled(!running, egui::Button::new("Start TAS"))
            .clicked()
        {
            let gm = tas::create_tas();
            *game_manager = Some(gm);
        }

        if ui
            .add_enabled(!running, egui::Button::new("Start Move Test"))
            .clicked()
        {
            let gm = tas::create_movement_test();
            *game_manager = Some(gm);
        }
    }
}
