//! Zale — Sunball. A Sun-element magic attack that charges: hold Confirm
//! through the QTE and release the frame it peaks to fire at max level.
//!
//! The whole charge is Sunball's own business: the [`ChargeState`] machine
//! below drives the attack step, and the trait's default menu steps handle
//! everything up to the cast.

use data::prelude::PlayerPartyCharacter;
use joystick::common::JoystickBtnInterface;

use super::{Action, ActionCtx, SkillResource, StepOutcome, TargetType, TimingType};
use crate::control::SosAction;
use crate::memory::combat_manager::{
    CombatDamageType, CombatEnemy, CombatManagerData, CombatPlayer, SunballCharge,
    SunballChargeStep,
};

/// How long to keep holding through a `None` charge-read blip before concluding
/// the charge really ended (a few frames' worth). Releasing for even one frame
/// mid-build fires the sunball early.
const DROPOUT_GRACE: f64 = 0.2;

/// The charge QTE as far as our Confirm hold is concerned, advanced once per
/// frame from the live [`SunballCharge`] read. Each state is one phase of the
/// cast and implies exactly one input: hold in [`Priming`](ChargeState::Priming)
/// and [`Building`](ChargeState::Building), release otherwise.
///
/// `dropout` on the holding states is time spent in a transient `None` read;
/// the hold is latched through blips shorter than [`DROPOUT_GRACE`].
#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum ChargeState {
    /// No live charge to drive — before the QTE appears (the caster's leap) or
    /// after a read dropout outlived the grace. Wait, mashing Confirm if the
    /// wait drags on.
    #[default]
    Awaiting,
    /// Holding through the intro. The projectile is pooled, so its `level` can
    /// still read the previous cast's max here — "at max" is not yet trusted.
    Priming { dropout: f64 },
    /// Holding while the level climbs. It's been seen below max this cast, so
    /// the read is live and "at max" means the real peak.
    Building { dropout: f64 },
    /// Released at the peak (or the game reached `Shoot` on its own). The ball
    /// is out; stay released until the menus return and end the action.
    Fired,
}

impl ChargeState {
    /// Whether this state holds Confirm.
    fn holding(self) -> bool {
        matches!(
            self,
            ChargeState::Priming { .. } | ChargeState::Building { .. }
        )
    }

    /// The state after one frame, given the live charge read.
    fn next(self, read: Option<&SunballCharge>, dt: f64) -> Self {
        use ChargeState::*;
        match read {
            Some(charge) => match charge.step {
                // A fresh intro (re)starts the cycle; the level is untrusted.
                SunballChargeStep::In => Priming { dropout: 0.0 },
                SunballChargeStep::Charging => self.charging(charge),
                SunballChargeStep::Shoot | SunballChargeStep::Other => Fired,
            },
            None => match self {
                // Transient read blip mid-hold — keep holding through the grace.
                Priming { dropout } if dropout < DROPOUT_GRACE => Priming {
                    dropout: dropout + dt,
                },
                Building { dropout } if dropout < DROPOUT_GRACE => Building {
                    dropout: dropout + dt,
                },
                // The charge is really gone (fired, ended, or a long dropout).
                _ => Awaiting,
            },
        }
    }

    /// Advance within the game's `Charging` step. The subtlety is trusting
    /// "at max": at the intro→charging boundary the pooled projectile can read
    /// the previous cast's max for a frame, so the peak only counts after the
    /// level has been watched climbing from below ([`Building`](Self::Building)).
    fn charging(self, charge: &SunballCharge) -> Self {
        use ChargeState::*;
        let at_max = charge.max_level > 0 && charge.level >= charge.max_level;
        match self {
            // Already released — the lingering max read is the shot going out.
            Fired => Fired,
            // The climb was watched from below, so this peak is real: release.
            Building { .. } if at_max => Fired,
            Building { .. } => Building { dropout: 0.0 },
            // At max without having seen the climb — a stale pooled level; keep
            // holding until it resets and climbs for real.
            _ if at_max => Priming { dropout: 0.0 },
            _ => Building { dropout: 0.0 },
        }
    }
}

#[derive(Default)]
pub struct Sunball {
    /// Per-cast charge state. The executor latches a fresh `Sunball` for each
    /// cast, so this starts at [`Awaiting`](ChargeState::Awaiting) and dies
    /// with the action.
    charge: ChargeState,
}

impl Action for Sunball {
    fn character(&self) -> PlayerPartyCharacter {
        PlayerPartyCharacter::Zale
    }

    fn internal_name(&self) -> &'static str {
        "Sunball"
    }

    fn resource(&self) -> SkillResource {
        SkillResource::Mana
    }

    fn cost(&self) -> u32 {
        8
    }

    fn timing_type(&self) -> TimingType {
        // Sunball charges: hold Confirm, release on the window.
        TimingType::Charge
    }

    fn target_type(&self) -> TargetType {
        // Sunball's target selector is a `PlayerRadiusTargetSelector`
        // (verified live via the loaded-move selector log): the confirmed
        // target anchors a splash sphere, so the appraiser scores the splash
        // and anchors on the highest-value main target (e.g. the middle of a
        // cluster).
        TargetType::Aoe
    }

    fn qte_in_flight(&self, cmd: &CombatManagerData) -> bool {
        cmd.sunball_charge.is_some()
    }

    fn damage_types(&self) -> Vec<CombatDamageType> {
        vec![CombatDamageType::Sun]
    }

    fn estimate_damage_at(
        &self,
        cmd: &CombatManagerData,
        player: &CombatPlayer,
        enemy: &CombatEnemy,
        roll: f32,
    ) -> f32 {
        self.special_move_estimate(cmd, player, enemy, CombatDamageType::Sun, roll)
    }

    /// Drive the charge QTE: advance [`ChargeState`] from the live read, then
    /// press or release Confirm per the state. The cast resolves like any other
    /// action — the menus return and the step reports done.
    fn execute_attacking(&mut self, ctx: &mut ActionCtx) -> StepOutcome {
        // The menus returning means the cast resolved — release (a held charge
        // would auto-confirm the menu) and hand the turn back.
        if ctx.menus_returned() {
            ctx.gamepad.release(&SosAction::Confirm);
            ctx.scratch.timer = 0.0;
            return StepOutcome::Done;
        }
        if ctx.cmd.sunball_charge.is_some() {
            ctx.scratch.timer = 0.0; // a live QTE is progress for the watchdog
            ctx.scratch.window_seen = true;
        }
        self.charge = self.charge.next(ctx.cmd.sunball_charge.as_ref(), ctx.dt);
        if self.charge.holding() {
            ctx.gamepad.press(&SosAction::Confirm);
        } else {
            ctx.gamepad.release(&SosAction::Confirm);
            // Pre-QTE lull with no QTE ever seen: the confirms desynced —
            // mash the turn along. Once the QTE has appeared the cast is
            // real, and the post-fire quiet is enemy turns playing out (often
            // longer than the stuck timeout) — never mash those.
            if !ctx.scratch.window_seen {
                ctx.mash_if_stuck();
            }
        }
        StepOutcome::Stay
    }
}
