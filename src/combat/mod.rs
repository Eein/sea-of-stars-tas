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
    /// Press Confirm to pick the (default-highlighted) Attack command.
    ConfirmAttack,
    /// Move the enemy cursor onto the chosen appraisal's target (closed-loop on
    /// `selected_attack_target_guid`), bailing to the default after a bounded
    /// number of taps.
    SelectTarget,
    /// Press Confirm to commit the attack on the cursor's current target.
    ConfirmTarget,
    /// Watch `timed_attack_ready` and tap Confirm on each rising edge.
    AwaitTimedHit,
    /// Action committed; wait for the turn to end (index changes).
    Done,
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
    /// `timed_attack_ready` last frame, so we only tap on the rising edge.
    last_timed_ready: bool,
    /// Whether a timed-hit Confirm press is currently in flight.
    timed_pressing: bool,
    /// Cursor taps spent this turn trying to reach the chosen target, so we can
    /// bail to the default target instead of looping forever.
    target_taps: u32,
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
            timed_pressing: false,
            target_taps: 0,
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

    /// Recompute the ranked appraisals and the chosen (top) action from the
    /// current combat snapshot.
    fn appraise(&mut self, state: &GameState) {
        let cmd = &state.memory_managers.combat_manager.data;
        self.appraisals = appraisal::generate_appraisals(cmd);
        self.chosen = self.appraisals.first().cloned();
    }

    /// Reset the per-turn executor to a known state and drop any turn tracking.
    fn reset_turn(&mut self, fsm: TurnFsm) {
        self.turn_fsm = fsm;
        self.turn_index = None;
        self.state_timer = 0.0;
        self.last_timed_ready = false;
        self.timed_pressing = false;
        self.target_taps = 0;
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

    /// Drive one basic attack for the active player: pick Attack, commit on the
    /// default target, then land the timed hit. See [`TurnFsm`] for the
    /// closed-/open-loop split.
    fn execute_turn(&mut self, state: &mut GameState, dt: f64) {
        /// Settle delay before the executor first acts on a new turn, so the
        /// command ring is up before we press.
        const TURN_SETTLE: f64 = 0.35;
        /// Give up waiting for a timed-hit window after this long (enemy died,
        /// no window, etc.) and consider the action done.
        const AWAIT_TIMEOUT: f64 = 4.0;
        /// If a turn never yields (open-loop confirms desynced), fall back to
        /// mashing so the fight can't hard-hang.
        const STUCK_TIMEOUT: f64 = 6.0;
        /// Cursor taps to spend chasing the chosen target before committing on
        /// whatever the cursor lands on (mirrors the Python bot's bail-out).
        const MAX_TARGET_TAPS: u32 = 12;

        let gamepad = Self::active_gamepad(state);
        let cmd = &state.memory_managers.combat_manager.data;
        let has_control = cmd.selected_character.is_some();
        let timed_ready = cmd
            .players
            .items
            .iter()
            .any(|p| p.selected && p.timed_attack_ready);
        // UUID currently under the target cursor (may be stale/None until the
        // offsets are verified live).
        let cursor_target = cmd.selected_attack_target_guid.clone();
        let turn = state
            .memory_managers
            .encounter_players_manager
            .data
            .current_player_index;
        // The enemy the appraisal wants to hit, if any.
        let want_target = self.chosen.as_ref().map(|a| a.target_enemy_id.clone());

        // Turn boundary: a different active index means a fresh actor. Reset and
        // let the settle delay elapse before the first press.
        if turn != self.turn_index {
            state.release_all();
            self.turn_index = turn;
            self.state_timer = 0.0;
            self.last_timed_ready = false;
            self.timed_pressing = false;
            self.target_taps = 0;
            self.turn_fsm = TurnFsm::Idle;
            self.last_gamepad = Some(gamepad);
        }

        self.state_timer += dt;

        match self.turn_fsm {
            TurnFsm::Idle => {
                // Wait for a player to own the ring, then begin after a settle.
                if has_control && turn.is_some() && self.state_timer >= TURN_SETTLE {
                    self.btn = Self::confirm_press();
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::ConfirmAttack;
                }
            }
            TurnFsm::ConfirmAttack => {
                if self.btn.update(&mut state.gamepads[gamepad], dt) {
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::SelectTarget;
                }
            }
            TurnFsm::SelectTarget => {
                // Drive any in-flight cursor tap to completion first.
                if !self.btn.done() {
                    self.btn.update(&mut state.gamepads[gamepad], dt);
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
                        // Step the cursor one enemy over and re-check next frame.
                        self.btn = Self::tap_press(SosAction::MenuRight);
                        self.target_taps += 1;
                    }
                }
            }
            TurnFsm::ConfirmTarget => {
                if self.btn.update(&mut state.gamepads[gamepad], dt) {
                    state.gamepads[gamepad].release(&SosAction::Confirm);
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::AwaitTimedHit;
                }
            }
            TurnFsm::AwaitTimedHit => {
                // Tap Confirm on each rising edge of the timed-hit window; this
                // naturally covers multi-hit basic attacks (one window per hit).
                if timed_ready && !self.last_timed_ready {
                    self.btn = Self::timed_press();
                    self.timed_pressing = true;
                }
                if self.timed_pressing {
                    if self.btn.update(&mut state.gamepads[gamepad], dt) {
                        state.gamepads[gamepad].release(&SosAction::Confirm);
                        self.timed_pressing = false;
                    }
                    self.state_timer = 0.0; // making progress; hold off the watchdog
                } else if self.state_timer >= AWAIT_TIMEOUT {
                    self.turn_fsm = TurnFsm::Done;
                    self.state_timer = 0.0;
                }
            }
            TurnFsm::Done => {
                state.gamepads[gamepad].release(&SosAction::Confirm);
                // Anti-hang: if the turn still hasn't passed to someone else, the
                // open-loop confirms likely desynced — mash to force progress.
                if self.state_timer >= STUCK_TIMEOUT {
                    self.mash_turn(state, dt);
                }
            }
        }

        self.last_timed_ready = timed_ready;
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

    /// A deliberate menu Confirm with a settle gap before the next press, used
    /// to step through the command ring and target select. Timings are
    /// conservative and meant to be tuned against the live game.
    fn confirm_press() -> ButtonPress {
        ButtonPress {
            action: SosAction::Confirm,
            press_time: 0.09,
            release_time: 0.35,
            ..Default::default()
        }
    }

    /// A crisp Confirm tap for landing a timed hit.
    fn timed_press() -> ButtonPress {
        ButtonPress {
            action: SosAction::Confirm,
            press_time: 0.07,
            release_time: 0.14,
            ..Default::default()
        }
    }

    /// A directional tap with a settle gap, used to step the enemy cursor. The
    /// release gap lets the cursor move and memory update before we re-check.
    fn tap_press(action: SosAction) -> ButtonPress {
        ButtonPress {
            action,
            press_time: 0.06,
            release_time: 0.28,
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
