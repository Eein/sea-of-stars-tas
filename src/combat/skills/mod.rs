//! Combat actions — one module (an `Action` impl) per skill.
//!
//! An `Action` is the unit the utility AI appraises (`is_usable` +
//! `calculate_value`) and the executor drives step-by-step (the `execute_*`
//! methods, one per [`ActionStep`]). Basic attacks and combos are modelled by
//! the appraiser's `CombatAction` enum today; skills — which each want custom
//! timing, damage formulas, and types — live here as trait impls so a simple
//! skill is a few lines and a weird one overrides only what it needs.

use data::prelude::PlayerPartyCharacter;

use crate::combat::damage;
use crate::memory::combat_manager::{
    CombatDamageType, CombatEnemy, CombatManagerData, CombatPlayer,
};

mod crescent_arc;
mod sunball;

/// The steps an action is driven through, mirroring the executor's `TurnFsm`.
/// The executor calls the matching `Action::execute_*` for the current step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionStep {
    /// Navigating the battle-command ring (Attack/Skill/Combo/Item).
    SelectingCommand,
    /// Boosting the command with extra mana (optional).
    Boost,
    /// Confirming the highlighted command.
    ConfirmCommand,
    /// Navigating the skill/combo submenu to the desired ability.
    SelectingSkill,
    /// Confirming the highlighted ability.
    ConfirmSkill,
    /// Moving the enemy cursor onto the target.
    SelectingEnemySequence,
    /// Confirming the target and committing.
    ConfirmEnemySequence,
    /// Landing the timed input during the animation.
    TimingSequence,
    /// The action has resolved.
    ActionComplete,
}

/// How the timed input is landed during [`ActionStep::TimingSequence`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimingType {
    /// No timed input.
    None,
    /// A single crisp tap when the window opens.
    OneHit,
    /// Hold, then release when the window fires.
    Charge,
    /// One tap per hit across a multi-hit animation.
    MultiHit,
}

/// What an action targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    None,
    Player,
    Enemy,
    Aoe,
    All,
}

/// The resource an action spends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillResource {
    None,
    Mana,
    ComboPoints,
    UltimateGuage,
}

/// Which top-level battle command the action lives under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleCommand {
    Attack,
    Skill,
    Combo,
    Item,
}

/// A combat action. One `impl` per skill; identity + usability + value power the
/// appraiser now, and the `execute_*` steps will drive the executor (slice 2).
#[allow(unused_variables)]
pub trait Action {
    // --- Identity ---

    /// The character who performs the action.
    fn character(&self) -> PlayerPartyCharacter;
    /// The move's `combatMoveId` (matches `CombatMove.move_id`).
    fn internal_name(&self) -> &'static str;
    fn battle_command(&self) -> BattleCommand {
        BattleCommand::Skill
    }
    fn target_type(&self) -> TargetType {
        TargetType::Enemy
    }
    fn timing_type(&self) -> TimingType {
        // Most skills land a single timed hit; only a few charge (Sunball) or
        // hit multiple times (Moonerang). Those override this.
        TimingType::OneHit
    }
    fn resource(&self) -> SkillResource {
        SkillResource::Mana
    }
    fn cost(&self) -> u32 {
        0
    }
    fn damage_types(&self) -> Vec<CombatDamageType> {
        Vec::new()
    }

    // --- Appraisal (utility AI) ---

    /// Whether the action can be used this turn: its character is alive and
    /// on-screen, the move is loaded (unlocked), and the resource is affordable.
    fn is_usable(&self, cmd: &CombatManagerData) -> bool {
        let Some(player) = self.player(cmd) else {
            return false;
        };
        if player.dead || !player.enabled {
            return false;
        }
        if !self.move_loaded(cmd) {
            return false;
        }
        match self.resource() {
            SkillResource::None => true,
            SkillResource::Mana => player.current_mp >= self.mp_cost(cmd),
            SkillResource::ComboPoints => cmd.combo_points >= self.cost(),
            SkillResource::UltimateGuage => cmd.ultimate_progress >= 1.0,
        }
    }

    /// The move's live MP cost, read from its `skillPointCost` in the move
    /// definition, falling back to the module's declared [`cost`](Self::cost)
    /// when the move isn't present in memory.
    fn mp_cost(&self, cmd: &CombatManagerData) -> u32 {
        cmd.moves
            .iter()
            .filter(|cm| cm.character == self.character())
            .flat_map(|cm| &cm.moves)
            .find(|m| m.move_id.as_deref() == Some(self.internal_name()))
            .and_then(|m| m.skill_point_cost)
            .unwrap_or_else(|| self.cost())
    }

    /// Estimated damage against `enemy`. Default is a multiple of the basic
    /// attack; damaging skills override this with their real formula.
    fn estimate_damage(&self, player: &CombatPlayer, enemy: &CombatEnemy) -> f32 {
        let (base, timed) = damage::basic_attack_damage(player, enemy, damage::MAX_ROLL);
        ((base + timed) * 1.5).floor()
    }

    // --- Execution steps (wired into the FSM in the execution slice) ---
    //
    // The executor dispatches the current `ActionStep` to the matching method;
    // each returns `true` when its step is complete. Default no-ops let simple
    // actions inherit the standard menu-driving behaviour.

    fn execute_boost(&self, cmd: &CombatManagerData) -> bool {
        true
    }
    fn execute_selecting_command(&self, cmd: &CombatManagerData) -> bool {
        true
    }
    fn execute_confirm_command(&self, cmd: &CombatManagerData) -> bool {
        true
    }
    fn execute_selecting_skill(&self, cmd: &CombatManagerData) -> bool {
        true
    }
    fn execute_confirm_skill(&self, cmd: &CombatManagerData) -> bool {
        true
    }
    fn execute_selecting_enemy_sequence(&self, cmd: &CombatManagerData) -> bool {
        true
    }
    fn execute_confirm_enemy_sequence(&self, cmd: &CombatManagerData) -> bool {
        true
    }
    fn execute_timing_sequence(&self, cmd: &CombatManagerData) -> bool {
        true
    }
    fn execute_complete(&self, cmd: &CombatManagerData) -> bool {
        true
    }

    // --- Helpers ---

    /// The live `CombatPlayer` for this action's character, if present.
    fn player<'a>(&self, cmd: &'a CombatManagerData) -> Option<&'a CombatPlayer> {
        cmd.players
            .items
            .iter()
            .find(|p| p.character == self.character())
    }

    /// Whether this action's move is loaded (unlocked) for its character.
    fn move_loaded(&self, cmd: &CombatManagerData) -> bool {
        cmd.moves
            .iter()
            .filter(|cm| cm.character == self.character())
            .flat_map(|cm| &cm.moves)
            .any(|m| m.move_id.as_deref() == Some(self.internal_name()) && m.loaded)
    }
}

/// Every skill action the appraiser considers. One entry per skill module.
pub fn skill_actions() -> Vec<Box<dyn Action>> {
    vec![
        Box::new(sunball::Sunball),
        Box::new(crescent_arc::CrescentArc),
    ]
}

/// The timing type of the skill with this `combatMoveId`, if registered.
pub fn skill_timing(internal_name: &str) -> Option<TimingType> {
    skill_actions()
        .iter()
        .find(|a| a.internal_name() == internal_name)
        .map(|a| a.timing_type())
}
