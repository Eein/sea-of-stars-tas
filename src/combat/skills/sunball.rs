//! Zale — Sunball. A Sun-element magic attack.

use data::prelude::PlayerPartyCharacter;

use super::{Action, SkillResource, TimingType};
use crate::memory::combat_manager::{CombatDamageType, CombatEnemy, CombatPlayer};

pub struct Sunball;

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

    fn damage_types(&self) -> Vec<CombatDamageType> {
        vec![CombatDamageType::Sun]
    }

    fn estimate_damage(&self, player: &CombatPlayer, enemy: &CombatEnemy) -> f32 {
        magic_damage_estimate(player, enemy, CombatDamageType::Sun)
    }
}

/// Rough magic-skill estimate: scales with magical attack, reduced by the
/// enemy's magical defense, times the enemy's modifier for `damage_type`.
/// Placeholder pending the real per-skill formula.
pub(super) fn magic_damage_estimate(
    player: &CombatPlayer,
    enemy: &CombatEnemy,
    damage_type: CombatDamageType,
) -> f32 {
    const MAGICAL_DEFENSE_CAP: f32 = 150.0;
    const SKILL_MULTIPLIER: f32 = 2.0;

    let defense_ratio = 1.0 - (enemy.magical_defense as f32 / MAGICAL_DEFENSE_CAP);
    let modifier = enemy
        .damage_type_modifiers
        .items
        .iter()
        .find(|(k, _)| k.key == damage_type)
        .map(|(_, v)| v.value)
        .unwrap_or(1.0);

    (player.magical_attack as f32 * SKILL_MULTIPLIER * modifier * defense_ratio).floor()
}
