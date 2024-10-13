use super::GuiHelper;
use crate::{game_manager::GameManager, state::GameState};

pub const NAME: &str = "Title Helper";

#[derive(Debug)]
pub struct TitleHelper;

impl TitleHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for TitleHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut GameManager,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let tsmd = &game_state.memory_managers.title_sequence_manager.data;
        ui.label(format!(
            "Menu Item Selected: {:?}",
            tsmd.title_menu_option_selected
        ));
        ui.label(format!("Pressed Start: {:?}", tsmd.pressed_start));
        ui.label(format!("Load Save Done: {:?}", tsmd.load_save_done));

        ui.separator();

        ui.label(format!(
            "Current Screen Name: {:?}",
            tsmd.current_screen_name
        ));

        ui.separator();

        ui.label("New Game Character Select");

        ui.label(format!(
            "Left Character: {:?}",
            tsmd.new_game_characters.left.character
        ));

        ui.label(format!(
            "Right Character: {:?}",
            tsmd.new_game_characters.right.character
        ));

        ui.label(format!(
            "Selected Character: {:?}",
            tsmd.new_game_characters.selected
        ));

        ui.separator();

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
}
