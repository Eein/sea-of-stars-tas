#![allow(dead_code)]

pub mod appraisal;
mod controllers;
pub mod damage;
mod skills;

use joystick::common::JoystickBtnInterface;

use crate::combat::appraisal::Appraisal;
use crate::combat::skills::{Action, ActionStep};
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
    /// Swap the acting character (Left/Right on the command ring) to the chosen
    /// action's `attacker`, so a character-specific skill/combo is available.
    /// Closed-loop on `selected_character`; bounded so it can't loop forever.
    SelectCharacter,
    /// Driving the chosen action's own step machine (command ring → submenu →
    /// target → timed hit). The active [`ActionStep`] lives in
    /// [`CombatManager::action_step`]; the per-step logic is on the ability's
    /// `execute_*` methods (see `skills::Action`).
    Driving,
}

/// The action the executor has committed to for the current `Driving` cycle,
/// latched so it survives the per-frame `chosen` flicker. Without this, a charge
/// skill's hold is abandoned the moment `chosen` momentarily changes mid-animation
/// (which is what breaks Sunball). Latched when leaving `SelectCharacter`, dropped
/// when the action resolves.
struct CommittedAction {
    action: Box<dyn Action>,
    /// The `unique_id` of the enemy this action targets.
    target: String,
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
    /// Character-swap taps spent this turn trying to select the chosen action's
    /// attacker, so we give up (act as whoever's up) instead of looping forever.
    char_swaps: u32,
    /// Cursor taps spent this turn trying to reach the chosen target, so we can
    /// bail to the default target instead of looping forever.
    target_taps: u32,
    /// Index into [`TARGET_DIRS`] for the current cursor-step direction. We
    /// advance it when a tap doesn't move the cursor (hit an edge / wrong axis).
    target_dir: usize,
    /// Cursor target at the last tap, to detect when a direction stops moving it.
    last_cursor: Option<String>,
    /// The chosen action's current step in its own micro-FSM, advanced while
    /// `turn_fsm == Driving`. The per-step logic lives on the ability's
    /// `execute_*` methods; this just tracks where the action is.
    action_step: ActionStep,
    /// The action latched for the current `Driving` cycle (see [`CommittedAction`]).
    committed: Option<CommittedAction>,
    /// Last-logged appraisal summary, so the diagnostic logs only on change.
    debug_appraisal: Option<String>,
    /// Whether we've dumped the per-character move list once (debug diagnostic).
    debug_moves_dumped: bool,
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
            char_swaps: 0,
            target_taps: 0,
            target_dir: 0,
            last_cursor: None,
            action_step: ActionStep::SelectingCommand,
            committed: None,
            debug_appraisal: None,
            debug_moves_dumped: false,
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
        // The highest-scored executable action across the whole party (appraisals
        // are already sorted best-first). If it belongs to another character the
        // executor selects that character first (see the character-selection gate
        // in `execute_turn`).
        self.chosen = self
            .appraisals
            .iter()
            .find(|a| a.action.is_executable())
            .cloned();

        // Diagnostic: one-time dump of every character's move list, so we can see
        // whether combos are present in `allMoveDefinitions` and why they filter.
        if log::log_enabled!(log::Level::Debug) && !self.debug_moves_dumped && !cmd.moves.is_empty()
        {
            for cm in &cmd.moves {
                for m in &cm.moves {
                    log::debug!(
                        "move {:?}: {} cp={:?} mp={:?} loaded={} dmg={} unlockable={:?} {}",
                        cm.character,
                        m.move_id.as_deref().unwrap_or("?"),
                        m.combo_point_cost,
                        m.skill_point_cost,
                        m.loaded,
                        m.is_damaging,
                        m.unlockable,
                        m.learned_dbg.as_deref().unwrap_or("learned?=none"),
                    );
                }
            }
            self.debug_moves_dumped = true;
        }

        // Diagnostic: log the top candidates (and combo points) whenever the
        // ranking changes, so we can see what's being appraised and why.
        if log::log_enabled!(log::Level::Debug) {
            let summary = format!(
                "pts={} | top: [{}]",
                cmd.combo_points,
                self.appraisals
                    .iter()
                    .take(4)
                    .map(|a| format!("{} s{:.0}", a.action.label(), a.score))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            if self.debug_appraisal.as_deref() != Some(summary.as_str()) {
                log::debug!("appraise: {summary}");
                self.debug_appraisal = Some(summary);
            }
        }
    }

    /// Reset the per-turn executor to a known state and drop any turn tracking.
    fn reset_turn(&mut self, fsm: TurnFsm) {
        self.turn_fsm = fsm;
        self.turn_index = None;
        self.state_timer = 0.0;
        self.last_timed_ready = false;
        self.char_swaps = 0;
        self.target_taps = 0;
        self.target_dir = 0;
        self.last_cursor = None;
        self.action_step = ActionStep::SelectingCommand;
        self.committed = None;
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
        /// Character-swap taps to spend selecting the chosen attacker before
        /// giving up and acting as whoever's turn it currently is. A ring of a
        /// few party members wraps, so this only needs to cover the roster.
        const MAX_CHAR_SWAPS: u32 = 6;

        let gamepad = Self::active_gamepad(state);
        let cmd = &state.memory_managers.combat_manager.data;
        let has_control = cmd.selected_character.is_some();
        // It's our turn iff at least one party member is "enabled". If none are,
        // an enemy is acting (or an animation is playing) and we must stay out of
        // the menus. Unlike the per-character `enabled`/`selected` flags — which
        // flicker between members mid-turn — `any_enabled` is stable across a turn.
        let any_enabled = cmd.players.items.iter().any(|p| p.enabled);
        // Drive the turn from the live best action, recomputed every frame.
        let acting = self.chosen.as_ref();
        // Who currently owns the ring, and who the committed action needs acting.
        let selected_character = cmd.selected_character.clone();
        // Only *skills* are character-specific and need a party swap here. Basic
        // attacks work for whoever's up. Combos are joint: with the current
        // two-character party (Zale + Valere) every combo is reachable from
        // either member's Combo menu, and the two participants make `selected`
        // flip between them — chasing that flip is what wedged us. TODO: once a
        // third+ party member joins, a combo between two non-current members
        // *will* need a swap; key that off the combo's participants then.
        let want_character = match acting.map(|a| &a.action) {
            Some(appraisal::CombatAction::Skill { .. }) => acting.map(|a| a.attacker.clone()),
            _ => None,
        };
        // The timed-hit window opens during the attack *animation*, after the
        // command is confirmed — at which point the attacker is no longer
        // `selected`. So don't filter by `selected`: only the acting player has
        // `timed_attack_ready` set during their hit.
        let timed_ready = cmd.players.items.iter().any(|p| p.timed_attack_ready);
        let turn = state
            .memory_managers
            .encounter_players_manager
            .data
            .current_player_index;
        // Battle-command ring focus, and whether an ability submenu is open — the
        // controller only needs these for the Idle recovery and the char-swap
        // preemption; the rest of the menu state is read by the action's steps.
        let command_focus = cmd.battle_command_has_focus;
        let in_combo_submenu = cmd.highlighted_combo_id.is_some();
        let in_skill_submenu = cmd.skill_command_has_focus;

        // Turn boundary: a different active index means a fresh actor. Reset and
        // let the settle delay elapse before the first press.
        if turn != self.turn_index {
            state.release_all();
            self.turn_index = turn;
            self.state_timer = 0.0;
            self.last_timed_ready = false;
            self.char_swaps = 0;
            self.target_taps = 0;
            self.target_dir = 0;
            self.last_cursor = None;
            self.action_step = ActionStep::SelectingCommand;
            self.committed = None;
            self.turn_fsm = TurnFsm::Idle;
            self.last_gamepad = Some(gamepad);
        }

        self.state_timer += dt;

        // Not our turn: nobody is enabled, so an enemy is acting. Release any held
        // input, park in Idle, and wait — never drive a menu now.
        if !any_enabled {
            if !self.btn.done() {
                self.btn.update(&mut state.gamepads[gamepad], dt);
            } else {
                state.release_all();
            }
            self.turn_fsm = TurnFsm::Idle;
            self.state_timer = 0.0;
            self.last_timed_ready = timed_ready;
            return;
        }

        // Character selection takes priority over any menu. While navigating the
        // command ring or an ability submenu, if the committed action belongs to
        // a character who isn't the one selected, we're in the wrong place: back
        // out of any submenu with Cancel and return to SelectCharacter to swap
        // first. This keeps us from ever hunting a move in the wrong character's
        // menu (which is what wedged us before).
        let in_menu_nav = matches!(self.turn_fsm, TurnFsm::Driving)
            && matches!(
                self.action_step,
                ActionStep::SelectingCommand | ActionStep::SelectingSkill
            );
        if in_menu_nav
            && let Some(want) = &want_character
            && selected_character.as_ref() != Some(want)
        {
            if in_skill_submenu || in_combo_submenu {
                // Stuck in a submenu for the wrong character — cancel out of it.
                if self.btn.done() {
                    self.btn = skills::cancel_press();
                } else {
                    self.btn.update(&mut state.gamepads[gamepad], dt);
                }
            }
            self.char_swaps = 0;
            self.state_timer = 0.0;
            self.turn_fsm = TurnFsm::SelectCharacter;
            self.last_timed_ready = timed_ready;
            return;
        }

        match self.turn_fsm {
            TurnFsm::Idle => {
                // Recover if we start (or a prior turn leaves us) inside an
                // ability submenu: the command ring won't have focus, so we'd sit
                // here forever. Cancel back out to the ring first.
                if !self.btn.done() {
                    self.btn.update(&mut state.gamepads[gamepad], dt);
                } else if in_skill_submenu || in_combo_submenu {
                    self.btn = skills::cancel_press();
                    self.state_timer = 0.0;
                } else if has_control && command_focus && self.state_timer >= TURN_SETTLE {
                    // Wait until the command ring actually has focus before acting,
                    // so our confirm lands on the menu instead of being lost early.
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::SelectCharacter;
                }
            }
            TurnFsm::SelectCharacter => {
                // Swap the acting character to the chosen action's attacker. The
                // command ring cycles party members with Left/Right; the chosen
                // attacker is enabled, so it's reachable. Once selected (or we
                // have no preference / exhaust the swap budget), select the
                // command. Bail back to Idle if we somehow left the ring.
                let on_character = match (&want_character, &selected_character) {
                    (Some(want), Some(have)) => want == have,
                    (None, _) => true,
                    _ => false,
                };
                if !self.btn.done() {
                    self.btn.update(&mut state.gamepads[gamepad], dt);
                } else if !command_focus {
                    // Not on the ring anymore — resync from Idle.
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::Idle;
                } else if on_character || self.char_swaps >= MAX_CHAR_SWAPS {
                    // On the chosen character (or we exhausted the swap budget and
                    // proceed with whoever's up) — latch the current best action
                    // and start driving it from the top of its step machine. The
                    // latch is what keeps a charge from being abandoned when the
                    // live `chosen` flickers mid-animation.
                    self.committed = self.chosen.as_ref().map(|c| CommittedAction {
                        action: c.to_action(),
                        target: c.target_enemy_id.clone(),
                    });
                    self.state_timer = 0.0;
                    self.action_step = ActionStep::SelectingCommand;
                    self.turn_fsm = TurnFsm::Driving;
                } else {
                    // Party members are cycled with Left/Right on the command ring.
                    self.btn = skills::tap_press(SosAction::MenuRight);
                    self.char_swaps += 1;
                }
            }
            TurnFsm::Driving => {
                // Drive the chosen action's own micro-FSM one step. The action is
                // rebuilt from the live `chosen` each frame; its per-step logic
                // (ring nav → submenu → target → timed hit) lives on the ability's
                // `execute_*` methods, which press at most one input and return a
                // `StepOutcome` telling us to stay, advance, or drop the action.
                // Take the latched action out to drive it, so we can borrow the
                // controller's button/scratch mutably alongside it; it goes back
                // unless the action resolves.
                let Some(committed) = self.committed.take() else {
                    // Nothing latched — resync from Idle.
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::Idle;
                    self.last_timed_ready = timed_ready;
                    return;
                };
                let outcome = {
                    let mut ctx = skills::ActionCtx {
                        cmd: &state.memory_managers.combat_manager.data,
                        gamepad: &mut state.gamepads[gamepad],
                        btn: &mut self.btn,
                        dt,
                        want_target: Some(committed.target.as_str()),
                        state_timer: &mut self.state_timer,
                        target_taps: &mut self.target_taps,
                        target_dir: &mut self.target_dir,
                        last_cursor: &mut self.last_cursor,
                        last_timed_ready: &mut self.last_timed_ready,
                    };
                    match self.action_step {
                        ActionStep::SelectingCommand => {
                            committed.action.execute_selecting_command(&mut ctx)
                        }
                        ActionStep::SelectingSkill => {
                            committed.action.execute_selecting_skill(&mut ctx)
                        }
                        ActionStep::SelectingEnemySequence => {
                            committed.action.execute_selecting_enemy_sequence(&mut ctx)
                        }
                        ActionStep::ConfirmEnemySequence => {
                            committed.action.execute_confirm_enemy_sequence(&mut ctx)
                        }
                        ActionStep::TimingSequence => {
                            committed.action.execute_timing_sequence(&mut ctx)
                        }
                        // Boost/ConfirmCommand/ConfirmSkill are folded into their
                        // neighbouring steps today; ActionComplete resolves.
                        ActionStep::ActionComplete => skills::StepOutcome::Done,
                        _ => skills::StepOutcome::Stay,
                    }
                };
                match outcome {
                    skills::StepOutcome::Stay => self.committed = Some(committed),
                    skills::StepOutcome::Advance(next) => {
                        self.action_step = next;
                        self.committed = Some(committed);
                    }
                    skills::StepOutcome::Done => {
                        // Action resolved — drop the latch and re-select the next
                        // action from the live appraisal (via Idle → SelectCharacter).
                        self.action_step = ActionStep::SelectingCommand;
                        self.state_timer = 0.0;
                        self.committed = None;
                        self.turn_fsm = TurnFsm::Idle;
                    }
                }
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
