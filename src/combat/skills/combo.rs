//! Combo — a joint move under the Combo command, costing combo points.
//!
//! Combos are discovered from memory (their `combatMoveId` isn't known at compile
//! time), so a `Combo` carries its `name` and `cost` as fields. Damage is a
//! placeholder heuristic pending a real combo model — see [`Self::estimate_damage`].

use data::prelude::PlayerPartyCharacter;

use super::{Action, BattleCommand, SkillResource, TimingType};
use crate::combat::damage;
use crate::memory::combat_manager::{CombatEnemy, CombatManagerData, CombatPlayer};

/// Placeholder combo damage estimate: combos hit roughly this multiple of the
/// caster's basic attack, scaled up by combo-point cost. Mirrors the appraiser's
/// prior `combo_damage_estimate` heuristic (pending `damageTypeDefinitions` RE).
const COMBO_DAMAGE_FACTOR: f32 = 2.5;

/// A combo `name` cast by `character`, costing `cost` combo points.
pub struct Combo {
    pub character: PlayerPartyCharacter,
    pub name: String,
    pub cost: u32,
}

impl Action for Combo {
    fn character(&self) -> PlayerPartyCharacter {
        self.character.clone()
    }

    fn internal_name(&self) -> &str {
        &self.name
    }

    fn battle_command(&self) -> BattleCommand {
        BattleCommand::Combo
    }

    fn timing_type(&self) -> TimingType {
        TimingType::OneHit
    }

    fn resource(&self) -> SkillResource {
        SkillResource::ComboPoints
    }

    fn cost(&self) -> u32 {
        self.cost
    }

    fn estimate_damage_at(
        &self,
        _cmd: &CombatManagerData,
        player: &CombatPlayer,
        enemy: &CombatEnemy,
        roll: f32,
    ) -> f32 {
        let (base, timed) = damage::basic_attack_damage(player, enemy, roll);
        ((base + timed) * COMBO_DAMAGE_FACTOR * (1.0 + self.cost as f32 * 0.5)).floor()
    }
}
