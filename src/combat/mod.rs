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
    /// Swap the acting character (Left/Right on the command ring) to the chosen
    /// action's `attacker`, so a character-specific skill/combo is available.
    /// Closed-loop on `selected_character`; bounded so it can't loop forever.
    SelectCharacter,
    /// Navigate the battle-command ring to the chosen action's command (closed-
    /// loop on `battle_command_index`) and confirm it.
    SelectCommand,
    /// Navigate the combo submenu (battle selector) to the chosen combo.
    SelectComboAbility,
    /// Navigate the skill submenu (skill selector) to the chosen skill.
    SelectSkillAbility,
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
    /// The timing type latched when the attack is committed (entering
    /// `Attacking`). `chosen` is live every frame and can flip mid-animation, so
    /// the attack phase must be driven from this stable copy — otherwise a charge
    /// skill's hold gets abandoned when `chosen` momentarily changes.
    attack_timing: Option<skills::TimingType>,
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
            attack_timing: None,
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
        // The best executable action across the whole party. If it belongs to
        // another character the executor selects that character first (see the
        // character-selection gate in `execute_turn`).
        // TEMP(slice2 RE): prefer a skill so the FSM opens the skill submenu.
        self.chosen = self
            .appraisals
            .iter()
            .find(|a| matches!(a.action, appraisal::CombatAction::Skill { .. }))
            .or_else(|| self.appraisals.iter().find(|a| a.action.is_executable()))
            .cloned();
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
        self.attack_timing = None;
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
        /// Character-swap taps to spend selecting the chosen attacker before
        /// giving up and acting as whoever's turn it currently is. A ring of a
        /// few party members wraps, so this only needs to cover the roster.
        const MAX_CHAR_SWAPS: u32 = 6;
        /// Grace period after confirming a Skill/Combo command before we treat an
        /// unopened submenu as "closed" and back out — the submenu takes a couple
        /// frames to appear, and bouncing early wedges us on the command ring.
        const SUBMENU_SETTLE: f64 = 0.30;
        /// Delay after committing a charge action before we start holding Confirm.
        /// The skill's animation has to reach its charge phase first — the caster
        /// often leaps to center screen before the charge opens — so holding too
        /// early gets the input eaten and the charge never builds.
        const CHARGE_SETTLE: f64 = 1.50;
        /// How long to hold Confirm to build the charge before releasing to fire.
        /// A charge skill isn't a timed hit: you enter the charge screen, hold to
        /// build it, then release — there's no `timed_attack_ready` window.
        const CHARGE_HOLD: f64 = 1.75;
        /// Grace after the charge releases before the mash safety net kicks in, to
        /// let the cast animation resolve on its own.
        const CHARGE_RESOLVE_GRACE: f64 = 4.0;
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
        // UUID currently under the target cursor.
        let cursor_target = cmd.selected_attack_target_guid.clone();
        let turn = state
            .memory_managers
            .encounter_players_manager
            .data
            .current_player_index;
        // The enemy the committed action wants to hit, if any.
        let want_target = acting.map(|a| a.target_enemy_id.clone());
        // The committed action's command ring index (Attack=0, Combo=2, ...) and,
        // for combos, the combo's name to find in the submenu.
        let want_command = acting.map(|a| a.action.battle_command_index()).unwrap_or(0);
        // Submenu ability name for combos/skills (None for basic attacks).
        let want_ability = acting.and_then(|a| a.action.ability_name().map(str::to_string));
        // How to drive the input during the attack. Basic attacks and combos are
        // one-hits by default; skills use their declared timing. A skill charge
        // (Sunball) is its own mechanic — hold-then-release with no timed window.
        let timing = match acting.map(|a| &a.action) {
            Some(appraisal::CombatAction::Skill { name, .. }) => {
                skills::skill_timing(name).unwrap_or(skills::TimingType::OneHit)
            }
            _ => skills::TimingType::OneHit,
        };
        // Battle-command ring state (Attack=0, Skill=1, Combo=2, Item=3).
        let command_focus = cmd.battle_command_has_focus;
        let command_index = cmd.battle_command_index;
        // Skill submenu focus (the skill submenu lives on the skill selector).
        let skill_focus = cmd.skill_command_has_focus;
        // The move highlighted in each ability submenu (matched against
        // want_ability) and whether it's castable. Combos use the battle selector
        // (battle focus stays true); skills use the skill selector.
        let highlighted_combo = cmd.highlighted_combo_id.clone();
        let combo_castable = cmd.highlighted_combo_castable;
        let highlighted_skill = cmd.highlighted_skill_id.clone();
        let skill_castable = cmd.highlighted_skill_castable;
        // A submenu is open iff its items are moves (a move is highlighted). This
        // is how a state knows it's in a submenu vs the top-level command ring.
        let in_combo_submenu = highlighted_combo.is_some();
        let in_skill_submenu = skill_focus;

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
            self.attack_timing = None;
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
        let in_menu_nav = matches!(
            self.turn_fsm,
            TurnFsm::SelectCommand | TurnFsm::SelectComboAbility | TurnFsm::SelectSkillAbility
        );
        if in_menu_nav
            && let Some(want) = &want_character
            && selected_character.as_ref() != Some(want)
        {
            if in_skill_submenu || in_combo_submenu {
                // Stuck in a submenu for the wrong character — cancel out of it.
                if self.btn.done() {
                    self.btn = Self::cancel_press();
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
                    self.btn = Self::cancel_press();
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
                } else if on_character {
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::SelectCommand;
                } else if self.char_swaps >= MAX_CHAR_SWAPS {
                    // Couldn't select the wanted character (e.g. a skill for a
                    // member who can't act this turn). Proceed with whoever's up;
                    // the live appraisal will settle on an action they can take.
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::SelectCommand;
                } else {
                    // Party members are cycled with Left/Right on the command ring.
                    self.btn = Self::tap_press(SosAction::MenuRight);
                    self.char_swaps += 1;
                }
            }
            TurnFsm::SelectCommand => {
                // Drive any in-flight nav/confirm press first.
                if !self.btn.done() {
                    self.btn.update(&mut state.gamepads[gamepad], dt);
                } else if in_combo_submenu || in_skill_submenu {
                    // We're actually in an ability submenu, not the ring — back
                    // out so we can navigate the top-level commands.
                    self.btn = Self::cancel_press();
                    self.state_timer = 0.0;
                } else if command_index == Some(want_command) {
                    // Desired command highlighted — confirm it. Skill/Combo open a
                    // submenu; everything else goes straight to target select.
                    self.btn = Self::confirm_press();
                    self.state_timer = 0.0;
                    self.turn_fsm = match (want_ability.is_some(), want_command) {
                        (true, 2) => TurnFsm::SelectComboAbility,
                        (true, _) => TurnFsm::SelectSkillAbility,
                        (false, _) => TurnFsm::SelectTarget,
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
                let on_combo = match (&want_ability, &highlighted_combo) {
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
                } else if !in_combo_submenu && self.state_timer < SUBMENU_SETTLE {
                    // Submenu still opening after the confirm — wait for it.
                } else if !in_combo_submenu
                    || want_ability.is_none()
                    || self.target_taps >= MAX_TARGET_TAPS
                    || (on_combo && !combo_castable)
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
            TurnFsm::SelectSkillAbility => {
                // The skill submenu lives on the *skill* selector (skill focus
                // true; battle focus drops). Navigate until the highlighted skill
                // matches the chosen one, then confirm. When skill focus drops
                // we've committed (→ target select). Back out if we shouldn't be
                // here, don't want a skill, can't find it, or it isn't castable.
                let on_skill = match (&want_ability, &highlighted_skill) {
                    (Some(want), Some(have)) => want == have,
                    _ => false,
                };
                if !self.btn.done() {
                    self.btn.update(&mut state.gamepads[gamepad], dt);
                } else if !in_skill_submenu && !command_focus {
                    // Off the ring with no skill submenu → the skill was confirmed;
                    // we're in target select now.
                    self.state_timer = 0.0;
                    self.target_taps = 0;
                    self.turn_fsm = TurnFsm::SelectTarget;
                } else if !in_skill_submenu && self.state_timer < SUBMENU_SETTLE {
                    // Submenu still opening after the confirm — wait for it.
                } else if !in_skill_submenu {
                    // Settled and still on the ring — the submenu didn't open;
                    // back out via command select.
                    self.state_timer = 0.0;
                    self.target_taps = 0;
                    self.turn_fsm = TurnFsm::SelectCommand;
                } else if want_ability.is_none()
                    || self.target_taps >= MAX_TARGET_TAPS
                    || (on_skill && !skill_castable)
                {
                    // Hand off to SelectCommand, which owns backing out.
                    self.state_timer = 0.0;
                    self.target_taps = 0;
                    self.turn_fsm = TurnFsm::SelectCommand;
                } else if on_skill {
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
                } else if in_skill_submenu {
                    // Still in the skill submenu — handle it there.
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::SelectSkillAbility;
                } else if in_combo_submenu {
                    // Still in the combo submenu — handle it there.
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
                    // Latch how to drive the attack now, before `chosen` can flip
                    // mid-animation and abandon (e.g.) a charge hold.
                    self.attack_timing = Some(timing);
                    self.turn_fsm = TurnFsm::Attacking;
                }
            }
            TurnFsm::Attacking => {
                // A menu returning means the action resolved — release (dropping
                // any held charge so it can't auto-confirm the menu) and pick the
                // next action.
                if command_focus || in_combo_submenu || in_skill_submenu {
                    state.gamepads[gamepad].release(&SosAction::Confirm);
                    self.state_timer = 0.0;
                    self.turn_fsm = TurnFsm::SelectCommand;
                } else {
                    // Drive the input per the timing latched when we committed —
                    // `chosen` (and thus the live `timing`) can flip mid-animation.
                    let attack_timing = self.attack_timing.unwrap_or(timing);
                    match attack_timing {
                        skills::TimingType::Charge => {
                            // Skill charge (Sunball): settle → hold to build the
                            // charge → release to fire. No timed window is involved;
                            // it's driven off the state timer alone.
                            let hold_end = CHARGE_SETTLE + CHARGE_HOLD;
                            if self.state_timer >= CHARGE_SETTLE && self.state_timer < hold_end {
                                state.gamepads[gamepad].press(&SosAction::Confirm);
                            } else {
                                state.gamepads[gamepad].release(&SosAction::Confirm);
                            }
                            // Safety net: if the cast still hasn't resolved a while
                            // after releasing, mash to force the turn along.
                            if self.state_timer >= hold_end + CHARGE_RESOLVE_GRACE {
                                self.mash_turn(state, dt);
                            }
                        }
                        skills::TimingType::OneHit | skills::TimingType::MultiHit => {
                            // Tap Confirm on each rising edge of the window.
                            state.gamepads[gamepad].release(&SosAction::Confirm);
                            if timed_ready && !self.last_timed_ready {
                                self.btn = Self::confirm_press();
                            }
                            self.btn.update(&mut state.gamepads[gamepad], dt);
                            if timed_ready {
                                self.state_timer = 0.0; // window activity = progress
                            } else if self.state_timer >= STUCK_TIMEOUT {
                                self.mash_turn(state, dt);
                            }
                        }
                        skills::TimingType::None => {
                            // No input; just wait for the action to resolve.
                            state.gamepads[gamepad].release(&SosAction::Confirm);
                            if self.state_timer >= STUCK_TIMEOUT {
                                self.mash_turn(state, dt);
                            }
                        }
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
