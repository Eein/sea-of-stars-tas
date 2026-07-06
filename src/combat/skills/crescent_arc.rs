//! Valere — Crescent Arc. A Moon-element magic attack (hits multiple enemies).

use data::prelude::PlayerPartyCharacter;

use super::{Action, SkillResource, TargetType};
use crate::combat::damage;
use crate::memory::combat_manager::{CombatDamageType, CombatEnemy, CombatPlayer};

pub struct CrescentArc;

impl Action for CrescentArc {
    fn character(&self) -> PlayerPartyCharacter {
        PlayerPartyCharacter::Valere
    }

    fn internal_name(&self) -> &'static str {
        "CrescentArc"
    }

    fn target_type(&self) -> TargetType {
        TargetType::Aoe
    }

    fn resource(&self) -> SkillResource {
        SkillResource::Mana
    }

    fn cost(&self) -> u32 {
        6
    }

    fn damage_types(&self) -> Vec<CombatDamageType> {
        vec![CombatDamageType::Moon]
    }

    fn estimate_damage(&self, player: &CombatPlayer, enemy: &CombatEnemy) -> f32 {
        damage::magic_damage_estimate(player, enemy, CombatDamageType::Moon)
    }
}
