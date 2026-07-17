//! The per-turn state machine for normal (`Basic`) encounters: who acts, which
//! action is latched, and how it's driven through its steps.

use data::prelude::PlayerPartyCharacter;
use joystick::prelude::GenericJoystick;

use super::appraisal::{self, Appraisal};
use super::controller::CombatController;
use super::skills::{self, Action, ActionStep, StepScratch};
use crate::control::SosAction;
use crate::memory::combat_manager::CombatManagerData;
use crate::state::GameState;

/// Per-turn executor state. One value of this enum is the whole story of what
/// the executor is doing right now: each variant carries exactly the
/// bookkeeping its phase needs, so entering a state starts with fresh scratch
/// by construction — there are no shared counters to remember to reset.
///
/// We have closed-loop signals only for turn ownership and the timed-hit window
/// (`timed_attack_ready`); the battle-command highlight and the enemy-target
/// cursor are NOT in memory yet, so the two menu confirms are open-loop (we
/// assume Attack is the default command and accept the default-highlighted
/// target). Landing the timed hit is the closed-loop part. Targeting the
/// appraisal's *chosen* enemy is a seam for when the cursor offset is mapped.
pub(super) enum TurnState {
    /// No player owns the command ring (enemy acting, animation playing, or the
    /// ring hasn't appeared yet); wait. `elapsed` is time spent waiting, for
    /// the settle delay before the first press of a turn.
    Idle { elapsed: f64 },
    /// Tap Right on the command ring (which cycles party members) until the
    /// chosen action's attacker is the selected character. `swaps` counts taps
    /// so we give up (act as whoever's up) instead of looping forever.
    SelectingCharacter { swaps: u32 },
    /// Driving a latched action through its [`ActionStep`]s (command ring →
    /// submenu → target → timed hit). Holding the action *inside* the state is
    /// the latch: this state owns its action until it resolves, never re-reading
    /// the live `chosen` (which can flicker mid-animation and must not interrupt
    /// an in-progress hold like Sunball's charge).
    Acting(ActingState),
}

impl TurnState {
    /// A fresh Idle with the settle timer at zero.
    pub(super) fn idle() -> Self {
        TurnState::Idle { elapsed: 0.0 }
    }

    /// One-line summary of where the executor is, for the GUI/debugging.
    pub fn status(&self) -> String {
        match self {
            TurnState::Idle { elapsed } => format!("Idle ({elapsed:.1}s)"),
            TurnState::SelectingCharacter { swaps } => {
                format!("SelectingCharacter (swaps {swaps})")
            }
            TurnState::Acting(acting) => format!(
                "Acting({} @ {:?}, step timer {:.1}s)",
                acting.action.internal_name(),
                acting.step,
                acting.scratch.timer,
            ),
        }
    }
}

impl Default for TurnState {
    fn default() -> Self {
        Self::idle()
    }
}

/// The action [`TurnState::Acting`] is driving, with everything it needs to
/// finish: the action itself, who to hit, where it is in its steps, and the
/// per-step scratch the steps share.
pub(super) struct ActingState {
    action: Box<dyn Action>,
    /// The `unique_id` of the enemy this action targets.
    target: String,
    /// Total Live Mana charges the appraisal expects the attacker to hold —
    /// the [`Boosting`](ActionStep::Boosting) step absorbs up to this before
    /// the menus are touched.
    mana_charges: u32,
    step: ActionStep,
    scratch: StepScratch,
}

impl ActingState {
    /// Latch `appraisal` as the committed action, starting from the first step.
    fn latch(appraisal: &Appraisal) -> Self {
        Self {
            action: appraisal.to_action(),
            target: appraisal.target_enemy_id.clone(),
            mana_charges: appraisal.mana_charges,
            step: ActionStep::Boosting,
            scratch: StepScratch::default(),
        }
    }

    /// Latch `action` straight into its execution ([`Attacking`](ActionStep::Attacking))
    /// step, skipping the menus. Used to (re-)own an in-progress timed input — a
    /// Sunball charge — that's already on screen (there are no menus left to
    /// drive, and no target to pick since the cast is committed).
    fn executing(action: Box<dyn Action>) -> Self {
        Self {
            action,
            target: String::new(),
            mana_charges: 0,
            step: ActionStep::Attacking,
            scratch: StepScratch::default(),
        }
    }
}

/// The per-frame combat flags the turn logic branches on, copied out of the
/// memory snapshot up front so the game state can be mutably borrowed afterwards.
struct TurnSignals {
    /// At least one party member is "enabled", i.e. it's the party's turn. If
    /// none are, an enemy is acting (or an animation is playing) and we must
    /// stay out of the menus. Unlike the per-character `enabled`/`selected`
    /// flags — which flicker between members mid-turn — this is stable across
    /// a turn.
    our_turn: bool,
    /// A character is selected (we have menu control).
    has_control: bool,
    /// The battle-command ring has focus.
    command_focus: bool,
    /// An ability submenu (combo or skill) is open.
    in_submenu: bool,
    /// A target-select screen is up (an enemy cursor guid is readable).
    target_select_open: bool,
    /// Who currently owns the command ring.
    selected_character: Option<PlayerPartyCharacter>,
    /// Who the chosen action needs acting, if it's character-specific.
    want_character: Option<PlayerPartyCharacter>,
}

impl TurnSignals {
    fn read(cmd: &CombatManagerData, chosen: Option<&Appraisal>) -> Self {
        // Only *skills* are character-specific and need a party swap. Basic
        // attacks work for whoever's up. Combos are joint: with the current
        // two-character party (Zale + Valere) every combo is reachable from
        // either member's Combo menu, and `selected` flips between the two
        // participants mid-combo, so combos never request a swap. TODO: once a
        // third+ party member joins, a combo between two non-current members
        // *will* need a swap; key that off the combo's participants then.
        let want_character = match chosen.map(|a| &a.action) {
            Some(appraisal::CombatAction::Skill { .. }) => chosen.map(|a| a.attacker.clone()),
            _ => None,
        };
        Self {
            our_turn: cmd.players.items.iter().any(|p| p.enabled),
            has_control: cmd.selected_character.is_some(),
            command_focus: cmd.battle_command_has_focus,
            in_submenu: cmd.ability_submenu_open(),
            target_select_open: cmd.selected_attack_target_guid.is_some(),
            selected_character: cmd.selected_character.clone(),
            want_character,
        }
    }

    /// Whether the ring is on the character the chosen action wants (an action
    /// with no character preference counts as yes).
    fn on_wanted_character(&self) -> bool {
        match (&self.want_character, &self.selected_character) {
            (None, _) => true,
            (Some(want), Some(have)) => want == have,
            (Some(_), None) => false,
        }
    }
}

impl CombatController {
    /// Drive one frame of the active player's turn: select the chosen action's
    /// attacker, then drive the action through its steps (command ring →
    /// submenu → target → timed hit). The coarse gates (turn boundary, enemy
    /// turns, wrong-character preemption) live here; the per-state behaviour is
    /// one small handler per [`TurnState`] variant.
    ///
    /// A committed action in its *execution* phase (see
    /// [`acting_in_execution`](Self::acting_in_execution)) is exempt from those
    /// gates: its timed input (a Sunball charge) plays out during the
    /// special-move animation — after the menu turn has ended, when no player is
    /// `enabled` and the active index may move — exactly when the gates would
    /// otherwise `release_all` and abandon it. So we tick the action straight
    /// through until its own step machine says [`Done`](skills::StepOutcome::Done).
    pub(super) fn execute_turn(&mut self, state: &mut GameState, dt: f64) {
        let gamepad_idx = Self::active_gamepad(state);
        let signals = TurnSignals::read(
            &state.memory_managers.combat_manager.data,
            self.chosen.as_ref(),
        );
        let turn = state
            .memory_managers
            .encounter_players_manager
            .data
            .current_player_index;

        // A live charge QTE that nothing is executing needs an owner: it plays
        // out after the menu turn (so the gates below would just drop the hold),
        // and we may not have committed it through the menus at all (self-heal a
        // desync/restart mid-cast). Latch the charge action straight into its
        // execution step so its own FSM drives it to completion.
        let charge_live = state
            .memory_managers
            .combat_manager
            .data
            .sunball_charge
            .is_some();
        if charge_live
            && !self.acting_in_execution()
            && let Some(action) = skills::charge_action()
        {
            self.turn_state = TurnState::Acting(ActingState::executing(action));
        }

        // Gates apply only when we're *not* mid-execution of a committed action.
        if !self.acting_in_execution() {
            // Turn boundary: a different active index means a fresh actor. Reset
            // and let the settle delay elapse before the first press. A `None`
            // read is a transient blip, not a boundary — resetting on it would
            // abandon a half-driven menu (e.g. an open target select, which
            // Idle can't recover from).
            if turn.is_some() && turn != self.turn_index {
                state.release_all();
                self.turn_index = turn;
                self.last_gamepad = Some(gamepad_idx);
                self.turn_state = TurnState::idle();
            }

            // Not our turn: nobody is enabled, so an enemy is acting. Release any
            // held input, park in Idle, and wait — never drive a menu now.
            if !signals.our_turn {
                if !self.btn.done() {
                    self.btn.update(&mut state.gamepads[gamepad_idx], dt);
                } else {
                    state.release_all();
                }
                self.turn_state = TurnState::idle();
                return;
            }
        }

        let cmd = &state.memory_managers.combat_manager.data;
        let pad = &mut state.gamepads[gamepad_idx];

        // Character selection takes priority over any menu. While navigating the
        // command ring or an ability submenu, if the chosen action belongs to a
        // character who isn't the one selected, back out of any submenu with
        // Cancel and swap first — a move is only ever looked for in its own
        // character's menu.
        if self.must_switch_character(&signals) {
            if signals.in_submenu {
                // Stuck in a submenu for the wrong character — cancel out of it.
                if self.btn.done() {
                    self.btn = skills::cancel_press();
                } else {
                    self.btn.update(pad, dt);
                }
            }
            self.turn_state = TurnState::SelectingCharacter { swaps: 0 };
            return;
        }

        // Tick the current state; each handler returns the state for next frame.
        self.turn_state = match std::mem::take(&mut self.turn_state) {
            TurnState::Idle { elapsed } => self.idle(elapsed + dt, &signals, pad, dt),
            TurnState::SelectingCharacter { swaps } => {
                self.select_character(swaps, &signals, pad, dt)
            }
            TurnState::Acting(acting) => self.act(acting, cmd, pad, dt),
        };
    }

    /// Whether a committed action is in its execution phase — latched in
    /// [`TurnState::Acting`] on the target-confirm or attack step, where its
    /// timed input plays out. From `ConfirmingTarget` onward the cast is
    /// committed and the special-move animation can drop `our_turn`, so the
    /// action owns the inputs until its step machine resolves; the per-turn gates
    /// (turn boundary, not-our-turn) are skipped rather than abandon it.
    fn acting_in_execution(&self) -> bool {
        matches!(
            &self.turn_state,
            TurnState::Acting(acting) if matches!(
                acting.step,
                ActionStep::ConfirmingTarget | ActionStep::Attacking
            )
        )
    }

    /// Whether we're navigating menus (command ring / ability submenu) while
    /// the chosen action's attacker isn't the selected character, meaning we
    /// must swap characters before going any deeper.
    fn must_switch_character(&self, signals: &TurnSignals) -> bool {
        let in_menu_nav = matches!(
            &self.turn_state,
            TurnState::Acting(acting) if matches!(
                acting.step,
                // Boosting counts: charges absorb to the *selected* character,
                // so the swap must happen before any mana is spent on them.
                ActionStep::Boosting
                    | ActionStep::SelectingCommand
                    | ActionStep::SelectingAbility
            )
        );
        in_menu_nav && !signals.on_wanted_character()
    }

    /// [`TurnState::Idle`]: wait out the turn-start settle, recover from a
    /// stray ability submenu, and hand off to character selection once the
    /// command ring has focus.
    fn idle(
        &mut self,
        elapsed: f64,
        signals: &TurnSignals,
        pad: &mut GenericJoystick,
        dt: f64,
    ) -> TurnState {
        /// Settle delay before the executor first acts on a new turn, so the
        /// command ring is up before we press.
        const TURN_SETTLE: f64 = 0.10;

        if !self.btn.done() {
            self.btn.update(pad, dt);
            return TurnState::Idle { elapsed };
        }
        if signals.in_submenu {
            // We start (or a prior turn left us) inside an ability submenu: the
            // ring won't get focus, so we'd sit here forever. Cancel out first.
            self.btn = skills::cancel_press();
            return TurnState::idle();
        }
        if signals.has_control && !signals.command_focus && signals.target_select_open {
            // A dangling target-select screen (an aborted action left it up):
            // the ring can't regain focus until it's dismissed. Cancel out,
            // then re-plan from the ring as normal.
            self.btn = skills::cancel_press();
            return TurnState::idle();
        }
        if signals.has_control && signals.command_focus && elapsed >= TURN_SETTLE {
            // The command ring actually has focus, so our presses will land on
            // the menu instead of being lost early.
            return TurnState::SelectingCharacter { swaps: 0 };
        }
        TurnState::Idle { elapsed }
    }

    /// [`TurnState::SelectingCharacter`]: cycle party members on the command
    /// ring until the chosen action's attacker is selected (or the swap budget
    /// runs out), then latch the best action and start driving it.
    fn select_character(
        &mut self,
        swaps: u32,
        signals: &TurnSignals,
        pad: &mut GenericJoystick,
        dt: f64,
    ) -> TurnState {
        /// Character-swap taps to spend selecting the chosen attacker before
        /// giving up and acting as whoever's turn it currently is. A ring of a
        /// few party members wraps, so this only needs to cover the roster.
        const MAX_CHAR_SWAPS: u32 = 6;

        if !self.btn.done() {
            self.btn.update(pad, dt);
            return TurnState::SelectingCharacter { swaps };
        }
        if !signals.command_focus {
            // Not on the ring anymore — resync from Idle.
            return TurnState::idle();
        }
        if signals.on_wanted_character() || swaps >= MAX_CHAR_SWAPS {
            // On the chosen character (or we exhausted the swap budget and
            // proceed with whoever's up) — latch the current best action and
            // drive it from the top of its step machine. The live `chosen` is
            // not consulted again for the rest of the action (it can flicker
            // mid-animation).
            return match self.chosen.as_ref() {
                Some(chosen) => TurnState::Acting(ActingState::latch(chosen)),
                None => TurnState::idle(),
            };
        }
        // Party members are cycled with Left/Right on the command ring.
        self.btn = skills::tap_press(SosAction::MenuRight);
        TurnState::SelectingCharacter { swaps: swaps + 1 }
    }

    /// [`TurnState::Acting`]: drive the latched action's current step. Each
    /// `execute_*` reads live memory, presses at most one input, and returns a
    /// [`StepOutcome`](skills::StepOutcome) telling us to stay, advance, or
    /// drop the action.
    fn act(
        &mut self,
        mut acting: ActingState,
        cmd: &CombatManagerData,
        pad: &mut GenericJoystick,
        dt: f64,
    ) -> TurnState {
        acting.scratch.timer += dt;
        let mut ctx = skills::ActionCtx {
            cmd,
            gamepad: pad,
            btn: &mut self.btn,
            dt,
            // An empty target (the charge re-latch path) means "no
            // preference" — the cursor driver accepts whatever it's on.
            want_target: (!acting.target.is_empty()).then_some(acting.target.as_str()),
            want_mana_charges: acting.mana_charges,
            scratch: &mut acting.scratch,
        };
        let outcome = match acting.step {
            ActionStep::Boosting => acting.action.execute_boosting(&mut ctx),
            ActionStep::SelectingCommand => acting.action.execute_selecting_command(&mut ctx),
            ActionStep::SelectingAbility => acting.action.execute_selecting_ability(&mut ctx),
            ActionStep::SelectingTarget => acting.action.execute_selecting_target(&mut ctx),
            ActionStep::ConfirmingTarget => acting.action.execute_confirming_target(&mut ctx),
            ActionStep::Attacking => acting.action.execute_attacking(&mut ctx),
        };
        match outcome {
            skills::StepOutcome::Stay => TurnState::Acting(acting),
            skills::StepOutcome::Advance(step) => {
                acting.step = step;
                TurnState::Acting(acting)
            }
            // Resolved — drop the action and re-select the next one from the
            // live appraisal (via Idle → SelectingCharacter).
            skills::StepOutcome::Done => TurnState::idle(),
        }
    }
}
