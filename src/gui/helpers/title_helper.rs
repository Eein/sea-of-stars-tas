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

fn relic_button_decorator(enabled: bool, name: &str) -> String {
    let enabled_string = match enabled {
        true => "[x]",
        false => "[  ]",
    };
    format!("{} {}", enabled_string, name)
}

impl GuiHelper for TitleHelper {
    fn draw(&mut self, managers: &MemoryManagers, ui: &mut egui::Ui, _tab: &mut String) {
        ui.label(format!(
            "Menu Item Selected: {:?}",
            managers.title_sequence_manager.data.title_menu.selected
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
            ui.label(relic_button_decorator(relic.enabled, &relic.name));
        }
    }
}
