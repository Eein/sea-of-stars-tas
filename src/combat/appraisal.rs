//! Combat appraisal — the decision layer.
//!
//! Given a snapshot of the live combat state, generate candidate actions
//! (currently basic attacks: every controllable player against every live
//! enemy), score each, and rank them. This layer only *decides*; acting on the
//! chosen appraisal (targeting + input) is handled separately.

use data::prelude::PlayerPartyCharacter;

use crate::combat::damage;
use crate::memory::combat_manager::{CombatEnemy, CombatManagerData, CombatPlayer};

/// A concrete thing a player can do on their turn.
///
/// Grows Skill/Item variants as they're modelled.
#[derive(Debug, Clone, PartialEq)]
pub enum CombatAction {
    /// A basic attack. `timed` = we intend to land the timed hit.
    BasicAttack { timed: bool },
    /// A combo move (costs combo points; `name` is its `combatMoveId`).
    Combo { name: String, cost: u32 },
}

impl CombatAction {
    pub fn label(&self) -> String {
        match self {
            CombatAction::BasicAttack { timed: true } => "Basic Attack (timed)".to_string(),
            CombatAction::BasicAttack { timed: false } => "Basic Attack".to_string(),
            CombatAction::Combo { name, cost } => format!("Combo: {name} (cp {cost})"),
        }
    }

    /// Whether the executor can currently act on this action.
    pub fn is_executable(&self) -> bool {
        matches!(
            self,
            CombatAction::BasicAttack { .. } | CombatAction::Combo { .. }
        )
    }

    /// The top-level battle-command ring index this action lives under
    /// (`Attack=0, Skill=1, Combo=2, Item=3`).
    pub fn battle_command_index(&self) -> i64 {
        match self {
            CombatAction::BasicAttack { .. } => 0,
            CombatAction::Combo { .. } => 2,
        }
    }

    /// The combo's move name, if this action is a combo.
    pub fn combo_name(&self) -> Option<&str> {
        match self {
            CombatAction::Combo { name, .. } => Some(name),
            _ => None,
        }
    }
}

/// A scored candidate action: have `attacker` perform `action` against the
/// enemy identified by `target_enemy_id`.
#[derive(Debug, Clone)]
pub struct Appraisal {
    pub attacker: PlayerPartyCharacter,
    pub action: CombatAction,
    /// The target enemy's `unique_id`.
    pub target_enemy_id: String,
    /// Expected damage (floored, timed hit included when `action` is timed).
    pub expected_damage: f32,
    /// Whether `expected_damage` is enough to kill the target outright.
    pub lethal: bool,
    /// Utility score used for ranking. Higher is better.
    pub score: f32,
}

impl Appraisal {
    /// One-line human-readable summary for logging / the GUI.
    pub fn describe(&self) -> String {
        format!(
            "{:?} -> {} on {} | dmg {:.0}{} | score {:.1}",
            self.attacker,
            self.action.label(),
            self.target_enemy_id,
            self.expected_damage,
            if self.lethal { " (lethal)" } else { "" },
            self.score,
        )
    }
}

/// Bonus for an action that kills its target outright — securing a kill is
/// worth more than raw damage spread across a survivor.
const LETHAL_BONUS: f32 = 1000.0;
/// Bonus for targeting an enemy that is about to act, weighted by how imminent
/// its turn is (fewer turns-to-action = larger nudge).
const IMMINENT_THREAT_BONUS: f32 = 50.0;
/// Placeholder combo damage estimate: combos hit roughly this multiple of the
/// attacker's basic attack, scaled up by combo-point cost. This is a heuristic
/// pending a real combo damage model (needs `damageTypeDefinitions` RE).
const COMBO_DAMAGE_FACTOR: f32 = 2.5;

/// Score a basic-attack candidate against an enemy.
fn score_basic(player: &CombatPlayer, enemy: &CombatEnemy, timed: bool) -> Appraisal {
    let (base, timed_bonus) = damage::basic_attack_damage(player, enemy, damage::MAX_ROLL);
    let expected_damage = if timed {
        (base + timed_bonus).floor()
    } else {
        base.floor()
    };

    let lethal = expected_damage >= enemy.current_hp as f32;

    let mut score = expected_damage;
    if lethal {
        score += LETHAL_BONUS;
    }
    // Prioritise enemies whose turn is imminent (turns_to_action counts down).
    if enemy.turns_to_action > 0 {
        score += IMMINENT_THREAT_BONUS / enemy.turns_to_action as f32;
    }

    Appraisal {
        attacker: player.character.clone(),
        action: CombatAction::BasicAttack { timed },
        target_enemy_id: enemy.unique_id.clone(),
        expected_damage,
        lethal,
        score,
    }
}

/// Estimate a combo's damage against an enemy as a heuristic multiple of the
/// attacker's basic attack (higher-cost combos hit harder). Placeholder until a
/// real combo damage model exists.
fn combo_damage_estimate(player: &CombatPlayer, enemy: &CombatEnemy, cost: u32) -> f32 {
    let (base, timed_bonus) = damage::basic_attack_damage(player, enemy, damage::MAX_ROLL);
    ((base + timed_bonus) * COMBO_DAMAGE_FACTOR * (1.0 + cost as f32 * 0.5)).floor()
}

/// Score a combo candidate: `attacker` uses combo `name` (costing `cost` combo
/// points) against `enemy`.
fn score_combo(player: &CombatPlayer, enemy: &CombatEnemy, name: &str, cost: u32) -> Appraisal {
    let expected_damage = combo_damage_estimate(player, enemy, cost);
    let lethal = expected_damage >= enemy.current_hp as f32;

    let mut score = expected_damage;
    if lethal {
        score += LETHAL_BONUS;
    }
    if enemy.turns_to_action > 0 {
        score += IMMINENT_THREAT_BONUS / enemy.turns_to_action as f32;
    }

    Appraisal {
        attacker: player.character.clone(),
        action: CombatAction::Combo {
            name: name.to_string(),
            cost,
        },
        target_enemy_id: enemy.unique_id.clone(),
        expected_damage,
        lethal,
        score,
    }
}

/// Generate every candidate appraisal for the current combat state, ranked best
/// first.
///
/// Candidates: a timed basic attack for every controllable (alive, on-screen)
/// player against every living enemy, plus every affordable, loaded combo move
/// (`combo_point_cost > 0` and `<= combo_points`) against every living enemy.
pub fn generate_appraisals(cmd: &CombatManagerData) -> Vec<Appraisal> {
    let mut appraisals = Vec::new();
    let living_enemies = || cmd.enemies.items.iter().filter(|e| e.current_hp != 0);

    for player in &cmd.players.items {
        if player.dead || !player.enabled {
            continue;
        }
        for enemy in living_enemies() {
            appraisals.push(score_basic(player, enemy, true));
        }
    }

    // Combos need every participating character alive. We don't read a combo's
    // `requiredCharacters` yet, so as a safe proxy only appraise combos when the
    // whole party is up (a downed member means some combos are uncastable).
    let party_all_alive = cmd.players.items.iter().all(|p| !p.dead);

    // Combos: each character's affordable, loaded combo moves.
    for character_moves in cmd.moves.iter().filter(|_| party_all_alive) {
        let Some(player) = cmd
            .players
            .items
            .iter()
            .find(|p| p.character == character_moves.character && !p.dead && p.enabled)
        else {
            continue;
        };
        for combat_move in &character_moves.moves {
            let cost = combat_move.combo_point_cost.unwrap_or(0);
            // A combo we can appraise for damage: costs (and can afford) combo
            // points, is loaded, and actually deals damage (excludes heals/buffs
            // like MendingLight, whose damageTypeDefinitions are empty).
            let is_damage_combo = cost > 0
                && combat_move.loaded
                && combat_move.is_damaging
                && cost <= cmd.combo_points;
            let Some(name) = combat_move.move_id.as_deref().filter(|_| is_damage_combo) else {
                continue;
            };
            for enemy in living_enemies() {
                appraisals.push(score_combo(player, enemy, name, cost));
            }
        }
    }

    appraisals.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    appraisals
}

/// Pick the single best appraisal for the current state, if any.
pub fn pick_best(cmd: &CombatManagerData) -> Option<Appraisal> {
    generate_appraisals(cmd).into_iter().next()
}
