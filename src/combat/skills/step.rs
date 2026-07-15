//! The step machine's vocabulary: the steps an action moves through, the
//! outcome of driving one, and the context/scratch threaded into each step.

use joystick::prelude::GenericJoystick;

use super::menu;
use crate::memory::combat_manager::CombatManagerData;
use crate::seq::button::ButtonPress;

/// The menu-driving steps an action moves through on its turn, in order. The
/// executor calls the matching `Action::execute_*` for the current step every
/// frame until the step returns [`StepOutcome::Advance`] or resolves.
///
/// Steps the game has but we don't model yet (separate command/ability confirm
/// screens) are folded into these; add a variant when one is modelled
/// explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionStep {
    /// Absorbing Live Mana up to the appraised charge count: hold Boost on the
    /// command ring and tap Confirm once per charge (each merges 5 small mana),
    /// verified against the attacker's live `mana_charge_count`. Skips straight
    /// through when the appraisal wants no more charges than are held.
    Boosting,
    /// Navigating the battle-command ring (Attack/Skill/Combo/Item) to the
    /// action's command and confirming it.
    SelectingCommand,
    /// Navigating the skill/combo submenu to the desired ability and
    /// confirming it. Skipped by actions with no submenu (basic attacks).
    SelectingAbility,
    /// Moving the enemy cursor onto the action's target.
    SelectingTarget,
    /// Driving the confirm press that commits the target.
    ConfirmingTarget,
    /// The attack animation is playing; land the timed input (tap, multi-tap,
    /// or charge-and-release, per the action's `TimingType`).
    Attacking,
}

/// The result of driving one [`ActionStep`]: stay on this step, advance to
/// another, or the whole action has resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    /// Remain on the current step next frame.
    Stay,
    /// Move to `next` step (the executor resets nothing else).
    Advance(ActionStep),
    /// The action finished; the executor drops it and picks the next.
    Done,
}

/// The exit of one frame of a shared driver (submenu navigation, target
/// cursor): its job is done, it needs more frames, or it can't make progress.
/// Steps match on this to route — the driver knows the mechanics, the step
/// knows where to go next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriveResult {
    /// The driver finished its job (the ability/target is committed).
    Ok,
    /// Still working; drive again next frame.
    Wait,
    /// Can't make progress; the caller must back out and recover.
    Error,
}

/// Bookkeeping an action carries across its steps. Created fresh when the
/// executor latches an action and dropped with it when the action resolves,
/// so nothing here can leak from one action into the next.
#[derive(Debug, Default)]
pub struct StepScratch {
    /// Time spent on the current step. Steps zero it on progress and read it
    /// for settle delays and anti-hang watchdogs.
    pub timer: f64,
    /// Directional/confirm taps spent chasing a menu entry or the target
    /// cursor this step, so steps bail out instead of looping forever.
    pub taps: u32,
    /// Index into the cursor-direction cycle (see `menu::TARGET_DIRS`).
    pub cursor_dir: usize,
    /// Cursor target at the last tap, to detect when a direction stops moving it.
    pub last_cursor: Option<String>,
    /// Enemies the target cursor has already landed on this step. Landing on
    /// one again means the current direction cycles without reaching the
    /// wanted target (e.g. Left/Right ping-ponging between the bottom two of
    /// a triangle), so the driver rotates to the next direction.
    pub visited_targets: Vec<String>,
    /// `timed_attack_ready` last frame, so taps land only on the rising edge.
    pub last_timed_ready: bool,
}

/// Everything an `Action`'s `execute_*` step needs to drive one frame: the
/// live combat snapshot, the active player's gamepad, the in-flight button
/// press, and the action's own [`StepScratch`].
pub struct ActionCtx<'a> {
    pub cmd: &'a CombatManagerData,
    pub gamepad: &'a mut GenericJoystick,
    pub btn: &'a mut ButtonPress,
    pub dt: f64,
    /// The enemy `unique_id` the action wants to hit (`None` = accept the default
    /// cursor target).
    pub want_target: Option<&'a str>,
    /// Total Live Mana charges the appraisal expects the attacker to hold when
    /// the attack lands; [`ActionStep::Boosting`] absorbs up to this.
    pub want_mana_charges: u32,
    pub scratch: &'a mut StepScratch,
}

impl ActionCtx<'_> {
    /// If the attack step sees no progress for this long, the confirms likely
    /// desynced — see [`mash_if_stuck`](Self::mash_if_stuck).
    const STUCK_TIMEOUT: f64 = 6.0;

    /// Whether an ability submenu (combo or skill) is currently open.
    pub(super) fn in_submenu(&self) -> bool {
        self.cmd.ability_submenu_open()
    }

    /// Whether the battle menus are back on screen — during
    /// [`Attacking`](ActionStep::Attacking) this means the action resolved.
    pub(super) fn menus_returned(&self) -> bool {
        self.cmd.battle_command_has_focus || self.in_submenu()
    }

    /// Any acting player's timed-hit window is open this frame.
    pub(super) fn timed_ready(&self) -> bool {
        self.cmd.players.items.iter().any(|p| p.timed_attack_ready)
    }

    /// Safety-net mash: spam Confirm on the active pad to force a stuck turn along.
    pub(super) fn mash(&mut self) {
        if self.btn.done() {
            *self.btn = menu::mash_press();
        }
        self.btn.update(self.gamepad, self.dt);
    }

    /// Anti-hang watchdog: [`mash`](Self::mash) once the step has gone
    /// [`STUCK_TIMEOUT`](Self::STUCK_TIMEOUT) without progress (steps zero
    /// `scratch.timer` when they see progress).
    pub(super) fn mash_if_stuck(&mut self) {
        if self.scratch.timer >= Self::STUCK_TIMEOUT {
            self.mash();
        }
    }
}
