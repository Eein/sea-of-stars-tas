#![allow(dead_code)]

pub mod appraisal;
mod controllers;
pub mod damage;
mod skills;

use joystick::common::JoystickBtnInterface;

use crate::combat::appraisal::Appraisal;
use crate::control::SosAction;
use crate::memory::combat_manager::CombatControllerType;
use crate::seq::button::ButtonPress;
use crate::state::GameState;

// The following imports are only used by the old WIP combat FSM below, kept
// commented out for reference:
// use data::prelude::PlayerPartyCharacter;
// use skills::{BasicAttack, Skill};
// use crate::combat::controllers::basic_encounter_controller::BasicEncounterController;
// use crate::combat::controllers::EncounterController;
// use crate::memory::combat_manager::CombatDamageType;

// Old WIP combat state machine, kept for reference:
// #[derive(Debug)]
// enum CombatFsm {
//     Idle,
//     Dialog,
//     Action,
//     Blocking,
//     Consideration,
//     Appraisal,
// }

/// Per-turn executor state for a normal (`Basic`) encounter.
///
/// We have closed-loop signals only for turn ownership and the timed-hit window
/// (`timed_attack_ready`); the battle-command highlight and the enemy-target
/// cursor are NOT in memory yet, so the two menu confirms are open-loop (we
/// assume Attack is the default command and accept the default-highlighted
/// target). Landing the timed hit is the closed-loop part. Targeting the
/// appraisal's *chosen* enemy is a seam for when the cursor offset is mapped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TurnFsm {
    /// No player owns the ring; wait.
    Idle,
    /// Navigate the battle-command ring to the chosen action's command (closed-
    /// loop on `battle_command_index`) and confirm it.
    SelectCommand,
    /// Navigate the skill/combo submenu to the chosen combo and confirm it.
    SelectComboAbility,
    /// Move the enemy cursor onto the chosen appraisal's target (closed-loop on
    /// `selected_attack_target_guid`), bailing to the default after a bounded
    /// number of taps. Only active once we've left the command ring.
    SelectTarget,
    /// Press Confirm to commit the attack on the cursor's current target.
    ConfirmTarget,
    /// Attack committed. Watch `timed_attack_ready` for the whole animation and
    /// tap Confirm on each window edge, until the turn ends (index changes).
    Attacking,
}

pub struct CombatManager {
    btn: ButtonPress,
    /// Controller index we were mashing on last frame, so we can release it if
    /// the active player changes mid-mash.
    last_gamepad: Option<usize>,
    /// Duty-cycle timer for mashing Confirm on all pads to dismiss mid-fight
    /// dialogue.
    dialog_timer: f64,
    /// Ranked candidate actions for the current combat state, recomputed every
    /// frame.
    pub appraisals: Vec<Appraisal>,
    /// The top-ranked appraisal, if any.
    pub chosen: Option<Appraisal>,
    /// Per-turn executor state (normal encounters only).
    turn_fsm: TurnFsm,
    /// The `current_player_index` we're currently acting on, so we can detect
    /// when the turn passes to another actor and reset the FSM.
    turn_index: Option<i32>,
    /// Time spent in the current `turn_fsm` state, for settle delays and the
    /// anti-hang watchdogs.
    state_timer: f64,
    /// `timed_attack_ready` last frame, so we only act on the rising edge.
    last_timed_ready: bool,
    /// Cursor taps spent this turn trying to reach the chosen target, so we can
    /// bail to the default target instead of looping forever.
    target_taps: u32,
    /// Index into [`TARGET_DIRS`] for the current cursor-step direction. We
    /// advance it when a tap doesn't move the cursor (hit an edge / wrong axis).
    target_dir: usize,
    /// Cursor target at the last tap, to detect when a direction stops moving it.
    last_cursor: Option<String>,
    // Old WIP FSM state, kept for reference:
    // fsm: CombatFsm,
    // controller: Option<Box<dyn EncounterController>>,
    // action: Option<Skill>,
}

impl Default for CombatManager {
    fn default() -> Self {
        Self {
            btn: ButtonPress::default(),
            last_gamepad: None,
            dialog_timer: 0.0,
            appraisals: Vec::new(),
            chosen: None,
            turn_fsm: TurnFsm::Idle,
            turn_index: None,
            state_timer: 0.0,
            last_timed_ready: false,
            target_taps: 0,
            target_dir: 0,
            last_cursor: None,
            // Old WIP FSM state, kept for reference:
            // fsm: CombatFsm::Idle,
            // controller: None,
            // action: None,
        }
    }
}

impl CombatManager {
    pub fn update(&mut self, state: &mut GameState, dt: f64) -> bool {
        let encounter_active = state.memory_managers.combat_manager.data.encounter_active;

        // Decision layer: rank candidate actions for the current state. The
        // executor acts on the active player's turn (normal encounters); the
        // GUI also surfaces the ranking.
        self.appraise(state);

        // Mid-fight dialogue isn't tied to a player's turn, so mash Confirm on
        // every controller to dismiss it.
        if state.memory_managers.new_dialog_manager.data.dialog_visible {
            self.mash_all_confirm(state, dt);
            // Force a fresh per-player mash / turn once dialogue clears.
            self.last_gamepad = None;
            self.reset_turn(TurnFsm::Idle);
            return !encounter_active;
        }
        self.dialog_timer = 0.0;

        // Only the normal encounter controller drives the appraisal executor.
        // Tutorials and scripted fights (FirstEncounter, LiveManaTutorial, ...)
        // still ride the proven Confirm-mash until they're modelled explicitly.
        let controller_type = &state
            .memory_managers
            .combat_manager
            .data
            .combat_controller_type;
        if matches!(controller_type, CombatControllerType::Basic) {
            self.execute_turn(state, dt);
        } else {
            self.reset_turn(TurnFsm::Idle);
            self.mash_turn(state, dt);
        }

        // Done once the encounter ends.
        !encounter_active
    }

    /// Recompute the ranked appraisals and the chosen action from the current
    /// combat snapshot. `chosen` is the top *executable* appraisal.
    fn appraise(&mut self, state: &GameState) {
        let cmd = &state.memory_managers.combat_manager.data;
        self.appraisals = appraisal::generate_appraisals(cmd);
        self.chosen = self
            .appraisals
            .iter()
            .find(|a| a.action.is_executable())
            .cloned();
    }

    /// Reset the per-turn executor to a known state and drop any turn tracking.
    fn reset_turn(&mut self, fsm: TurnFsm) {
        self.turn_fsm = fsm;
        self.turn_index = None;
        self.state_timer = 0.0;
        self.last_timed_ready = false;
        self.target_taps = 0;
        self.target_dir = 0;
        self.last_cursor = None;
    }

    /// Mash Confirm on the controller of the player whose turn it is.
    ///
    /// Re-read every frame, since the active player changes turn-to-turn. If the
    /// encounter-players manager isn't active, assume a single player on
    /// controller 0. This is the fallback path for fights the executor doesn't
    /// model yet.
    fn mash_turn(&mut self, state: &mut GameState, dt: f64) {
        let gamepad = Self::active_gamepad(state);

        // If the active player changed, release every controller and restart the
        // mash so we never leave a button held on the previous one.
        if self.last_gamepad != Some(gamepad) {
            state.release_all();
            self.btn = Self::mash_press();
            self.last_gamepad = Some(gamepad);
        }

        if self.btn.update(&mut state.gamepads[gamepad], dt) {
            self.btn = Self::mash_press();
        }
    }

    /// Drive the active player's turn: select the chosen action's command
    /// (Attack, or Combo + its submenu ability), move the cursor onto the chosen
    /// enemy, commit, then hold the charge through the attack. See [`TurnFsm`].
    fn execute_turn(&mut self, state: &mut GameState, dt: f64) {
        /// Settle delay before the executor first acts on a new turn, so the
        /// command ring is up before we press.
        const TURN_SETTLE: f64 = 0.10;
        /// If the post-commit wait sees no timed-window activity for this long,
        /// the confirms likely desynced — mash to force the turn along.
        const STUCK_TIMEOUT: f64 = 6.0;
        /// Cursor taps to spend chasing the chosen target before committing on
        /// whatever the cursor lands on (mirrors the Python bot's bail-out).
        const MAX_TARGET_TAPS: u32 = 16;
        /// Directions to step the enemy cursor. Enemies can be laid out 2D, so
        /// we cycle through these when a direction stops moving the cursor.
        const TARGET_DIRS: [SosAction; 4] = [
            SosAction::MenuRight,
            SosAction::MenuDown,
            SosAction::MenuLeft,
            SosAction::MenuUp,
        ];

        let gamepad = Self::active_gamepad(state);
        let cmd = &state.memory_managers.combat_manager.data;
        let has_control = cmd.selected_character.is_some();
        // The timed-hit window opens during the attack *animation*, after the
        // command is confirmed — at which point the attacker is no longer
        // `selected`. So don't filter by `selected`: only the acting player has
        // `timed_attack_ready` set during their hit.
        let timed_ready = cmd.players.items.iter().any(|p| p.timed_attack_ready);
        // UUID currently under the target cursor.
        let cursor_target = cmd.selected_attack_target_guid.clone();
        let turn = state
            .memory_managers
            .encounter_players_manager
            .data
            .current_player_index;
        // The enemy the appraisal wants to hit, if any.
        let want_target = self.chosen.as_ref().map(|a| a.target_enemy_id.clone());
        // The chosen action's command ring index (Attack=0, Combo=2, ...) and,
        // for combos, the combo's name to find in the submenu.
        let want_command = self
            .chosen
            .as_ref()
            .map(|a| a.action.battle_command_index())
            .unwrap_or(0);
        let want_combo = self
            .chosen
            .as_ref()
            .and_then(|a| a.action.combo_name().map(str::to_string));
        // Battle-command ring state (Attack=0, Skill=1, Combo=2, Item=3).
        let command_focus = cmd.battle_command_has_focus;
        let command_index = cmd.battle_command_index;
        // Skill/combo submenu focus.
        let skill_focus = cmd.skill_command_has_focus;
        // The combo highlighted in the combo submenu (matched against want_combo)
        // and whether it's castable right now.
        let highlighted_combo = cmd.highlighted_combo_id.clone();
        let highlighted_castable = cmd.highlighted_combo_castable;
        // The combo submenu is open iff the selector's items are combos (i.e. a
        // combo is highlighted). This is how a state knows it's in the submenu
        // vs the top-level command ring (both keep battle focus).
        let in_combo_submenu = highlighted_combo.is_some();

        // Turn boundary: a different active index means a fresh actor. Reset and
        // let the settle delay elapse before the first press.
        if turn != self.turn_index {
            state.release_all();
            self.turn_index = turn;
            self.state_timer = 0.0;
            self.last_timed_ready = false;
            self.target_taps = 0;
            self.target_dir = 0;
            self.last_cursor = None;
            self.turn_fsm = TurnFsm::Idle;
            self.last_gamepad = Some(gamepad);
        }

        self.state_timer += dt;

        match self.turn_fsm {
            TurnFsm::Idle => {
                // Wait until the command ring actually has focus before acting,
                // so our confirm lands on the menu instead of being lost early.
                if has_control && command_focus && self.state_timer >= TURN_SETTLE {
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::SelectCommand;
                }
            }
            TurnFsm::SelectCommand => {
                // Drive any in-flight nav/confirm press first.
                if !self.btn.done() {
                    self.btn.update(&mut state.gamepads[gamepad], dt);
                } else if in_combo_submenu {
                    // We're actually in the combo submenu, not the ring — back
                    // out so we can navigate the top-level commands.
                    self.btn = Self::cancel_press();
                    self.state_timer = 0.0;
                } else if command_index == Some(want_command) {
                    // Desired command highlighted — confirm it. Combos open a
                    // submenu; everything else goes straight to target select.
                    self.btn = Self::confirm_press();
                    self.state_timer = 0.0;
                    self.turn_fsm = if want_combo.is_some() {
                        TurnFsm::SelectComboAbility
                    } else {
                        TurnFsm::SelectTarget
                    };
                } else {
                    // Step the command cursor toward the desired command.
                    self.btn = Self::tap_press(SosAction::MenuDown);
                }
            }
            TurnFsm::SelectComboAbility => {
                // The combo submenu reuses the *battle* selector (battle focus
                // stays true, skill focus doesn't). Navigate until the highlighted
                // combo's id matches the chosen one, then confirm. Once battle
                // focus drops we've entered target select. Back out (Cancel) if we
                // no longer want a combo, the chosen combo isn't castable, or we
                // can't find it — so a dead-character/uncastable combo can't hang.
                let on_combo = match (&want_combo, &highlighted_combo) {
                    (Some(want), Some(have)) => want == have,
                    _ => false,
                };
                if !self.btn.done() {
                    self.btn.update(&mut state.gamepads[gamepad], dt);
                } else if !command_focus {
                    // Combo confirmed — we're in target select now.
                    self.state_timer = 0.0;
                    self.target_taps = 0; // reset the tap budget for targeting
                    self.turn_fsm = TurnFsm::SelectTarget;
                } else if !in_combo_submenu
                    || want_combo.is_none()
                    || self.target_taps >= MAX_TARGET_TAPS
                    || (on_combo && !highlighted_castable)
                {
                    // We shouldn't be here, don't want a combo, can't find it, or
                    // the chosen combo isn't castable — hand off to SelectCommand,
                    // which owns backing out of the submenu.
                    self.state_timer = 0.0;
                    self.target_taps = 0;
                    self.turn_fsm = TurnFsm::SelectCommand;
                } else if on_combo {
                    self.btn = Self::confirm_press();
                    self.state_timer = 0.0;
                } else {
                    self.btn = Self::tap_press(SosAction::MenuDown);
                    self.target_taps += 1;
                }
            }
            TurnFsm::SelectTarget => {
                // Finish any in-flight press (the command/combo-confirm or a tap).
                // We're only really in target select once we've left every menu;
                // otherwise recover to the menu we're actually in.
                if !self.btn.done() {
                    self.btn.update(&mut state.gamepads[gamepad], dt);
                } else if in_combo_submenu || skill_focus {
                    // Still in the combo/skill submenu — handle it there.
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::SelectComboAbility;
                } else if command_focus {
                    // Still on the command ring — the confirm didn't take; retry.
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::SelectCommand;
                } else {
                    // Commit when the cursor is on the chosen enemy, when we have
                    // no preference, or after exhausting the tap budget (bail to
                    // the default target rather than loop forever).
                    let on_target = match (&want_target, &cursor_target) {
                        (Some(want), Some(have)) => want == have,
                        (None, _) => true,
                        _ => false,
                    };
                    if on_target || self.target_taps >= MAX_TARGET_TAPS {
                        self.btn = Self::confirm_press();
                        self.state_timer = 0.0;
                        self.turn_fsm = TurnFsm::ConfirmTarget;
                    } else {
                        // If the previous tap didn't move the cursor, that axis
                        // is exhausted (edge / wrong direction) — rotate.
                        if cursor_target == self.last_cursor {
                            self.target_dir = (self.target_dir + 1) % TARGET_DIRS.len();
                        }
                        self.last_cursor = cursor_target.clone();
                        self.btn = Self::tap_press(TARGET_DIRS[self.target_dir]);
                        self.target_taps += 1;
                    }
                }
            }
            TurnFsm::ConfirmTarget => {
                if self.btn.update(&mut state.gamepads[gamepad], dt) {
                    state.gamepads[gamepad].release(&SosAction::Confirm);
                    self.state_timer = 0.0;
                    // Arm the edge detector so the first window fires.
                    self.last_timed_ready = false;
                    self.turn_fsm = TurnFsm::Attacking;
                }
            }
            TurnFsm::Attacking => {
                // Check for the command ring returning FIRST: the attack is over
                // and it's a fresh action. Release (dropping any held charge so it
                // can't auto-confirm the menu) and go select the next command.
                if command_focus {
                    state.gamepads[gamepad].release(&SosAction::Confirm);
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::SelectCommand;
                } else if timed_ready {
                    // Charge-attack timing (per the Python TAS): the RELEASE the
                    // instant the flag fires is the timed input.
                    state.gamepads[gamepad].release(&SosAction::Confirm);
                    self.state_timer = 0.0; // window activity = progress
                } else {
                    // Hold Confirm to charge while waiting for the window.
                    state.gamepads[gamepad].press(&SosAction::Confirm);
                    if self.state_timer >= STUCK_TIMEOUT {
                        // Only after a genuine stall (no window activity) do we
                        // mash to force the turn along.
                        self.mash_turn(state, dt);
                    }
                }
                self.last_timed_ready = timed_ready;
            }
        }
    }

    /// Tap Confirm on every controller in a duty cycle to dismiss mid-fight
    /// dialogue, regardless of which player owns the prompt.
    fn mash_all_confirm(&mut self, state: &mut GameState, dt: f64) {
        const PRESS: f64 = 0.1;
        const CYCLE: f64 = 0.3;
        self.dialog_timer += dt;
        if self.dialog_timer >= CYCLE {
            self.dialog_timer = 0.0;
        }
        let pressing = self.dialog_timer < PRESS;
        for gamepad in state.gamepads.iter_mut() {
            if pressing {
                gamepad.press(&SosAction::Confirm);
            } else {
                gamepad.release(&SosAction::Confirm);
            }
        }
    }

    /// A single Confirm tap used for mashing.
    fn mash_press() -> ButtonPress {
        ButtonPress {
            action: SosAction::Confirm,
            press_time: 0.1,
            release_time: 0.2,
            ..Default::default()
        }
    }

    /// A menu Confirm with a short settle gap before the next press, used to
    /// step through the command ring and target select.
    fn confirm_press() -> ButtonPress {
        ButtonPress {
            action: SosAction::Confirm,
            press_time: 0.04,
            release_time: 0.10,
            ..Default::default()
        }
    }

    /// A Cancel (B) tap to back out of a menu (e.g. an uncastable combo submenu).
    fn cancel_press() -> ButtonPress {
        ButtonPress {
            action: SosAction::Cancel,
            press_time: 0.04,
            release_time: 0.12,
            ..Default::default()
        }
    }

    /// A directional tap with a short settle gap, used to step the command/enemy
    /// cursor. The release gap lets the cursor move and memory update before we
    /// re-check.
    fn tap_press(action: SosAction) -> ButtonPress {
        ButtonPress {
            action,
            press_time: 0.04,
            release_time: 0.12,
            ..Default::default()
        }
    }

    /// Index of the controller to send combat input to.
    ///
    /// Controllers are mapped by *position* in the single-player-plus players
    /// list: the co-op join (`SeqMove` / `Move::Join`) binds `gamepads[i]` to
    /// `players.items[i]`. The active player's `index` field is a different
    /// numbering, so we find the list slot whose player matches the active one.
    /// Falls back to controller 0 when there's no active player or no match
    /// (single-player assumption).
    fn active_gamepad(state: &GameState) -> usize {
        let epmd = &state.memory_managers.encounter_players_manager.data;
        let Some(current_index) = epmd.current_player_index else {
            return 0;
        };

        let players = &state
            .memory_managers
            .single_player_plus_manager
            .data
            .players
            .items;
        if let Some(slot) = players.iter().position(|p| p.index == current_index)
            && slot < state.gamepads.len()
        {
            return slot;
        }
        0
    }

    // Old WIP combat behaviour (utility-AI action selection), kept for reference:
    //
    // pub fn update(&mut self, state: &mut GameState, dt: f64) -> bool {
    //     let combat_manager = &state.memory_managers.combat_manager.data;
    //
    //     if self.controller.is_none() {
    //         self.controller = Self::encounter_factory(state)
    //     }
    //
    //     // Execute Different states if controller is active
    //     if let Some(ref controller) = self.controller {
    //         // Always right before fsm - we never want to deal with dialog
    //         if controller.execute_dialog(state) {
    //             self.fsm = CombatFsm::Dialog
    //         }
    //     } else {
    //         // Otherwise set idle and wait for a controller
    //         self.fsm = CombatFsm::Idle
    //     }
    //
    //     match self.fsm {
    //         // intended to wait for acceptable parameters
    //         CombatFsm::Idle => {
    //             if self.action.is_none() && combat_manager.selected_character.is_some() {
    //                 println!("setting action");
    //                 self.fsm = CombatFsm::Action;
    //             }
    //         }
    //         CombatFsm::Dialog => {
    //             // this may not be needed - can probably handle this outside of combat
    //             // by delegating control to whoever can deal with this.
    //             // TODO(eein): button press here
    //         }
    //         CombatFsm::Action => {
    //             // Generate Action from Utility and select
    //             println!("action");
    //             self.action = Some(Skill {
    //                 character: PlayerPartyCharacter::Zale,
    //                 internal_name: "ZaleBasicAttack",
    //                 timing_type: skills::TimingType::MultiHit,
    //                 target_type: skills::TargetType::Enemy,
    //                 resource: skills::SkillResource::None,
    //                 damage_types: [CombatDamageType::Sword].to_vec(),
    //                 battle_command: skills::BattleCommand::Attack,
    //                 timing_controller: Box::new(BasicAttack),
    //                 cost: 0,
    //             });
    //
    //             if self.action.is_some() {
    //                 self.fsm = CombatFsm::Consideration;
    //             }
    //         }
    //         CombatFsm::Blocking => {
    //             // Blocking Behaviour
    //         }
    //         CombatFsm::Consideration => {
    //             // Execute Consideration from action (selection)
    //             if let Some(action) = &self.action {
    //                 if combat_manager.selected_character != Some(action.character.clone()) {
    //                     // press right if consideration is invalid
    //                     if self.btn.update(&mut state.gamepads[0], dt) {
    //                         self.btn = ButtonPress {
    //                             action: SosAction::MenuRight,
    //                             press_time: 0.05,
    //                             release_time: 0.05,
    //                             ..Default::default()
    //                         }
    //                     }
    //                 } else {
    //                     self.fsm = CombatFsm::Appraisal;
    //                 }
    //             }
    //         }
    //         CombatFsm::Appraisal => {
    //             // Execute Appraisal
    //         }
    //     }
    //
    //     !combat_manager.encounter_active
    // }
    //
    // fn encounter_factory(state: &GameState) -> Option<Box<dyn EncounterController>> {
    //     match &state
    //         .memory_managers
    //         .combat_manager
    //         .data
    //         .combat_controller_type
    //     {
    //         CombatControllerType::Basic => Some(Box::new(BasicEncounterController::default())),
    //         _ => Some(Box::new(BasicEncounterController::default())),
    //     }
    // }
}
