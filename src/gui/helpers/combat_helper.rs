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

                        let mut lock_str = String::from("");
                        for lock in enemy.spell_locks.items.iter() {
                            lock_str = format!("{} {:?}", &lock_str, &lock);
                        }
                        ui.label(lock_str);

                        let mut damage_modifiers_str = String::from("");
                        for modifier in enemy.damage_type_modifiers.items.iter() {
                            damage_modifiers_str = format!(
                                "{} {:?} {}",
                                damage_modifiers_str, modifier.0.key, modifier.1.value
                            );
                        }
                        ui.label(damage_modifiers_str);

                        let mut damage_modifiers_override_str = String::from("");
                        for modifier in enemy.damage_type_modifiers_override.items.iter() {
                            damage_modifiers_override_str = format!(
                                "{} {:?} {}",
                                damage_modifiers_override_str, modifier.0.key, modifier.1.value
                            );
                        }
                        ui.label(damage_modifiers_override_str);
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

        // TODO(eein): Alternatively the following; not sure if we want to keep
        // the same style here or not.
        // for (i, enemy) in combat_manager.enemies.items.iter().enumerate() {
        //     ui.label(format!("Name NYI ({})", i));
        //     ui.label(format!("GUID: {}", enemy.guid));
        //     ui.label(format!("Unique ID: {}", enemy.unique_id));
        //     ui.label(format!("HP: {}/{}", enemy.current_hp, enemy.max_hp));
        //     ui.label(format!("Speed: {}", enemy.speed));
        //     ui.label(format!("Physical Attack: {}", enemy.physical_attack));
        //     ui.label(format!("Physical Defense: {}", enemy.physical_defense));
        //     ui.label(format!("Magical Attack: {}", enemy.magical_attack));
        //     ui.label(format!("Magical Defense: {}", enemy.magical_defense));
        //     ui.label(format!("Next Action: {}", enemy.turns_to_action));
        //     ui.label(format!("Locks: {}", enemy.total_spell_locks));
        //     for lock in enemy.spell_locks.items.iter() {
        //         ui.label(format!("{:?}", lock));
        //     }
        // }

        // TODO(eein): Another alternative where they are collapsable headers
        // egui::Grid::new("enemies")
        //     .min_col_width(10.0)
        //     .striped(true)
        //     .show(ui, |ui| {

        //         for (i, enemy) in combat_manager.enemies.items.iter().enumerate() {
        //            let mut header = format!("NYI ({})  G: {:.5}  U: {:.5}  HP: {}/{}  Act: {}  Locks ({})  ",
        //                 i,
        //                 enemy.guid,
        //                 enemy.unique_id,
        //                 enemy.current_hp,
        //                 enemy.max_hp,
        //                 enemy.turns_to_action,
        //                 enemy.total_spell_locks);

        //             for lock in enemy.spell_locks.items.iter() {
        //                 header = format!("{} {:?}", header, lock);
        //             }

        //             egui::CollapsingHeader::new(header)
        //                 .default_open(true)
        //                 .show(ui, |ui| {
        //                 ui.label(format!("Spd: {}, Patk: {}, Pdef: {}, Matk: {}, Mdef: {}",
        //                     enemy.speed,
        //                     enemy.physical_attack,
        //                     enemy.physical_defense,
        //                     enemy.magical_attack,
        //                     enemy.magical_defense,
        //                 ));
        //             });
        //             ui.end_row();
        //         }
        //     });
        // ui.separator();
    }
}
