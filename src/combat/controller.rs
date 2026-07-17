//! The combat controller: appraises the fight every frame and routes control
//! to the right driver — the turn executor for normal encounters, a Confirm
//! mash for dialogue and not-yet-modelled fights.
//!
//! Not to be confused with `memory::combat_manager`, which mirrors the *game's*
//! `CombatManager` singleton; this is the bot acting on that snapshot.

use super::appraisal::{self, Appraisal};
use super::skills;
use super::turn::TurnState;
use crate::control::SosAction;
use crate::memory::combat_manager::CombatControllerType;
use crate::seq::button::ButtonPress;
use crate::state::GameState;

/// What the controller does this frame, decided fresh from the snapshot every
/// frame — one mode, one driver, no overlap.
enum CombatMode {
    /// Mid-fight dialogue is up; mash Confirm on every pad to dismiss it
    /// (dialogue isn't tied to a player's turn).
    Dialogue,
    /// A normal encounter — run the appraisal-driven turn executor.
    Execute,
    /// A scripted/unmodelled fight — ride the proven Confirm-mash.
    Mash,
}

#[derive(Default)]
pub struct CombatController {
    pub(super) btn: ButtonPress,
    /// Ranked candidate actions for the current combat state, recomputed every
    /// frame.
    pub appraisals: Vec<Appraisal>,
    /// The top-ranked appraisal, if any.
    pub chosen: Option<Appraisal>,
    /// Per-turn executor state (normal encounters only). See [`TurnState`].
    pub(super) turn_state: TurnState,
    /// Last frame's delta, displayed in the GUI status — a dt above a tap's
    /// press window (0.04s) means timed presses are at risk.
    last_dt: f64,
    /// The `current_player_index` we're currently acting on, so we can detect
    /// when the turn passes to another actor and reset.
    pub(super) turn_index: Option<i32>,
}

impl CombatController {
    pub fn update(&mut self, state: &mut GameState, dt: f64) -> bool {
        self.last_dt = dt;
        let encounter_active = state.memory_managers.combat_manager.data.encounter_active;

        // Decision layer: rank candidate actions for the current state. The
        // executor acts on the active player's turn (normal encounters); the
        // GUI also surfaces the ranking.
        self.appraise(state);

        match Self::mode(state) {
            CombatMode::Dialogue => {
                self.mash_all_confirm(state, dt);
                // Force a fresh turn once dialogue clears.
                self.reset_turn();
            }
            CombatMode::Execute => {
                self.execute_turn(state, dt);
            }
            CombatMode::Mash => {
                self.reset_turn();
                self.mash_all_turn(state, dt);
            }
        }

        // Done once the encounter ends.
        !encounter_active
    }

    /// Pick this frame's [`CombatMode`]. Only the normal encounter controller
    /// drives the appraisal executor; tutorials and scripted fights
    /// (FirstEncounter, KidsCavernEncounter, ...) ride the Confirm-mash.
    /// KidsCavern additionally exposes no readable moves (its
    /// `allMoveDefinitions` is empty), so the executor has nothing to appraise
    /// there yet — mash until that's modelled.
    fn mode(state: &GameState) -> CombatMode {
        if state.memory_managers.new_dialog_manager.data.dialog_visible {
            return CombatMode::Dialogue;
        }
        let controller_type = &state
            .memory_managers
            .combat_manager
            .data
            .combat_controller_type;
        match controller_type {
            CombatControllerType::Basic => CombatMode::Execute,
            _ => CombatMode::Mash,
        }
    }

    /// Recompute the ranked appraisals and the chosen action from the current
    /// combat snapshot. `chosen` is the top *executable* appraisal.
    fn appraise(&mut self, state: &GameState) {
        let cmd = &state.memory_managers.combat_manager.data;
        self.appraisals = appraisal::generate_appraisals(cmd);
        // If the chosen action belongs to another character the executor
        // selects that character first (see `TurnState::SelectingCharacter`).
        self.chosen = appraisal::choose(&self.appraisals).cloned();
    }

    /// One-line executor status (turn state + latched action/step + the pad
    /// being driven) for the GUI.
    pub fn turn_status(&self, state: &GameState) -> String {
        format!(
            "{} | pad {} | dt {:.0}ms",
            self.turn_state.status(),
            Self::active_gamepad(state),
            self.last_dt * 1000.0,
        )
    }

    /// Park the executor in a fresh Idle and drop the turn tracking.
    pub(super) fn reset_turn(&mut self) {
        self.turn_state = TurnState::idle();
        self.turn_index = None;
    }

    /// Mash Confirm on *every* controller to force a scripted/unmodelled fight
    /// along. Unlike a per-player mash, this doesn't try to guess which pad owns
    /// the prompt: scripted encounters (KidsCavern, tutorials) don't route
    /// through `current_player_index`, and by these fights the party has several
    /// players, so a single-pad mash on the wrong controller stalls the fight.
    fn mash_all_turn(&mut self, state: &mut GameState, dt: f64) {
        if self.btn.update_all(&mut state.gamepads, dt) {
            self.btn = skills::mash_press();
        }
    }

    /// Tap Confirm on every controller in a duty cycle to dismiss mid-fight
    /// dialogue, regardless of which player owns the prompt.
    fn mash_all_confirm(&mut self, state: &mut GameState, dt: f64) {
        if self.btn.update_all(&mut state.gamepads, dt) {
            self.btn = ButtonPress {
                action: SosAction::Confirm,
                press_time: 0.1,
                release_time: 0.3,
                timer: 0.0,
            };
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
    pub(super) fn active_gamepad(state: &GameState) -> usize {
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
}
