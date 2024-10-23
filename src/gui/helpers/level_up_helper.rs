use super::GuiHelper;
use crate::assets::ASSETS;
use crate::memory::level_up_manager::LevelUpUpgrade;
use crate::{game_manager::GameManager, state::GameState};
use egui::Response;

pub const NAME: &str = "Level Up";

#[derive(Debug)]
pub struct LevelUpHelper;

impl LevelUpHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for LevelUpHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let level_up_manager = &game_state.memory_managers.level_up_manager.data;

        ui.label(format!("Level Up Active: {}", level_up_manager.active));
        ui.label(format!(
            "Current Character: {:?}",
            level_up_manager.current_character
        ));

        ui.label(format!("Active Index: {}", level_up_manager.upgrade_index));

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            for upgrade in level_up_manager.current_upgrades.items.iter() {
                image(ui, &upgrade.upgrade, upgrade.selected);
            }
        });
    }
}

fn image(ui: &mut egui::Ui, upgrade: &LevelUpUpgrade, selected: bool) -> Response {
    let upgrade_icon = match upgrade {
        LevelUpUpgrade::HitPoint => ASSETS.stats.hp.clone(),
        LevelUpUpgrade::SkillPoint => ASSETS.stats.mp.clone(),
        LevelUpUpgrade::PhysicalAttack => ASSETS.stats.physical_attack.clone(),
        LevelUpUpgrade::PhysicalDefense => ASSETS.stats.physical_defense.clone(),
        LevelUpUpgrade::MagicalAttack => ASSETS.stats.magical_attack.clone(),
        LevelUpUpgrade::MagicalDefense => ASSETS.stats.magical_defense.clone(),
        _ => ASSETS.stats.hp.clone(),
    };
    if selected {
        ui.add(upgrade_icon)
    } else {
        ui.add(upgrade_icon.tint(egui::Color32::WHITE.gamma_multiply(0.1)))
    }
}
