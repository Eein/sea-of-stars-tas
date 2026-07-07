//! Basic attack — the always-available Attack command for any character.
//!
//! Unlike the per-character skills, a basic attack applies to whoever is acting,
//! so it's constructed bound to a `character` (mirrors the appraiser building one
//! candidate per controllable player).

use data::prelude::PlayerPartyCharacter;

use super::{Action, BattleCommand, SkillResource, TimingType};
use crate::combat::damage;
use crate::memory::combat_manager::{CombatEnemy, CombatManagerData, CombatPlayer};

/// A basic attack by `character`. `timed` = we intend to land the timed hit.
pub struct BasicAttack {
    pub character: PlayerPartyCharacter,
    pub timed: bool,
}

impl Action for BasicAttack {
    fn character(&self) -> PlayerPartyCharacter {
        self.character.clone()
    }

    fn internal_name(&self) -> &str {
        "BasicAttack"
    }

    fn battle_command(&self) -> BattleCommand {
        BattleCommand::Attack
    }

    fn timing_type(&self) -> TimingType {
        TimingType::OneHit
    }

    fn resource(&self) -> SkillResource {
        SkillResource::None
    }

    /// A basic attack is always available to a living, enabled character — there
    /// is no move to load and no resource to afford.
    fn is_usable(&self, cmd: &CombatManagerData) -> bool {
        self.player(cmd).is_some_and(|p| !p.dead && p.enabled)
    }

    fn estimate_damage(
        &self,
        cmd: &CombatManagerData,
        player: &CombatPlayer,
        enemy: &CombatEnemy,
    ) -> f32 {
        let (_, max_roll) = cmd.damage_roll_bounds();
        let (base, timed) = damage::basic_attack_damage(player, enemy, max_roll);
        if self.timed {
            damage::round_damage(base + timed)
        } else {
            damage::round_damage(base)
        }
    }
}
