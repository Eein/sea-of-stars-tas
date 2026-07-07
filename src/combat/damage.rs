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

/// The game's final `(int)Math.Round(damage)` cast.
///
/// `PlayerBasicAttackDamage.CalculatePreModifiersDamage` rounds the full float
/// damage exactly once, via `System.Math.Round` (round *half to even*), in
/// `double` precision — not `floor`, not round-half-away. Verified against the
/// decompiled `Math.Round` implementation (RVA 0x13880).
pub fn round_damage(damage: f32) -> f32 {
    (damage as f64).round_ties_even() as f32
}

/// Look up the enemy's modifier for a single damage type (neutral `1.0` if the
/// enemy has no entry for it — matches the game's `TryGetValue` miss).
fn modifier_for(enemy: &CombatEnemy, damage_type: CombatDamageType) -> f32 {
    enemy
        .damage_type_modifiers
        .items
        .iter()
        .find(|(k, _v)| k.key == damage_type)
        .map(|(_k, v)| v.value)
        .unwrap_or(1.0)
}

/// The damage-type modifier applied to the **physical** stream (the physical
/// attack stat). The game partitions a move's damage types into physical vs.
/// magic (`CombatDamageType::Magical == 0xFC` is the magic mask) and only the
/// physical types multiply the physical stream. For basic attacks that is the
/// character's weapon type (Sword/Blunt); everyone else is neutral.
fn physical_damage_modifier(player: &CombatPlayer, enemy: &CombatEnemy) -> f32 {
    match player.character {
        PlayerPartyCharacter::Zale => modifier_for(enemy, CombatDamageType::Sword),
        PlayerPartyCharacter::Valere | PlayerPartyCharacter::Garl => {
            modifier_for(enemy, CombatDamageType::Blunt)
        }
        _ => 1.0,
    }
}

/// The damage-type modifier applied to the **magic** stream (the Live Mana
/// boost). Only the elemental (Sun/Moon) type multiplies the magic stream — the
/// weapon type stays on the physical stream. Irrelevant when uncharged, since
/// the magic stream is then zero.
fn magic_damage_modifier(player: &CombatPlayer, enemy: &CombatEnemy) -> f32 {
    match player.character {
        PlayerPartyCharacter::Zale => modifier_for(enemy, CombatDamageType::Sun),
        PlayerPartyCharacter::Valere => modifier_for(enemy, CombatDamageType::Moon),
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
    // floats are very specific - the operation order here mirrors the decompiled
    // `PlayerBasicAttackDamage.GetDamage` (RVA 0x4CE490) so results stay bit-exact.
    let physical_modifier = physical_damage_modifier(player, enemy);
    let magic_modifier = magic_damage_modifier(player, enemy);

    // Physical stream: the game sums the attack stat, adds the random roll, and
    // *then* multiplies by the physical type modifier — i.e. the roll is inside
    // the modifier, not added after it. `(stat + random) * modifier`.
    let physical_stream = (player.physical_attack as f32 + random) * physical_modifier;

    // Magic stream: the Live Mana boost only. Order matches the game's
    // `boostLevel * manaChargeMultiplier * magicStat`, then the magic type
    // modifier. Zero when uncharged (`mana_charge_count == 0`).
    let magic_stream = player.mana_charge_count as f32
        * MANA_CHARGE_STAT_MULTIPLIER
        * player.magical_attack as f32
        * magic_modifier;

    let magical_defense_cap_ratio = enemy.magical_defense as f32 / MAGICAL_DEFENSE_CAP;
    let physical_defense_cap_ratio = enemy.physical_defense as f32 / PHYSICAL_DEFENSE_CAP;

    // Each stream is reduced by its own defense ratio, then summed. Matches the
    // game's `(1 - magDef) * magic + physical * (1 - physDef)`.
    let total_magical_attack = magic_stream * (1.0 - magical_defense_cap_ratio);
    let total_physical_attack = physical_stream * (1.0 - physical_defense_cap_ratio);

    let total_attack = total_magical_attack + total_physical_attack;

    // Timed-hit bonus is `(multiplier - 1) * total`, on the post-defense total.
    // (Combo/multi-hit scaling and TimedAttackBonusDamage modifiers are not yet
    // modelled — this is the base single-hit bonus.)
    let timed_hit_damage = (TIMED_HIT_MULTIPLIER * total_attack) - total_attack;

    (total_attack, timed_hit_damage)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::combat_manager::{DamageTypeModifierKey, DamageTypeModifierValue};

    /// The game rounds with `Math.Round` — half to *even*, not floor and not
    /// half-away-from-zero. These ties pin that down.
    #[test]
    fn round_damage_is_half_to_even() {
        assert_eq!(round_damage(2.4), 2.0);
        assert_eq!(round_damage(2.6), 3.0);
        // exact .5 ties resolve to the nearest even integer
        assert_eq!(round_damage(0.5), 0.0);
        assert_eq!(round_damage(1.5), 2.0);
        assert_eq!(round_damage(2.5), 2.0);
        assert_eq!(round_damage(3.5), 4.0);
        assert_eq!(round_damage(10.5), 10.0);
        assert_eq!(round_damage(11.5), 12.0);
    }

    fn enemy_with(mods: &[(CombatDamageType, f32)]) -> CombatEnemy {
        let mut enemy = CombatEnemy::default();
        for (key, value) in mods {
            enemy.damage_type_modifiers.items.insert(
                DamageTypeModifierKey { key: key.clone() },
                DamageTypeModifierValue { value: *value },
            );
        }
        enemy
    }

    /// Neutral, uncharged basic attack: only the physical stream contributes,
    /// and the max roll (3) sits inside it.
    #[test]
    fn neutral_uncharged_basic_attack() {
        let player = CombatPlayer {
            character: PlayerPartyCharacter::Zale,
            physical_attack: 50,
            magical_attack: 30, // ignored while uncharged
            mana_charge_count: 0,
            ..Default::default()
        };
        let enemy = enemy_with(&[]); // no modifiers -> all neutral 1.0

        let (base, timed) = basic_attack_damage(&player, &enemy, MAX_ROLL);
        assert_eq!(base, 53.0); // (50 + 3) * 1 * (1 - 0)
        assert_eq!(round_damage(base), 53.0);
        // 1.299999 * 53 = 68.8999.. -> Math.Round -> 69
        assert_eq!(round_damage(base + timed), 69.0);
    }

    /// Charged Zale vs an enemy weak to Sword but resistant to Sun. The fix:
    /// the Sword modifier hits *only* the physical stream and the Sun modifier
    /// *only* the magic (Live Mana) stream. The old combined `Sword*Sun == 1.0`
    /// on both streams would give ~133 here instead of 233.
    #[test]
    fn charged_split_applies_each_modifier_to_its_own_stream() {
        let player = CombatPlayer {
            character: PlayerPartyCharacter::Zale,
            physical_attack: 100,
            magical_attack: 200,
            mana_charge_count: 1,
            ..Default::default()
        };
        let enemy = enemy_with(&[
            (CombatDamageType::Sword, 2.0), // physical weakness
            (CombatDamageType::Sun, 0.5),   // magic resistance
        ]);

        // random 0 for clean arithmetic.
        let (base, timed) = basic_attack_damage(&player, &enemy, MIN_ROLL);
        // physical: (100 + 0) * 2.0 = 200 ; magic: 1 * 0.33 * 200 * 0.5 = 33
        assert!((base - 233.0).abs() < 0.05, "base was {base}");
        assert_eq!(round_damage(base), 233.0);
        // 1.299999 * 233 = 302.8997.. -> Math.Round -> 303
        assert_eq!(round_damage(base + timed), 303.0);
    }
}
