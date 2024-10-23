use super::GuiHelper;
use crate::game_manager::GameFsm;
use crate::route::tas;
use crate::{game_manager::GameManager, state::GameState};

pub const NAME: &str = "Main Helper";

#[derive(Debug)]
pub struct MainHelper {
    checkpoint: Option<String>,
    fsm: GameFsm,
}

impl MainHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self {
            checkpoint: None,
            fsm: GameFsm::Route,
        })
    }

    fn draw_title(&mut self, game_state: &mut GameState, ui: &mut egui::Ui) {
        let tsmd = &game_state.memory_managers.title_sequence_manager.data;

        match tsmd.current_screen_name.as_str() {
            "TitleScreen" => {
                if tsmd.pressed_start {
                    ui.label(format!(
                        "Menu Item Selected: {:?}",
                        tsmd.title_menu_option_selected
                    ));
                    ui.label(format!("Load Save Done: {:?}", tsmd.load_save_done));
                } else {
                    ui.label(format!("Pressed Start: {:?}", tsmd.pressed_start));
                }
            }
            "CharacterSelection" => {
                let chars = &tsmd.new_game_characters;
                ui.label("New Game Character Select");
                ui.label(format!("Left: {:?}", chars.left.character));
                ui.label(format!("Right: {:?}", chars.right.character));
                ui.label(format!("Selected: {:?}", chars.selected));
            }
            "RelicSelection" => {
                ui.label(format!("Relics (Total: {})", tsmd.relic_buttons.count));
                for relic in tsmd.relic_buttons.items.iter() {
                    let relic_selected = match &relic.selected {
                        true => "👈 👈 👈",
                        false => "",
                    };
                    let name = format!("{} {}", &relic.name, relic_selected);
                    ui.checkbox(&mut relic.enabled.clone(), name);
                }
            }
            _ => {}
        }
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

        ui.separator();

        //let tsmd = &game_state.memory_managers.title_sequence_manager.data;
        self.fsm = GameFsm::Title;

        ui.label(format!("State: {:?}", self.fsm));

        match self.fsm {
            GameFsm::Combat => {}
            GameFsm::LevelUp => {}
            GameFsm::Cutscene => {}
            GameFsm::Title => self.draw_title(game_state, ui),
            _ => {}
        }
    }
}
