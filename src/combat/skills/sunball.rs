//! Zale — Sunball. A Sun-element magic attack.

use data::prelude::PlayerPartyCharacter;

use super::{Action, SkillResource, TimingType};
use crate::combat::damage;
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
        damage::magic_damage_estimate(player, enemy, CombatDamageType::Sun)
    }
}
