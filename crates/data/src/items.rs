use super::PlayerPartyCharacter;

#[derive(Default, Debug)]
pub enum ItemType {
    Valuable,
    Weapon,
    Armor,
    Trinket,
    GroupTrinket,
    Food,
    Key,
    Ingredient,
    Recipe,
    Relic,
    #[default]
    Unknown,
}

#[derive(Debug, Default)]
pub struct Item {
    pub guid: &'static str,
    pub item_type: ItemType,
    pub name: &'static str,
    pub order_prio: u32,
    pub cost: u32,
    pub sell_value: u32,
    pub physical_attack: u32,
    pub magical_attack: u32,
    pub physical_defense: u32,
    pub magical_defense: u32,
    pub equippable_by: Option<&'static [PlayerPartyCharacter]>
}

use std::collections::HashMap;
use std::sync::OnceLock;

pub fn armor() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(
            "AdventurersVest",
            Item {
                guid: "3ac3907f841cc2a40bd0fdce51cd52e2",
                item_type: ItemType::Armor,
                name: "Adventurer's Vest",
                order_prio: 0,
                cost: 24,
                sell_value: 6,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 5,
                magical_defense: 5,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                ]),
            },
        );
        m
    })
}
