#![allow(dead_code)]

//! Combat — the appraisal-driven battle executor.
//!
//! The pipeline, one module per stage:
//! - `appraisal` — decide: score every candidate action against every living
//!   enemy and rank them.
//! - `controller` — route: recompute the appraisal each frame and hand control
//!   to the turn executor (normal encounters) or a Confirm mash (dialogue,
//!   not-yet-modelled fights).
//! - `turn` — drive the turn: whose turn it is, which action is latched, and
//!   the `TurnState` machine that walks it through the menus.
//! - `skills` — act: the `Action` trait (one impl per skill) with the per-step
//!   menu-driving logic.
//! - `damage` — the shared damage formulas.

pub mod appraisal;
mod controller;
pub mod damage;
mod skills;
mod turn;

pub use controller::CombatController;
