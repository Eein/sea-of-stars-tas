//! Damage formulas, shared by the combat appraiser and the GUI.
//!
//! The basic-attack formula is lifted verbatim from the GUI helper so the
//! decision layer and the on-screen damage readout compute from a single
//! source of truth; the magic-skill estimate is the shared placeholder the
//! magic skills appraise with.

use data::prelude::PlayerPartyCharacter;

use crate::memory::combat_manager::{CombatDamageType, CombatEnemy, CombatPlayer};

const PHYSICAL_DEFENSE_CAP: f32 = 150.0;
const MAGICAL_DEFENSE_CAP: f32 = 150.0;
const MANA_CHARGE_STAT_MULTIPLIER: f32 = 0.330000;
/// `globalCombatSettings -> basicAttackTimedHitMultiplier`.
const TIMED_HIT_MULTIPLIER: f32 = 1.299999;

/// The 0..=3 damage roll the game applies on a basic attack. Passing the low
/// and high ends gives the min/max damage window.
pub const MIN_ROLL: f32 = 0.0;
pub const MAX_ROLL: f32 = 3.0;

/// Look up a character's per-hit damage-type modifier against an enemy.
///
/// Only Zale/Valere/Garl carry real damage-type logic today; everyone else
/// falls through to a neutral `1.0`. Mana-charged Zale/Valere also apply their
/// secondary (Sun/Moon) type.
fn damage_type_attack_modifier(player: &CombatPlayer, enemy: &CombatEnemy) -> f32 {
    let modifier_for = |damage_type: CombatDamageType| -> f32 {
        enemy
            .damage_type_modifiers
            .items
            .iter()
            .find(|(k, _v)| k.key == damage_type)
            .map(|(_k, v)| v.value)
            .unwrap_or(1.0)
    };

    match player.character {
        PlayerPartyCharacter::Zale => {
            let mut modifiers = modifier_for(CombatDamageType::Sword);
            if player.mana_charge_count > 0 {
                modifiers *= modifier_for(CombatDamageType::Sun);
            }
            modifiers
        }
        PlayerPartyCharacter::Valere => {
            let mut modifiers = modifier_for(CombatDamageType::Blunt);
            if player.mana_charge_count > 0 {
                modifiers *= modifier_for(CombatDamageType::Moon);
            }
            modifiers
        }
        PlayerPartyCharacter::Garl => modifier_for(CombatDamageType::Blunt),
        _ => 1.0,
    }
}

/// Rough magic-skill estimate: scales with magical attack, reduced by the
/// enemy's magical defense, times the enemy's modifier for `damage_type`.
/// Placeholder pending the real per-skill formula.
pub fn magic_damage_estimate(
    player: &CombatPlayer,
    enemy: &CombatEnemy,
    damage_type: CombatDamageType,
) -> f32 {
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

/// Compute a basic attack's `(base_damage, timed_hit_bonus)` for a given roll.
///
/// The in-game value is cast to `(int)`, so callers should `.floor()` before
/// display or comparison. `timed_hit_bonus` is the *additional* damage a
/// successful timed hit adds on top of `base_damage`.
pub fn basic_attack_damage(player: &CombatPlayer, enemy: &CombatEnemy, random: f32) -> (f32, f32) {
    // floats are very specific - this matters
    let attack_modifier = damage_type_attack_modifier(player, enemy);

    // Apply boosted damage modifier to physical attack on basic attacks
    let boosted_live_mana_attack =
        player.magical_attack as f32 * MANA_CHARGE_STAT_MULTIPLIER * attack_modifier;

    let total_physical_attack = (attack_modifier * player.physical_attack as f32) + random;

    let magical_defense_cap_ratio = enemy.magical_defense as f32 / MAGICAL_DEFENSE_CAP;
    let physical_defense_cap_ratio = enemy.physical_defense as f32 / PHYSICAL_DEFENSE_CAP;

    let mut total_magical_attack = boosted_live_mana_attack * player.mana_charge_count as f32;
    total_magical_attack *= 1.0 - magical_defense_cap_ratio; // deduct enemy defense ratio

    let mut total_physical_attack = total_physical_attack;
    total_physical_attack *= 1.0 - physical_defense_cap_ratio; // deduct enemy defense ratio

    let total_attack = total_magical_attack + total_physical_attack;

    let timed_hit_damage = (TIMED_HIT_MULTIPLIER * total_attack) - total_attack;

    (total_attack, timed_hit_damage)
}
