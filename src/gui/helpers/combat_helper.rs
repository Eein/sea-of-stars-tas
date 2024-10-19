use super::GuiHelper;
use crate::{game_manager::GameManager, state::GameState};

pub const NAME: &str = "Combat";

#[derive(Debug)]
pub struct CombatHelper;

impl CombatHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for CombatHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let combat_manager = &game_state.memory_managers.combat_manager.data;

        ui.label(format!(
            "Encounter Active: {}",
            combat_manager.encounter_active
        ));
        if combat_manager.encounter_active {
            ui.label(format!(
                "Combat Controller: {:?}",
                combat_manager.combat_controller_type
            ));
            ui.label(format!(
                "Live Mana: Small: {} | Big: {}",
                combat_manager.live_mana.small, combat_manager.live_mana.big
            ));
            ui.label(format!(
                "Combo: {}/3000 | Filled: {}/3 | Ultimate Progress: {:.3}%",
                combat_manager.combo_point_progress,
                combat_manager.combo_points,
                combat_manager.ultimate_progress
            ));
            ui.label("Enemies");
            for enemy in combat_manager.enemies.items.iter() {
                ui.label(format!("GUID: {}", enemy.guid));
                ui.label(format!("Unique ID: {}", enemy.unique_id));
                ui.label(format!("HP: {}/{}", enemy.current_hp, enemy.max_hp));
                ui.label(format!("Speed: {}", enemy.speed));
                ui.label(format!("Physical Attack: {}", enemy.physical_attack));
                ui.label(format!("Physical Defense: {}", enemy.physical_defense));
                ui.label(format!("Magical Attack: {}", enemy.magical_attack));
                ui.label(format!("Magical Defense: {}", enemy.magical_defense));
                ui.label(format!("Next Action: {}", enemy.turns_to_action));
                ui.label(format!("Locks: {}", enemy.total_spell_locks));
                for lock in enemy.spell_locks.items.iter() {
                    ui.label(format!("{:?}", lock));
                }
            }
        }
    }
}
