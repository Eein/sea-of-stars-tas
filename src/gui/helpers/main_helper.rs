use super::GuiHelper;
use crate::cli::{Route, repro_command};
use crate::route::tas;
use crate::{game_manager::GameManager, state::GameState};

use crate::assets::ASSETS;
use crate::memory::combat_manager::CombatDamageType;
use crate::memory::combat_manager::{CombatEnemy, CombatPlayer};
use crate::memory::level_up_manager::LevelUpUpgrade;

use data::prelude::PlayerPartyCharacter;

use delta::Timer;
use log::info;

pub const NAME: &str = "Main Helper";

#[derive(Debug)]
pub struct MainHelper {
    checkpoint: Option<String>,
    save_slot: usize,
    auto_save_present: bool,
    timer: Timer,
    countdown: Option<f64>,
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

const COUNTDOWN_TIMEOUT: f64 = 5.0;

impl MainHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self {
            checkpoint: None,
            save_slot: 1,
            auto_save_present: true,
            timer: delta::Timer::new(),
            countdown: None,
        })
    }

    fn handle_countdown(&mut self) -> bool {
        let dt = self.timer.mark_secs();
        if let Some(timer) = self.countdown.as_mut() {
            if timer.floor() != (*timer - dt).floor() {
                info!("Counting down to TAS start: {}", timer.floor());
            }
            *timer -= dt;
            if *timer <= 0.0 {
                self.countdown = None;
                return true;
            }
        }
        false
    }

    fn draw_title(&self, game_state: &GameState, ui: &mut egui::Ui) {
        let tsmd = &game_state.memory_managers.title_sequence_manager.data;

        ui.label("Title Screen Active");
        match tsmd.current_screen_name.as_str() {
            "DifficultySelection" => {
                ui.label("Difficulty Selection");
                if let Some(difficulty) = &tsmd.selected_difficulty {
                    ui.label(format!("Selected: {:?}", difficulty));
                } else {
                    ui.label("Screen inactive");
                }
            }
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
                    let name = format!("{} {}", relic.name, relic_selected);
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
        ui.label(format!("Selected Character: {:?}", cmd.selected_character));
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
                                    ui.label(format!("{}", modifier.1.value));
                                }
                            });
                            ui.horizontal(|ui| {
                                for modifier in enemy.damage_type_modifiers_override.items.iter() {
                                    damage_type_image(ui, &modifier.0.key);
                                    ui.label(format!("{}", modifier.1.value));
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
                        ui.label("Lv");
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
                            ui.label(format!("{}", player.level));
                            ui.label(format!("{}", player.selected));
                            ui.label(format!("{}", player.enabled));
                            ui.label(format!("{}/{}", player.current_hp, player.max_hp));
                            ui.label(format!("{}/{}", player.current_mp, player.max_mp));
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

        ui.label("Level Up Active");
        ui.label(format!("Current Character: {:?}", lum.current_character));
        ui.label(format!("Active Index: {}", lum.upgrade_index));
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            for upgrade in lum.current_upgrades.items.iter() {
                stat_image(ui, &upgrade.upgrade, upgrade.selected);
            }
        });
    }

    fn draw_damage_calculations(&self, game_state: &mut GameState, ui: &mut egui::Ui) {
        let gmd = &game_state.memory_managers.combat_manager.data;
        // for each player
        for player in &gmd.players.items {
            // for each enemy
            ui.label(format!("{:?}", player.character));
            for enemy in &gmd.enemies.items {
                //calculate basic attack damage
                let min_damage = Self::calculate_basic_attack_damage(player, enemy, 0.0);
                let max_damage = Self::calculate_basic_attack_damage(player, enemy, 3.0);
                ui.label(format!(
                    "Enemy: {:.5} | Damage: {:.3}-{:.3} Timed: {:.3}-{:.3}",
                    enemy.unique_id,
                    min_damage.0.round(),
                    max_damage.0.round(),
                    (min_damage.0 + min_damage.1).round(),
                    (max_damage.0 + max_damage.1).round()
                ));
            }
        }
    }

    // The final damage calculation is (int)value, so just floor them.
    fn calculate_basic_attack_damage(
        player: &CombatPlayer,
        enemy: &CombatEnemy,
        random: f32,
    ) -> (f32, f32) {
        // floats are very specific - this matters
        const PHYSICAL_DEFENSE_CAP: f32 = 150.0;
        const MANA_CHARGE_STAT_MULTIPLIER: f32 = 0.330000;
        const MAGICAL_DEFENSE_CAP: f32 = 150.0;
        const TIMED_HIT_MULTIPLIER: f32 = 1.299999; // in globalCombatSettings -> basicAttackTimedHitMultiplier

        // manually do this for now
        let damage_type_attack_modifier = match player.character {
            PlayerPartyCharacter::Zale => {
                if player.mana_charge_count > 0 {
                    // find modifier in enemy weaknesses
                    if let Some((_key, value)) = enemy
                        .damage_type_modifiers
                        .items
                        .iter()
                        .find(|(k, _v)| k.key == CombatDamageType::Sun)
                    {
                        value.value
                    } else {
                        1.0
                    }
                } else {
                    1.0
                }
            }
            PlayerPartyCharacter::Valere => {
                if player.mana_charge_count > 0 {
                    if let Some((_key, value)) = enemy
                        .damage_type_modifiers
                        .items
                        .iter()
                        .find(|(k, _v)| k.key == CombatDamageType::Moon)
                    {
                        value.value
                    } else {
                        1.0
                    }
                } else {
                    1.0
                }
            }
            _ => 1.0,
        };

        // Apply boosted damage emodifier to physical attack on basic attacks
        let boosted_live_mana_attack = player.magical_attack as f32
            * MANA_CHARGE_STAT_MULTIPLIER
            * damage_type_attack_modifier; // + random;

        let total_physical_attack =
            (damage_type_attack_modifier * player.physical_attack as f32) + random;

        let magical_defense_cap_ratio = enemy.magical_defense as f32 / MAGICAL_DEFENSE_CAP;
        let physical_defense_cap_ratio = enemy.physical_defense as f32 / PHYSICAL_DEFENSE_CAP;

        let mut total_magical_attack = boosted_live_mana_attack * player.mana_charge_count as f32;
        total_magical_attack *= 1.0 - magical_defense_cap_ratio; // deduct enemy defense ratio

        let mut total_physical_attack = total_physical_attack;
        total_physical_attack *= 1.0 - physical_defense_cap_ratio; // deduct enemy defense ratio

        let total_attack = total_magical_attack + total_physical_attack;

        let timed_hit_damage = (TIMED_HIT_MULTIPLIER * total_attack) - total_attack;

        (total_attack, timed_hit_damage)
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
            let countdown_finished = self.handle_countdown();
            if countdown_finished {
                gm.start(game_state);
            }

            running = gm.is_running();
        }

        if let Some(countdown) = self.countdown {
            ui.label(format!("Counting down... {:.3}", countdown));
        } else if !running {
            ui.label("TAS Option".to_string());

            ui.checkbox(&mut game_state.config.konami_code, "Konami Code");
            ui.checkbox(&mut game_state.config.solstice_diploma, "Solstice Diploma");

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

            if let Some(checkpoint) = &self.checkpoint
                && checkpoint != "New Game"
            {
                egui::ComboBox::from_label("Save slot")
                    .selected_text(self.save_slot.to_string())
                    .show_ui(ui, |ui| {
                        for value in 1..=9 {
                            ui.selectable_value(&mut self.save_slot, value, value.to_string());
                        }
                    });
                ui.checkbox(&mut self.auto_save_present, "Auto save present");
                if ui.button("Run Load Sequence").clicked() {
                    info!(
                        "Reproduce this run with: {}",
                        repro_command(
                            Route::Load,
                            None,
                            Some(self.save_slot),
                            self.auto_save_present,
                            &game_state.config,
                        )
                    );
                    *game_manager = Some(tas::create_load_sequence(
                        self.save_slot,
                        self.auto_save_present,
                    ));
                    self.countdown = Some(COUNTDOWN_TIMEOUT);
                }

                ui.separator();
            }

            if ui
                .add_enabled(!running, egui::Button::new("Start TAS"))
                .clicked()
            {
                info!(
                    "Reproduce this run with: {}",
                    repro_command(
                        Route::Tas,
                        self.checkpoint.as_deref(),
                        // A real checkpoint resumes from a save; otherwise new game.
                        self.checkpoint
                            .as_deref()
                            .filter(|c| *c != "New Game")
                            .map(|_| self.save_slot),
                        self.auto_save_present,
                        &game_state.config,
                    )
                );
                let mut gm = tas::create_tas();
                if let Some(checkpoint) = &self.checkpoint {
                    gm.advance_to_checkpoint(game_state, checkpoint);
                }
                *game_manager = Some(gm);
                self.countdown = Some(COUNTDOWN_TIMEOUT);
            }
            if ui
                .add_enabled(!running, egui::Button::new("Start Combat Test"))
                .clicked()
            {
                info!(
                    "Reproduce this run with: {}",
                    repro_command(Route::Combat, None, None, false, &game_state.config)
                );
                let gm = tas::create_combat_test();
                *game_manager = Some(gm);
                self.countdown = Some(1.0);
            }
            // Debug: runs only the relic-selection sequence. Position the game on
            // the Difficulty Selection screen before clicking.
            if ui
                .add_enabled(!running, egui::Button::new("Start Relic Test"))
                .clicked()
            {
                info!(
                    "Reproduce this run with: {}",
                    repro_command(Route::Relic, None, None, false, &game_state.config)
                );
                let gm = tas::create_relic_test();
                *game_manager = Some(gm);
                self.countdown = Some(1.0);
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

                // Display player positions (game object)
                ui.separator();
                let sppmd = &game_state.memory_managers.single_player_plus_manager.data;
                for (idx, player) in sppmd.players.items.iter().enumerate() {
                    let gameobject_position = player.gameobject_position.unwrap_or_default();
                    ui.label(format!("P{} pos: {:?}", idx + 1, gameobject_position));
                }
            }
        }

        ui.separator();

        let cmd = &game_state.memory_managers.combat_manager.data;
        let lum = &game_state.memory_managers.level_up_manager.data;
        let tsmd = &game_state.memory_managers.title_sequence_manager.data;

        if cmd.encounter_active {
            self.draw_combat(game_state, ui);
            self.draw_players(game_state, ui);
            self.draw_damage_calculations(game_state, ui);
        } else if lum.active {
            self.draw_level_up(game_state, ui);
        } else if tsmd.active {
            self.draw_title(game_state, ui);
        }
    }
}
