use super::GuiHelper;
use crate::assets::ASSETS;
use crate::memory::combat_manager::CombatDamageType;
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
            ui.separator();
            ui.label("Enemies");
            egui::Grid::new("enemies")
                .min_col_width(10.0)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Lv");
                    ui.label("Guid");
                    ui.label("Uuid");
                    ui.label("HP");
                    ui.label("Spd");
                    ui.label("Patk");
                    ui.label("Pdef");
                    ui.label("Matk");
                    ui.label("Mdef");
                    ui.label("Act");
                    ui.label("#");
                    ui.label("Locks");
                    ui.label("Mod");
                    ui.label("ModOverride");
                    ui.label("Fleshmancer");
                    ui.label("ManaSpnQty");

                    ui.end_row();

                    for (i, enemy) in combat_manager.enemies.items.iter().enumerate() {
                        ui.label(format!("NYI ({})", i));
                        ui.label(format!("{}", enemy.level));
                        ui.label(format!("{:.5}", enemy.guid));
                        ui.label(format!("{:.5}", enemy.unique_id));
                        ui.label(format!("{}/{}", enemy.current_hp, enemy.max_hp));
                        ui.label(format!("{}", enemy.speed));
                        ui.label(format!("{}", enemy.physical_attack));
                        ui.label(format!("{}", enemy.physical_defense));
                        ui.label(format!("{}", enemy.magical_attack));
                        ui.label(format!("{}", enemy.magical_defense));
                        ui.label(format!("{}", enemy.turns_to_action));
                        ui.label(format!("{}", enemy.total_spell_locks));

                        ui.horizontal(|ui| {
                            for modifier in enemy.spell_locks.items.iter() {
                                damage_type_image(ui, modifier)
                            }
                            // This is required to push the column that images dont seem to resize
                            ui.label("");
                        });

                        ui.horizontal(|ui| {
                            for modifier in enemy.damage_type_modifiers.items.iter() {
                                damage_type_image(ui, &modifier.0.key);
                                ui.label(format!("{}", &modifier.1.value));
                            }
                        });
                        ui.horizontal(|ui| {
                            for modifier in enemy.damage_type_modifiers_override.items.iter() {
                                damage_type_image(ui, &modifier.0.key);
                                ui.label(format!("{}", &modifier.1.value));
                            }
                        });

                        ui.label(format!("{}", enemy.fleshmancer_minion));
                        ui.label(format!("{}", enemy.live_mana_spawn_quantity));

                        ui.end_row();
                    }
                });
        }
        ui.separator();
        ui.label("Players");
        egui::Grid::new("players")
            .min_col_width(10.0)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label("Selected");
                ui.label("Enabled");
                ui.label("HP");
                ui.label("MP");
                ui.label("Patk");
                ui.label("Pdef");
                ui.label("Matk");
                ui.label("Mdef");
                ui.label("Dead");
                ui.label("Timed Atk Rdy");
                ui.label("Charge");
                ui.label("Weapon");
                ui.label("Armor");
                ui.label("Trinket 1");
                ui.label("2");
                ui.end_row();

                for (i, player) in combat_manager.players.items.iter().enumerate() {
                    ui.label(format!("{:?} ({})", player.character, i));
                    ui.label(format!("{}", player.selected));
                    ui.label(format!("{}", player.enabled));
                    ui.label(format!("{}", player.current_hp));
                    ui.label(format!("{}", player.current_mp));
                    ui.label(format!(
                        "{}|{}",
                        player.base_physical_attack, player.physical_attack
                    ));
                    ui.label(format!(
                        "{}|{}",
                        player.base_physical_defense, player.physical_defense
                    ));
                    ui.label(format!(
                        "{}|{}",
                        player.base_magical_attack, player.magical_attack
                    ));
                    ui.label(format!(
                        "{}|{}",
                        player.base_magical_defense, player.magical_defense
                    ));
                    ui.label(format!("{}", player.dead));
                    ui.label(format!("{}", player.timed_attack_ready));
                    ui.label(format!("{}", player.mana_charge_count));
                    ui.label(
                        player
                            .equipped_weapon
                            .clone()
                            .unwrap_or(data::Item::default())
                            .name
                            .to_string(),
                    );
                    ui.label(
                        player
                            .equipped_armor
                            .clone()
                            .unwrap_or(data::Item::default())
                            .name
                            .to_string(),
                    );
                    for trinket in player.equipped_trinkets.iter() {
                        ui.label(
                            trinket
                                .clone()
                                .trinket
                                .unwrap_or(data::Item::default())
                                .name
                                .to_string(),
                        );
                    }
                    ui.end_row();
                }
            });
        ui.separator();
    }
}

fn damage_type_image(ui: &mut egui::Ui, damage_type: &CombatDamageType) {
    let damage_type_icon = match damage_type {
        CombatDamageType::Arcane => ASSETS.damage_types.arcane.clone(),
        CombatDamageType::Blunt => ASSETS.damage_types.blunt.clone(),
        CombatDamageType::Moon => ASSETS.damage_types.moon.clone(),
        CombatDamageType::Poison => ASSETS.damage_types.poison.clone(),
        CombatDamageType::Sun => ASSETS.damage_types.sun.clone(),
        CombatDamageType::Sword => ASSETS.damage_types.sword.clone(),
        _ => ASSETS.stats.hp.clone(),
    };
    ui.add(damage_type_icon);
}
