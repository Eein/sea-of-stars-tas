//! Combo — a joint move under the Combo command, costing combo points.
//!
//! Combos are discovered from memory (their `combatMoveId` isn't known at compile
//! time), so a `Combo` carries its `name` and `cost` as fields. Damage is
//! estimated as one basic strike per participant — see
//! [`Self::estimate_damage_at`].

use data::prelude::PlayerPartyCharacter;

use super::{Action, BattleCommand, SkillResource, TimingType};
use crate::combat::damage;
use crate::memory::combat_manager::{CombatEnemy, CombatManagerData, CombatPlayer};

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

    /// A combo strikes once per participant through the standard damage
    /// pipeline (its class has no damage math of its own — see the DualAttack
    /// decompile), so estimate it as the sum of each participant's plain
    /// basic hit. Calibrated against live DualAttack hits on Elder Mist
    /// (observed ≈18 where the old ×3.75 heuristic claimed ≈51); the timed
    /// bonus is deliberately left out, matching those observations. Falls
    /// back to the bound caster's basic when the move isn't readable.
    fn estimate_damage_at(
        &self,
        cmd: &CombatManagerData,
        player: &CombatPlayer,
        enemy: &CombatEnemy,
        roll: f32,
    ) -> f32 {
        let strikers: Vec<&CombatPlayer> = self
            .find_move(cmd)
            .map(|m| {
                m.required_characters
                    .iter()
                    .filter_map(|id| {
                        let parsed = PlayerPartyCharacter::parse(id);
                        (parsed != PlayerPartyCharacter::Unknown)
                            .then(|| cmd.players.items.iter().find(|p| p.character == parsed))
                            .flatten()
                    })
                    .collect()
            })
            .unwrap_or_default();
        if strikers.is_empty() {
            let (base, _) = damage::basic_attack_damage(player, enemy, roll);
            return damage::round_damage(base);
        }
        strikers
            .iter()
            .map(|p| damage::round_damage(damage::basic_attack_damage(p, enemy, roll).0))
            .sum()
    }
}
