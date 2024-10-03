use super::GuiHelper;
use crate::memory::MemoryManagers;

pub const NAME: &str = "Title Helper";

#[derive(Debug)]
pub struct TitleHelper;

impl TitleHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for TitleHelper {
    fn draw(&mut self, managers: &MemoryManagers, ui: &mut egui::Ui, _tab: &mut String) {
        ui.label(format!(
            "Menu Item Selected: {:?}",
            managers
                .title_sequence_manager
                .data
                .title_menu_option_selected
        ));
        ui.label(format!(
            "Pressed Start: {:?}",
            managers.title_sequence_manager.data.pressed_start
        ));
        ui.label(format!(
            "Load Save Done: {:?}",
            managers.title_sequence_manager.data.load_save_done
        ));

        ui.separator();

        ui.label(format!(
            "Current Screen Name: {:?}",
            managers.title_sequence_manager.data.current_screen_name
        ));

        ui.separator();

        ui.label("New Game Character Select");

        ui.label(format!(
            "Left Character: {:?}",
            managers
                .title_sequence_manager
                .data
                .new_game_characters
                .left
                .character
        ));

        ui.label(format!(
            "Right Character: {:?}",
            managers
                .title_sequence_manager
                .data
                .new_game_characters
                .right
                .character
        ));

        ui.label(format!(
            "Selected Character: {:?}",
            managers
                .title_sequence_manager
                .data
                .new_game_characters
                .selected
        ));

        ui.separator();

        ui.label(format!(
            "Relics (Total: {})",
            managers.title_sequence_manager.data.relic_buttons.count
        ));
        for relic in managers
            .title_sequence_manager
            .data
            .relic_buttons
            .items
            .iter()
        {
            ui.checkbox(&mut relic.enabled.clone(), &relic.name);
        }
    }
}
