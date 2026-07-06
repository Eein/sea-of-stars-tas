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
/// MVP: basic attacks only. Later this grows Skill/Combo/Item variants.
#[derive(Debug, Clone, PartialEq)]
pub enum CombatAction {
    /// A basic attack. `timed` = we intend to land the timed hit.
    BasicAttack { timed: bool },
}

impl CombatAction {
    pub fn label(&self) -> &'static str {
        match self {
            CombatAction::BasicAttack { timed: true } => "Basic Attack (timed)",
            CombatAction::BasicAttack { timed: false } => "Basic Attack",
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

/// Score a single (player, enemy, action) candidate.
fn score(player: &CombatPlayer, enemy: &CombatEnemy, action: &CombatAction) -> Appraisal {
    let CombatAction::BasicAttack { timed } = action;

    let (base, timed_bonus) = damage::basic_attack_damage(player, enemy, damage::MAX_ROLL);
    let expected_damage = if *timed {
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
        action: action.clone(),
        target_enemy_id: enemy.unique_id.clone(),
        expected_damage,
        lethal,
        score,
    }
}

/// Generate every candidate appraisal for the current combat state, ranked best
/// first.
///
/// Candidates are drawn from every controllable (alive, on-screen) player
/// against every living enemy, assuming a timed basic attack.
pub fn generate_appraisals(cmd: &CombatManagerData) -> Vec<Appraisal> {
    let mut appraisals = Vec::new();

    for player in &cmd.players.items {
        if player.dead || !player.enabled {
            continue;
        }
        for enemy in &cmd.enemies.items {
            if enemy.current_hp == 0 {
                continue;
            }
            appraisals.push(score(
                player,
                enemy,
                &CombatAction::BasicAttack { timed: true },
            ));
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
