pub mod bst;
pub mod garl;
pub mod reshan;
pub mod serai;
pub mod valere;
pub mod zale;

use crate::prelude::PlayerPartyCharacter;

pub fn get_sum_of_character_stats_by_level(
    character: PlayerPartyCharacter,
    level: u32,
) -> LevelUpStats {
    match character {
        PlayerPartyCharacter::Valere => self::valere::stats_table().get(&level).unwrap().clone(),
        PlayerPartyCharacter::Zale => self::valere::stats_table().get(&level).unwrap().clone(),
        PlayerPartyCharacter::Bst => self::bst::stats_table().get(&level).unwrap().clone(),
        PlayerPartyCharacter::Garl => self::garl::stats_table().get(&level).unwrap().clone(),
        PlayerPartyCharacter::Serai => self::serai::stats_table().get(&level).unwrap().clone(),
        PlayerPartyCharacter::Reshan => self::reshan::stats_table().get(&level).unwrap().clone(),
        _ => LevelUpStats::default().clone(),
    }
}

#[derive(Default, Clone)]
pub struct LevelUpStats {
    pub hp: u32,
    pub mp: u32,
    pub physical_attack: u32,
    pub physical_defense: u32,
    pub magical_attack: u32,
    pub magical_defense: u32,
}
