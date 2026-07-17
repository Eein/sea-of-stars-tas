use super::GuiHelper;
use crate::cli::{Route, repro_command};
use crate::route::tas;
use crate::{state::GameState, tas_runner::TasRunner};

use crate::assets::ASSETS;
use crate::combat::{CombatController, appraisal};
use crate::memory::combat_manager::CombatDamageType;
use crate::memory::level_up_manager::LevelUpUpgrade;

use delta::Timer;
use log::info;

pub const NAME: &str = "Main Helper";

#[derive(Debug)]
pub struct MainHelper {
    checkpoint: Option<String>,
    /// Checkpoint names harvested from the route definition (built once, on
    /// first draw) — the picker can't drift from the actual route.
    checkpoints: Vec<String>,
    save_slot: usize,
    auto_save_present: bool,
    timer: Timer,
    countdown: Option<f64>,
    /// Where the last "Dump State" click wrote its snapshot, for display.
    last_dump: Option<String>,
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
            checkpoints: Vec::new(),
            save_slot: 1,
            auto_save_present: true,
            timer: delta::Timer::new(),
            countdown: None,
            last_dump: None,
        })
    }

    /// Write a one-shot state snapshot (the CLI `--dump-state` fields) to a
    /// timestamped file in the working directory, for sharing/debugging.
    fn dump_state_to_file(&mut self, game_state: &GameState) {
        let text = crate::cli::snapshot_fields(game_state)
            .iter()
            .map(|(key, value)| format!("{key}: {value}"))
            .collect::<Vec<_>>()
            .join("\n");
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let path = format!("state-dump-{stamp}.txt");
        match std::fs::write(&path, text) {
            Ok(()) => {
                info!("State dump written to {path}");
                self.last_dump = Some(path);
            }
            Err(e) => {
                info!("State dump failed: {e}");
                self.last_dump = Some(format!("write failed: {e}"));
            }
        }
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

    /// Show the appraisal (decision) layer's ranked candidate actions and the
    /// chosen top action.
    fn draw_appraisals(
        &self,
        game_state: &GameState,
        combat: Option<&CombatController>,
        ui: &mut egui::Ui,
    ) {
        const LETHAL: egui::Color32 = egui::Color32::from_rgb(120, 220, 120);
        const CHOSEN: egui::Color32 = egui::Color32::from_rgb(255, 210, 90);
        /// The latched, in-progress action — brighter than CHOSEN so the two
        /// lines read apart at a glance.
        const EXECUTING: egui::Color32 = egui::Color32::from_rgb(130, 200, 255);

        let cmd = &game_state.memory_managers.combat_manager.data;
        // Prefer the running executor's cached ranking, so the panel shows
        // exactly what it is acting on; recompute only when no executor is
        // live (e.g. watching a fight with no route running).
        let recomputed;
        let (appraisals, chosen) = match combat {
            Some(combat) => (combat.appraisals.as_slice(), combat.chosen.as_ref()),
            None => {
                recomputed = appraisal::generate_appraisals(cmd);
                (recomputed.as_slice(), appraisal::choose(&recomputed))
            }
        };

        egui::CollapsingHeader::new("Appraisals")
            .default_open(true)
            .show(ui, |ui| {
                // Where the executor's turn machine is right now — the first
                // thing to read when a fight looks stalled.
                match combat {
                    Some(combat) => {
                        ui.label(format!("Executor: {}", combat.turn_status(game_state)));
                    }
                    None => {
                        ui.weak("Executor: (not running)");
                    }
                }
                // What is actually being driven right now: the appraisal
                // latched into the in-progress action. Unlike "Chosen" below
                // (the live re-ranked pick), this stays fixed for the whole
                // action.
                if let Some((executing, step)) = combat.and_then(|c| c.executing()) {
                    ui.horizontal(|ui| {
                        ui.label("Executing:");
                        ui.colored_label(
                            EXECUTING,
                            format!("{} @ {:?}", executing.describe(), step),
                        );
                    });
                }
                // Callout for the current decision.
                ui.horizontal(|ui| {
                    ui.label("Chosen:");
                    match chosen {
                        Some(best) => {
                            ui.colored_label(
                                CHOSEN,
                                format!(
                                    "{:?} -> {} on {:.5}",
                                    best.attacker,
                                    best.action.label(),
                                    best.target_enemy_id,
                                ),
                            );
                            ui.label(format!(
                                "dmg {:.0}{}{}",
                                best.expected_damage,
                                match best.mana_charges {
                                    0 => String::new(),
                                    n => format!(" LM{n}"),
                                },
                                if best.lethal { " (lethal)" } else { "" },
                            ));
                        }
                        None => {
                            ui.weak("(none)");
                        }
                    }
                });

                // Live executor signals — use these to verify the (drift-prone)
                // battle-command + target-cursor offsets against the game.
                // Label from the live ring (scripted fights remove commands
                // and shift the indices), falling back to the fixed mapping
                // when the ring isn't readable.
                let command = cmd
                    .battle_command_index
                    .map(|idx| {
                        usize::try_from(idx)
                            .ok()
                            .and_then(|i| cmd.battle_command_ring.get(i))
                            .map(|(name, _)| name.as_str())
                            .unwrap_or(match idx {
                                0 => "Attack",
                                1 => "Skill",
                                2 => "Combo",
                                3 => "Item",
                                _ => "-",
                            })
                    })
                    .unwrap_or("-");
                ui.label(format!(
                    "Command ring: focus={} idx={:?} ({})",
                    cmd.battle_command_has_focus, cmd.battle_command_index, command,
                ));
                ui.horizontal(|ui| {
                    ui.label("Cursor target:");
                    match &cmd.selected_attack_target_guid {
                        Some(guid) => {
                            let matches_chosen =
                                chosen.is_some_and(|best| best.target_enemy_id == *guid);
                            let color = if matches_chosen { LETHAL } else { CHOSEN };
                            ui.colored_label(color, format!("{:.5}", guid));
                            ui.weak(if matches_chosen {
                                "= chosen"
                            } else {
                                "!= chosen"
                            });
                        }
                        None => {
                            ui.weak("(none)");
                        }
                    }
                });

                if appraisals.is_empty() {
                    return;
                }

                egui::Grid::new("appraisals")
                    .min_col_width(10.0)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("#");
                        ui.label("Attacker");
                        ui.label("Action");
                        ui.label("Target");
                        ui.label("Dmg");
                        ui.label("LM");
                        ui.label("Lethal");
                        ui.label("Score");
                        ui.end_row();

                        for (i, appraisal) in appraisals.iter().enumerate().take(12) {
                            let rank_color = if i == 0 { Some(CHOSEN) } else { None };
                            let colored = |ui: &mut egui::Ui, text: String| match rank_color {
                                Some(c) => {
                                    ui.colored_label(c, text);
                                }
                                None => {
                                    ui.label(text);
                                }
                            };

                            colored(ui, format!("{}", i + 1));
                            colored(ui, format!("{:?}", appraisal.attacker));
                            colored(ui, appraisal.action.label());
                            colored(ui, format!("{:.5}", appraisal.target_enemy_id));
                            colored(ui, format!("{:.0}", appraisal.expected_damage));
                            match appraisal.mana_charges {
                                0 => {
                                    ui.label("");
                                }
                                n => colored(ui, format!("LM{n}")),
                            }
                            if appraisal.lethal {
                                ui.colored_label(LETHAL, "kill");
                            } else {
                                ui.label("");
                            }
                            colored(ui, format!("{:.1}", appraisal.score));
                            ui.end_row();
                        }
                    });
            });
    }

    /// Per-character move enumeration (id + costs), read from each actor's
    /// `fighterDefinition.allMoveDefinitions`.
    fn draw_moves(&self, game_state: &mut GameState, ui: &mut egui::Ui) {
        let cmd = &game_state.memory_managers.combat_manager.data;
        egui::CollapsingHeader::new("Moves")
            .default_open(false)
            .show(ui, |ui| {
                for character_moves in &cmd.moves {
                    ui.label(format!("{:?}", character_moves.character));
                    for m in &character_moves.moves {
                        let targeting =
                            m.main_target_guid.is_some() || m.current_target_guid.is_some();
                        // Available = in the fighter's per-fight loaded lists
                        // AND learned per progression — the appraiser's gate.
                        let available = m.loaded && m.unlocked;
                        let text = format!(
                            "    {}  cp={}  sp={}{}{}",
                            m.move_id.as_deref().unwrap_or("?"),
                            m.combo_point_cost
                                .map(|c| c.to_string())
                                .unwrap_or_else(|| "-".to_string()),
                            m.skill_point_cost
                                .map(|c| c.to_string())
                                .unwrap_or_else(|| "-".to_string()),
                            match (m.loaded, m.unlocked) {
                                (true, true) => "",
                                (true, false) => "  (locked)",
                                (false, _) => "  (not loaded)",
                            },
                            if targeting { "  👈 👈 👈" } else { "" },
                        );
                        if available {
                            ui.label(text);
                        } else {
                            ui.weak(text);
                        }
                    }
                }
            });
    }
}

impl GuiHelper for MainHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        tas_runner: &mut Option<TasRunner>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        // Always available, even mid-run — snapshot the live state for sharing.
        ui.horizontal(|ui| {
            if ui.button("Dump State").clicked() {
                self.dump_state_to_file(game_state);
            }
            if let Some(path) = &self.last_dump {
                ui.weak(path);
            }
        });
        ui.separator();

        let mut running = false;
        if let Some(gm) = tas_runner {
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

            // The checkpoint list is generated from the route definition (the
            // route is built once just to walk its `SeqCheckpoint`s), so the
            // picker always matches the actual route.
            if self.checkpoints.is_empty() {
                self.checkpoints = tas::create_tas().checkpoints();
            }
            egui::ComboBox::from_label("Checkpoint")
                .selected_text(
                    self.checkpoint
                        .clone()
                        .unwrap_or(String::from("New Game"))
                        .to_string(),
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.checkpoint,
                        Some("New Game".to_string()),
                        "New Game",
                    );
                    for checkpoint in &self.checkpoints {
                        ui.selectable_value(
                            &mut self.checkpoint,
                            Some(checkpoint.clone()),
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
                    *tas_runner = Some(tas::create_load_sequence(
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
                *tas_runner = Some(gm);
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
                *tas_runner = Some(gm);
                self.countdown = Some(1.0);
            }
            // Debug: runs only the campfire cooking interaction. Stand near the
            // Mountain Trail campfire (berries picked) before clicking.
            if ui
                .add_enabled(!running, egui::Button::new("Start Cook Test"))
                .clicked()
            {
                info!(
                    "Reproduce this run with: {}",
                    repro_command(Route::Cook, None, None, false, &game_state.config)
                );
                let gm = tas::create_cook_test();
                *tas_runner = Some(gm);
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
                *tas_runner = Some(gm);
                self.countdown = Some(1.0);
            }
        }

        if let Some(gm) = tas_runner {
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
            let combat = tas_runner.as_ref().and_then(|gm| gm.combat_controller());
            self.draw_appraisals(game_state, combat, ui);
            self.draw_moves(game_state, ui);
        } else if lum.active {
            self.draw_level_up(game_state, ui);
        } else if tsmd.active {
            self.draw_title(game_state, ui);
        }
    }
}
