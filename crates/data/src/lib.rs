pub mod characters;
pub mod items;
pub mod level_up_tables;

pub mod prelude {
    pub use crate::characters::*;
    pub use crate::items::*;
}

use characters::PlayerPartyCharacter;

#[derive(Default, PartialEq, Debug, Eq, PartialOrd, Ord, Clone)]
pub enum ItemType {
    Valuable,
    Weapon,
    Armor,
    Trinket,
    GroupTrinket,
    Snack,
    Key,
    Ingredient,
    Recipe,
    Relic,
    StoryArtifact, // Teaks Item
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Default)]
pub struct Item {
    pub guid: &'static str,
    pub item_type: ItemType,
    pub name: &'static str,
    pub order_priority: u32,
    pub buy_price: u32,
    pub sell_price: u32,
    pub max_quantity: u32,
    pub physical_attack: u32,
    pub magical_attack: u32,
    pub physical_defense: u32,
    pub magical_defense: u32,
    pub hp_to_restore: u32,
    pub mp_to_restore: u32,
    pub hp_use_percent: u32,
    pub mp_use_percent: u32,
    pub hp_percent_to_restore: f32,
    pub mp_percent_to_restore: f32,
    pub equippable_by: Option<&'static [PlayerPartyCharacter]>,
}

#[allow(dead_code)]
impl Item {
    fn equippable_by_character(self, character: PlayerPartyCharacter) -> bool {
        if let Some(characters) = self.equippable_by {
            characters.iter().any(|c| *c == character)
        } else {
            false
        }
    }
}
