use super::GuiHelper;
use crate::route::tas;
use crate::{game_manager::GameManager, state::GameState};

use crate::assets::ASSETS;
use crate::memory::combat_manager::CombatDamageType;
use crate::memory::level_up_manager::LevelUpUpgrade;

pub const NAME: &str = "Main Helper";

#[derive(Debug)]
pub struct MainHelper {
    checkpoint: Option<String>,
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

fn stat_image(ui: &mut egui::Ui, upgrade: &LevelUpUpgrade, selected: bool) -> egui::Response {
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

impl MainHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self { checkpoint: None })
    }

    fn draw_title(&self, game_state: &GameState, ui: &mut egui::Ui) {
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

    fn draw_combat(&self, game_state: &GameState, ui: &mut egui::Ui) {
        let cmd = &game_state.memory_managers.combat_manager.data;

        ui.label(format!(
            "Combat Controller: {:?}",
            cmd.combat_controller_type
        ));
        ui.label(format!(
            "Live Mana: Small: {} | Big: {}",
            cmd.live_mana.small, cmd.live_mana.big
        ));
        ui.label(format!(
            "Combo: {}/3000 | Filled: {}/3 | Ultimate Progress: {:.3}%",
            cmd.combo_point_progress, cmd.combo_points, cmd.ultimate_progress
        ));
        ui.separator();
        egui::CollapsingHeader::new("Enemies")
            .default_open(true)
            .show(ui, |ui| {
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

                        for (i, enemy) in cmd.enemies.items.iter().enumerate() {
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
            });
    }

    fn draw_players(&self, game_state: &GameState, ui: &mut egui::Ui) {
        let cmd = &game_state.memory_managers.combat_manager.data;

        egui::CollapsingHeader::new("Players")
            .default_open(true)
            .show(ui, |ui| {
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

                        for (i, player) in cmd.players.items.iter().enumerate() {
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
            });
    }

    fn draw_level_up(&self, game_state: &mut GameState, ui: &mut egui::Ui) {
        let lum = &game_state.memory_managers.level_up_manager.data;

        ui.label(format!("Current Character: {:?}", lum.current_character));

        ui.label(format!("Active Index: {}", lum.upgrade_index));

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            for upgrade in lum.current_upgrades.items.iter() {
                stat_image(ui, &upgrade.upgrade, upgrade.selected);
            }
        });
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
        let mut running = false;
        if let Some(gm) = game_manager {
            running = gm.is_running();
        }

        if !running {
            ui.label("TAS Option".to_string());

            ui.checkbox(&mut game_state.config.konami_code, "Konami Code");

            ui.separator();

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

        let cmd = &game_state.memory_managers.combat_manager.data;
        let lum = &game_state.memory_managers.level_up_manager.data;
        let tsmd = &game_state.memory_managers.title_sequence_manager.data;

        if cmd.encounter_active {
            self.draw_combat(game_state, ui);
            self.draw_players(game_state, ui);
        } else if lum.active {
            self.draw_level_up(game_state, ui);
        } else if tsmd.active {
            self.draw_title(game_state, ui);
        }
    }
}
