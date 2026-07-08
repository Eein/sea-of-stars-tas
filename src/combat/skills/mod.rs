//! Combat actions — what a character can do on their turn, and how to do it.
//!
//! An `Action` is the unit the utility AI appraises (`is_usable` +
//! `estimate_damage`) and the executor drives step-by-step (the `execute_*`
//! methods, one per `ActionStep`). Basic attacks and combos are modelled by
//! the appraiser's `CombatAction` enum today; skills — which each want custom
//! timing, damage formulas, and types — live here as trait impls so a simple
//! skill is a few lines and a weird one overrides only what it needs.
//!
//! Module map:
//! - `action` — the [`Action`] trait: identity, appraisal hooks, and the
//!   default step-driving behaviour.
//! - `step` — the step machine's vocabulary: [`ActionStep`], [`StepOutcome`],
//!   and the per-frame [`ActionCtx`] / per-action [`StepScratch`].
//! - `menu` — the input primitives: button-press builders and the shared
//!   submenu / target-cursor drivers.
//! - `basic_attack`, `combo`, and one module per skill — the impls.

mod action;
mod basic_attack;
mod combo;
mod crescent_arc;
mod menu;
mod step;
mod sunball;

pub use action::{Action, BattleCommand, SkillResource, TargetType, TimingType};
pub use basic_attack::BasicAttack;
pub use combo::Combo;
pub use step::{ActionCtx, ActionStep, StepOutcome, StepScratch};

pub(crate) use menu::{cancel_press, mash_press, tap_press};

/// Every skill action the appraiser considers. One entry per skill module.
pub fn skill_actions() -> Vec<Box<dyn Action>> {
    vec![
        Box::new(sunball::Sunball),
        Box::new(crescent_arc::CrescentArc),
    ]
}

/// The timing type of the skill with this `combatMoveId`, if registered.
pub fn skill_timing(internal_name: &str) -> Option<TimingType> {
    skill_actions()
        .iter()
        .find(|a| a.internal_name() == internal_name)
        .map(|a| a.timing_type())
}

/// The registered [`Charge`](TimingType::Charge) action (Sunball), if any. Used
/// to (re-)latch the charge QTE onto the executor when it appears — the charge
/// plays out after the menu turn, so it may need driving even when we didn't
/// commit it through the menus.
pub fn charge_action() -> Option<Box<dyn Action>> {
    skill_actions()
        .into_iter()
        .find(|a| a.timing_type() == TimingType::Charge)
}
