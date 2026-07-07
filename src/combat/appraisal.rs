//! Combat appraisal — the decision layer.
//!
//! Given a snapshot of the live combat state, generate candidate actions
//! (currently basic attacks: every controllable player against every live
//! enemy), score each, and rank them. This layer only *decides*; acting on the
//! chosen appraisal (targeting + input) is handled separately.

use data::prelude::PlayerPartyCharacter;

use crate::combat::skills::{self, Action, BasicAttack, Combo};
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
    /// A skill (costs mana; `name` is its `combatMoveId`). Appraised now,
    /// executed in a later slice.
    Skill { name: String, cost: u32 },
}

impl CombatAction {
    pub fn label(&self) -> String {
        match self {
            CombatAction::BasicAttack { timed: true } => "Basic Attack (timed)".to_string(),
            CombatAction::BasicAttack { timed: false } => "Basic Attack".to_string(),
            CombatAction::Combo { name, cost } => format!("Combo: {name} (cp {cost})"),
            CombatAction::Skill { name, cost } => format!("Skill: {name} (mp {cost})"),
        }
    }

    /// Whether the executor can currently act on this action.
    pub fn is_executable(&self) -> bool {
        matches!(
            self,
            CombatAction::BasicAttack { .. }
                | CombatAction::Combo { .. }
                | CombatAction::Skill { .. }
        )
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
    /// Build the executable [`Action`] for this appraisal, bound to its attacker.
    /// This is the bridge the executor drives step-by-step: basic attacks and
    /// combos are constructed from the enum, skills are looked up by name (falling
    /// back to a basic attack if somehow unregistered).
    pub fn to_action(&self) -> Box<dyn Action> {
        match &self.action {
            CombatAction::BasicAttack { timed } => Box::new(BasicAttack {
                character: self.attacker.clone(),
                timed: *timed,
            }),
            CombatAction::Combo { name, cost } => Box::new(Combo {
                character: self.attacker.clone(),
                name: name.clone(),
                cost: *cost,
            }),
            CombatAction::Skill { name, .. } => skills::skill_actions()
                .into_iter()
                .find(|a| a.internal_name() == name)
                .unwrap_or_else(|| {
                    Box::new(BasicAttack {
                        character: self.attacker.clone(),
                        timed: true,
                    })
                }),
        }
    }

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
/// Score a candidate action against an enemy. Damage comes from the action's own
/// [`estimate_damage`](Action::estimate_damage); the lethal/imminent-threat
/// bonuses are shared across every action kind. `combat_action` is the enum form
/// carried on the `Appraisal` for the GUI label and the executor's command routing.
fn score_action(
    cmd: &CombatManagerData,
    action: &dyn Action,
    player: &CombatPlayer,
    enemy: &CombatEnemy,
    combat_action: CombatAction,
) -> Appraisal {
    let expected_damage = action.estimate_damage(cmd, player, enemy);
    let lethal = expected_damage >= enemy.current_hp as f32;

    let mut score = expected_damage;
    if lethal {
        // Securing a kill is worth a big bonus — but halve it for summoned
        // enemies (a boss's adds) so an available boss kill always outranks
        // killing a summon.
        score += if enemy.summoned {
            LETHAL_BONUS / 2.0
        } else {
            LETHAL_BONUS
        };
    }
    // Prioritise enemies whose turn is imminent (turns_to_action counts down).
    if enemy.turns_to_action > 0 {
        score += IMMINENT_THREAT_BONUS / enemy.turns_to_action as f32;
    }

    Appraisal {
        attacker: action.character(),
        action: combat_action,
        target_enemy_id: enemy.unique_id.clone(),
        expected_damage,
        lethal,
        score,
    }
}

/// Whether the fighter for `character` has `command` disabled this fight (e.g. a
/// tutorial forcing a specific command). Normal fights disable nothing.
fn command_disabled(
    cmd: &CombatManagerData,
    character: &PlayerPartyCharacter,
    command: skills::BattleCommand,
) -> bool {
    cmd.moves
        .iter()
        .find(|cm| &cm.character == character)
        .is_some_and(|cm| {
            cm.disabled_commands
                .iter()
                .any(|name| name == command.class_name())
        })
}

/// Generate every candidate appraisal for the current combat state, ranked best
/// first.
///
/// Candidates: a timed basic attack per controllable player, every affordable
/// loaded combo, and every usable skill [`Action`] — each against every living
/// enemy.
pub fn generate_appraisals(cmd: &CombatManagerData) -> Vec<Appraisal> {
    let mut appraisals = Vec::new();
    let living_enemies = || cmd.enemies.items.iter().filter(|e| e.current_hp != 0);

    for player in &cmd.players.items {
        if player.dead || !player.enabled {
            continue;
        }
        if command_disabled(cmd, &player.character, skills::BattleCommand::Attack) {
            continue;
        }
        for enemy in living_enemies() {
            let action = BasicAttack {
                character: player.character.clone(),
                timed: true,
            };
            appraisals.push(score_action(
                cmd,
                &action,
                player,
                enemy,
                CombatAction::BasicAttack { timed: true },
            ));
        }
    }

    // Combos need every participating character alive. We don't read a combo's
    // `requiredCharacters` yet, so as a safe proxy only appraise combos when the
    // whole party is up (a downed member means some combos are uncastable).
    let party_all_alive = cmd.players.items.iter().all(|p| !p.dead);

    // Combos: each character's affordable, available combo moves.
    for character_moves in cmd.moves.iter().filter(|_| party_all_alive) {
        let Some(player) = cmd
            .players
            .items
            .iter()
            .find(|p| p.character == character_moves.character && !p.dead && p.enabled)
        else {
            continue;
        };
        if command_disabled(cmd, &character_moves.character, skills::BattleCommand::Combo) {
            continue;
        }
        for combat_move in &character_moves.moves {
            let cost = combat_move.combo_point_cost.unwrap_or(0);
            // A combo we can appraise for damage: costs (and can afford) combo
            // points, deals damage (excludes heals/buffs like MendingLight, whose
            // damageTypeDefinitions are empty), and is *available*. `loaded` is
            // always false for combos (they have no per-fighter
            // `combatMoveComponent`), so availability comes from `unlockable == 0`
            // — the base combos every party has. Learned-only combos
            // (`unlockable != 0`) stay out until we read a real learned signal.
            let is_damage_combo = cost > 0
                && combat_move.unlockable == Some(0)
                && combat_move.is_damaging
                && cost <= cmd.combo_points;
            let Some(name) = combat_move.move_id.as_deref().filter(|_| is_damage_combo) else {
                continue;
            };
            for enemy in living_enemies() {
                let action = Combo {
                    character: player.character.clone(),
                    name: name.to_string(),
                    cost,
                };
                appraisals.push(score_action(
                    cmd,
                    &action,
                    player,
                    enemy,
                    CombatAction::Combo {
                        name: name.to_string(),
                        cost,
                    },
                ));
            }
        }
    }

    // Skills: each registered skill Action usable this turn (character alive,
    // move loaded, mana affordable) against every living enemy.
    for action in skills::skill_actions() {
        if !action.is_usable(cmd) {
            continue;
        }
        if command_disabled(cmd, &action.character(), skills::BattleCommand::Skill) {
            continue;
        }
        let Some(player) = action.player(cmd) else {
            continue;
        };
        for enemy in living_enemies() {
            appraisals.push(score_action(
                cmd,
                action.as_ref(),
                player,
                enemy,
                CombatAction::Skill {
                    name: action.internal_name().to_string(),
                    cost: action.cost(),
                },
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

/// The appraisal the executor will act on: the top-ranked *executable* one
/// (appraisals are sorted best-first). The single source of the decision rule,
/// shared by the executor and the GUI's appraisal panel.
pub fn choose(appraisals: &[Appraisal]) -> Option<&Appraisal> {
    appraisals.iter().find(|a| a.action.is_executable())
}
