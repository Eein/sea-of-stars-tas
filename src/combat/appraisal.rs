//! Combat appraisal — the decision layer.
//!
//! Given a snapshot of the live combat state, generate candidate actions
//! (currently basic attacks: every controllable player against every live
//! enemy), score each, and rank them. This layer only *decides*; acting on the
//! chosen appraisal (targeting + input) is handled separately.

use data::prelude::PlayerPartyCharacter;

use crate::combat::damage;
use crate::combat::skills::{self, Action, BasicAttack, Combo, TargetType};
use crate::memory::combat_manager::{CombatEnemy, CombatManagerData, CombatPlayer};
use crate::util::vec3_ext::Vector3Ext;

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
    /// For AOE actions: the number of *secondary* enemies predicted inside the
    /// splash sphere around the main target (each takes 0.6× damage). Zero for
    /// single-target actions.
    pub splash_targets: u32,
    /// Live Mana charges factored into `expected_damage` (the boost term on
    /// special moves, the magic stream on basic attacks): what the attacker
    /// holds plus what the ground pool can still yield, capped at 3. The
    /// executor's Boosting step absorbs up to this before attacking. Zero
    /// means the estimate is charge-free.
    pub mana_charges: u32,
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
        let splash = match self.splash_targets {
            0 => String::new(),
            n => format!(" | splash {n}"),
        };
        let mana = match self.mana_charges {
            0 => String::new(),
            n => format!(" LM{n}"),
        };
        format!(
            "{:?} -> {} on {} | dmg {:.0}{mana}{}{splash} | score {:.1}",
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
/// Fallback splash radius when `playerAOERadius` hasn't been read yet (its
/// live value — see AOE.md).
const AOE_RADIUS_FALLBACK: f32 = 3.0;
/// The most Live Mana charges a character can hold
/// (`CombatBoostLevelController`'s boost levels run 0..=3).
const MAX_MANA_CHARGES: u32 = 3;
/// Small mana orbs merged into one charge per absorb
/// (`EncounterTransitionToAbsorbState.BeginMergeMana`, RVA 0x4B1CB0, groups of 5).
const SMALL_MANA_PER_CHARGE: u32 = 5;
/// Reach a hit-zone collider adds to the splash sphere: the game's
/// `Physics.OverlapSphere` hits *colliders*, not anchor points, so an enemy is
/// splashed when its anchor is within `radius + extent`. Calibrated against
/// the live 3-enemy boss fight (see AOE.md) pending an exact collider-bounds
/// read.
const AOE_COLLIDER_EXTENT: f32 = 1.7;

/// The living enemies an AOE centred on `main` is predicted to splash, `main`
/// excluded. Mirrors `PlayerRadiusTargetSelector.SelectAOETargets` (RVA
/// 0x6893D0): a sphere of `playerAOERadius` around the main target's AOE
/// anchor. Enemies without a readable position are conservatively not
/// splashed. Identity is by reference — the same enemy *instance* is skipped,
/// not the same `unique_id` (twin adds share their guid).
fn aoe_secondaries<'a>(cmd: &'a CombatManagerData, main: &CombatEnemy) -> Vec<&'a CombatEnemy> {
    let Some(center) = main.position else {
        return Vec::new();
    };
    let reach = cmd.player_aoe_radius.unwrap_or(AOE_RADIUS_FALLBACK) + AOE_COLLIDER_EXTENT;
    cmd.enemies
        .items
        .iter()
        .filter(|e| e.current_hp != 0 && !std::ptr::eq(*e, main))
        .filter(|e| {
            e.position.is_some_and(|p| {
                let (dx, dy, dz) = (
                    p.get_x() - center.get_x(),
                    p.get_y() - center.get_y(),
                    p.get_z() - center.get_z(),
                );
                (dx * dx + dy * dy + dz * dz).sqrt() <= reach
            })
        })
        .collect()
}

/// The kill bonus for downing `enemy` — halved for summoned enemies (a boss's
/// adds) so an available boss kill always outranks killing a summon.
fn kill_bonus(enemy: &CombatEnemy) -> f32 {
    if enemy.summoned {
        LETHAL_BONUS / 2.0
    } else {
        LETHAL_BONUS
    }
}

/// The Live Mana charges `player` can enter the attack with: what they already
/// hold plus what the ground pool can still yield (each big particle is one
/// charge; every 5 small merge into one), capped at the game's 3. The executor's
/// Boosting step absorbs the difference before the attack.
fn potential_mana_charges(cmd: &CombatManagerData, player: &CombatPlayer) -> u32 {
    let absorbable = cmd.live_mana.big + cmd.live_mana.small / SMALL_MANA_PER_CHARGE;
    (player.mana_charge_count + absorbable).min(MAX_MANA_CHARGES)
}

/// Score a candidate action against an enemy. Damage comes from the action's own
/// [`estimate_damage`](Action::estimate_damage), computed as if the attacker has
/// boosted to their [`potential_mana_charges`] (the executor absorbs to match);
/// the lethal/imminent-threat bonuses are shared across every action kind.
/// `combat_action` is the enum form carried on the `Appraisal` for the GUI label
/// and the executor's command routing.
///
/// AOE actions additionally score their predicted splash: every secondary in
/// the sphere contributes its *effective* damage (0.6× the full hit, capped by
/// its remaining HP — overkill on a 1-HP add is worthless) plus a kill bonus
/// when the splash downs it. This is what makes "hit the boss, splash both
/// adds" outrank "overkill one add directly".
fn score_action(
    cmd: &CombatManagerData,
    action: &dyn Action,
    player: &CombatPlayer,
    enemy: &CombatEnemy,
    combat_action: CombatAction,
) -> Appraisal {
    // Estimate as the boosted attacker when the ground pool allows it.
    let mana_charges = potential_mana_charges(cmd, player);
    let boosted;
    let player = if mana_charges > player.mana_charge_count {
        boosted = CombatPlayer {
            mana_charge_count: mana_charges,
            ..player.clone()
        };
        &boosted
    } else {
        player
    };
    let expected_damage = action.estimate_damage(cmd, player, enemy);
    let lethal = expected_damage >= enemy.current_hp as f32;

    let mut score = expected_damage;
    if lethal {
        score += kill_bonus(enemy);
    }
    // Prioritise enemies whose turn is imminent (turns_to_action counts down).
    if enemy.turns_to_action > 0 {
        score += IMMINENT_THREAT_BONUS / enemy.turns_to_action as f32;
    }

    let mut splash_targets = 0;
    if action.target_type() == TargetType::Aoe {
        for secondary in aoe_secondaries(cmd, enemy) {
            let splash_damage = damage::round_damage(
                action.estimate_damage(cmd, player, secondary) * damage::AOE_SECONDARY_MULTIPLIER,
            );
            score += splash_damage.min(secondary.current_hp as f32);
            if splash_damage >= secondary.current_hp as f32 {
                score += kill_bonus(secondary);
            }
            splash_targets += 1;
        }
    }

    Appraisal {
        attacker: action.character(),
        action: combat_action,
        target_enemy_id: enemy.unique_id.clone(),
        expected_damage,
        lethal,
        splash_targets,
        mana_charges,
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

/// Whether a party character matches a move definition's `requiredCharacters`
/// id (the game's `CharacterDefinitionId` strings, e.g. `"ZALE"`). Characters
/// we don't model (Artificer, the god-forms, ...) match nothing, which
/// correctly filters out their combos.
fn character_matches_id(character: &PlayerPartyCharacter, id: &str) -> bool {
    matches!(
        (character, id),
        (PlayerPartyCharacter::Zale, "ZALE")
            | (PlayerPartyCharacter::Valere, "VALERE")
            | (PlayerPartyCharacter::Garl, "GARL")
            | (PlayerPartyCharacter::Serai, "SERAI")
            | (PlayerPartyCharacter::Reshan, "RESHAN")
            | (PlayerPartyCharacter::Bst, "BST")
    )
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

    // Combos: each character's affordable, available combo moves.
    for character_moves in cmd.moves.iter() {
        let Some(player) = cmd
            .players
            .items
            .iter()
            .find(|p| p.character == character_moves.character && !p.dead && p.enabled)
        else {
            continue;
        };
        if command_disabled(
            cmd,
            &character_moves.character,
            skills::BattleCommand::Combo,
        ) {
            continue;
        }
        for combat_move in &character_moves.moves {
            let cost = combat_move.combo_point_cost.unwrap_or(0);
            // A combo we can appraise for damage: costs (and can afford) combo
            // points, deals damage (excludes heals/buffs like MendingLight, whose
            // damageTypeDefinitions are empty), and is *available* — it has a
            // live `combatMoveComponent` (`loaded`). The game only instantiates
            // the fight's real combos: scripted variants that share the party's
            // `requiredCharacters` (DualAttackKids, SpectacleStrike) stay
            // unloaded in normal encounters, so `unlockable == 0` alone is not
            // enough to admit a combo.
            let is_damage_combo = cost > 0
                && combat_move.loaded
                && combat_move.unlocked
                && combat_move.is_damaging
                && cost <= cmd.combo_points;
            // Every participating character must be in the party and alive
            // (`requiredCharacters` on the move definition) — e.g. Garl combos
            // are uncastable without Garl, and any combo needs its partners up.
            let participants_ready = combat_move.required_characters.iter().all(|id| {
                cmd.players
                    .items
                    .iter()
                    .any(|p| !p.dead && character_matches_id(&p.character, id))
            });
            if !participants_ready {
                continue;
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use vec3_rs::Vector3;

    /// The live 3-enemy boss fight captured in AOE.md: boss centre, one add on
    /// each side. Boss↔add distances (~4.3–4.6) are inside the splash reach
    /// (3.0 + 1.7); the add↔add distance (~8.6) is not.
    fn live_case() -> CombatManagerData {
        let mut cmd = CombatManagerData {
            player_aoe_radius: Some(3.0),
            ..Default::default()
        };
        let enemy = |hp: u32, mdef: u32, x: f32, z: f32| CombatEnemy {
            current_hp: hp,
            magical_defense: mdef,
            position: Some(Vector3::new(x, 2.0, z)),
            summoned: true,
            ..Default::default()
        };
        cmd.enemies.items = vec![
            enemy(250, 50, -39.38, 237.82), // boss
            enemy(1, 0, -34.93, 236.62),    // add, right
            enemy(1, 0, -43.49, 236.49),    // add, left
        ];
        cmd
    }

    #[test]
    fn splash_prediction_matches_observed_hit_sets() {
        let cmd = live_case();
        let hits = |main: usize| aoe_secondaries(&cmd, &cmd.enemies.items[main]).len();
        // Centred on the boss both adds are splashed; centred on either add
        // only the boss is (the adds are too far apart to splash each other).
        assert_eq!(hits(0), 2);
        assert_eq!(hits(1), 1);
        assert_eq!(hits(2), 1);
    }

    /// 15 small mana on the ground = 3 potential charges (5 merge into one,
    /// capped at 3): the appraisal estimates as the boosted attacker and tags
    /// the charge count for the executor's Boosting step.
    #[test]
    fn appraisal_counts_potential_mana_charges_from_the_ground_pool() {
        let mut cmd = live_case();
        let player = CombatPlayer {
            character: PlayerPartyCharacter::Valere,
            magical_attack: 13,
            ..Default::default()
        };
        let action = skills::skill_actions()
            .into_iter()
            .find(|a| a.internal_name() == "CrescentArc")
            .unwrap();
        let appraise = |cmd: &CombatManagerData| {
            score_action(
                cmd,
                action.as_ref(),
                &player,
                &cmd.enemies.items[0],
                CombatAction::Skill {
                    name: "CrescentArc".into(),
                    cost: 6,
                },
            )
        };

        let uncharged = appraise(&cmd);
        assert_eq!(uncharged.mana_charges, 0);

        cmd.live_mana.small = 15;
        let charged = appraise(&cmd);
        assert_eq!(charged.mana_charges, 3);
        assert!(
            charged.expected_damage > uncharged.expected_damage,
            "boosted estimate ({}) should beat uncharged ({})",
            charged.expected_damage,
            uncharged.expected_damage
        );

        // 4 small can't complete a merge; already-held charges still count.
        cmd.live_mana.small = 4;
        assert_eq!(appraise(&cmd).mana_charges, 0);
        cmd.live_mana.big = 2;
        assert_eq!(appraise(&cmd).mana_charges, 2);
    }

    /// AOE scoring makes "hit the boss, splash both 1-HP adds" outrank
    /// "overkill one add directly": two splash kills beat one direct kill.
    #[test]
    fn aoe_prefers_the_boss_centre_over_direct_add_kill() {
        let cmd = live_case();
        let player = CombatPlayer {
            character: PlayerPartyCharacter::Valere,
            magical_attack: 13,
            ..Default::default()
        };
        let action = skills::skill_actions()
            .into_iter()
            .find(|a| a.internal_name() == "CrescentArc")
            .unwrap();
        let score = |enemy| {
            score_action(
                &cmd,
                action.as_ref(),
                &player,
                enemy,
                CombatAction::Skill {
                    name: "CrescentArc".into(),
                    cost: 6,
                },
            )
        };

        let on_boss = score(&cmd.enemies.items[0]);
        let on_add = score(&cmd.enemies.items[1]);
        assert_eq!(on_boss.splash_targets, 2);
        assert_eq!(on_add.splash_targets, 1);
        assert!(
            on_boss.score > on_add.score,
            "boss-centre ({}) should outrank add-centre ({})",
            on_boss.score,
            on_add.score
        );
    }
}
