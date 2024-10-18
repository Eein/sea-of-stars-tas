use super::{Item, ItemType};
use crate::prelude::PlayerPartyCharacter;

use std::collections::HashMap;
use std::sync::OnceLock;

pub fn story_artifacts() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        all_items()
            .iter()
            .filter(|(_k, v)| v.item_type == ItemType::StoryArtifact)
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<_, _>>()
    })
}

pub fn recipes() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        all_items()
            .iter()
            .filter(|(_k, v)| v.item_type == ItemType::Recipe)
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<_, _>>()
    })
}

pub fn ingredients() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        all_items()
            .iter()
            .filter(|(_k, v)| v.item_type == ItemType::Ingredient)
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<_, _>>()
    })
}

pub fn snacks() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        all_items()
            .iter()
            .filter(|(_k, v)| v.item_type == ItemType::Snack)
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<_, _>>()
    })
}

pub fn group_trinkets() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        all_items()
            .iter()
            .filter(|(_k, v)| v.item_type == ItemType::GroupTrinket)
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<_, _>>()
    })
}

pub fn trinkets() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        all_items()
            .iter()
            .filter(|(_k, v)| v.item_type == ItemType::Trinket)
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<_, _>>()
    })
}

pub fn valuables() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        all_items()
            .iter()
            .filter(|(_k, v)| v.item_type == ItemType::Valuable)
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<_, _>>()
    })
}

pub fn key_items() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        all_items()
            .iter()
            .filter(|(_k, v)| v.item_type == ItemType::Key)
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<_, _>>()
    })
}

pub fn weapons() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        all_items()
            .iter()
            .filter(|(_k, v)| v.item_type == ItemType::Weapon)
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<_, _>>()
    })
}

pub fn armor() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        all_items()
            .iter()
            .filter(|(_k, v)| v.item_type == ItemType::Armor)
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<_, _>>()
    })
}

pub fn all_items() -> &'static HashMap<&'static str, Item> {
    static HASHMAP: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        // TRINKET_ABACUS_NAME
        m.insert(
            "Abacus",
            Item {
                guid: "6a39fd9b315b53a4faa9e3736d820eff",
                item_type: ItemType::Trinket,
                name: "Abacus",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 36,
                sell_price: 9,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "6a39fd9b315b53a4faa9e3736d820eff",
            Item {
                guid: "6a39fd9b315b53a4faa9e3736d820eff",
                item_type: ItemType::Trinket,
                name: "Abacus",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 36,
                sell_price: 9,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_ADAMANTINESTAFF_NAME
        m.insert(
            "Adamantine Staff",
            Item {
                guid: "5d108fe330c777241bb2b7c5326acc54",
                item_type: ItemType::Weapon,
                name: "Adamantine Staff",
                order_priority: 135,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 39,
                magical_attack: 23,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "5d108fe330c777241bb2b7c5326acc54",
            Item {
                guid: "5d108fe330c777241bb2b7c5326acc54",
                item_type: ItemType::Weapon,
                name: "Adamantine Staff",
                order_priority: 135,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 39,
                magical_attack: 23,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_AETHERWOODCORK_NAME
        m.insert(
            "Aetherwood Cork",
            Item {
                guid: "807d9f5c8b8f5514abcf261a36dc1711",
                item_type: ItemType::Weapon,
                name: "Aetherwood Cork",
                order_priority: 215,
                max_quantity: 255,
                buy_price: 172,
                sell_price: 43,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 44,
                magical_attack: 49,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "807d9f5c8b8f5514abcf261a36dc1711",
            Item {
                guid: "807d9f5c8b8f5514abcf261a36dc1711",
                item_type: ItemType::Weapon,
                name: "Aetherwood Cork",
                order_priority: 215,
                max_quantity: 255,
                buy_price: 172,
                sell_price: 43,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 44,
                magical_attack: 49,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_ADVENRURERVEST_NAME
        m.insert(
            "Adventurer's Vest",
            Item {
                guid: "3ac3907f841cc2a40bd0fdce51cd52e2",
                item_type: ItemType::Armor,
                name: "Adventurer's Vest",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 24,
                sell_price: 6,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 5,
                magical_defense: 5,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "3ac3907f841cc2a40bd0fdce51cd52e2",
            Item {
                guid: "3ac3907f841cc2a40bd0fdce51cd52e2",
                item_type: ItemType::Armor,
                name: "Adventurer's Vest",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 24,
                sell_price: 6,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 5,
                magical_defense: 5,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_AMULETONBOARDING_NAME
        m.insert(
            "Amulet of Storytelling",
            Item {
                guid: "b6acbe8e6e78e014d830094e248870af",
                item_type: ItemType::Trinket,
                name: "Amulet of Storytelling",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b6acbe8e6e78e014d830094e248870af",
            Item {
                guid: "b6acbe8e6e78e014d830094e248870af",
                item_type: ItemType::Trinket,
                name: "Amulet of Storytelling",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_APOGEEDAGGERS_NAME
        m.insert(
            "Apogee Daggers",
            Item {
                guid: "a164640364ccf3546b0b9a0ac82b59cf",
                item_type: ItemType::Weapon,
                name: "Apogee Daggers",
                order_priority: 185,
                max_quantity: 255,
                buy_price: 184,
                sell_price: 46,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 41,
                magical_attack: 35,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a164640364ccf3546b0b9a0ac82b59cf",
            Item {
                guid: "a164640364ccf3546b0b9a0ac82b59cf",
                item_type: ItemType::Weapon,
                name: "Apogee Daggers",
                order_priority: 185,
                max_quantity: 255,
                buy_price: 184,
                sell_price: 46,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 41,
                magical_attack: 35,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_ARCANE_AMULET_NAME
        m.insert(
            "Arcane Amulet",
            Item {
                guid: "d8a7bdab9aa9a6e4e95036b0eff55239",
                item_type: ItemType::Trinket,
                name: "Arcane Amulet",
                order_priority: 15,
                max_quantity: 255,
                buy_price: 200,
                sell_price: 50,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d8a7bdab9aa9a6e4e95036b0eff55239",
            Item {
                guid: "d8a7bdab9aa9a6e4e95036b0eff55239",
                item_type: ItemType::Trinket,
                name: "Arcane Amulet",
                order_priority: 15,
                max_quantity: 255,
                buy_price: 200,
                sell_price: 50,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_ASSASSINSPIN_NAME
        m.insert(
            "Assassin's Pin",
            Item {
                guid: "88810f8353ed7404bb576582cc39189f",
                item_type: ItemType::Trinket,
                name: "Assassin's Pin",
                order_priority: 50,
                max_quantity: 255,
                buy_price: 328,
                sell_price: 82,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "88810f8353ed7404bb576582cc39189f",
            Item {
                guid: "88810f8353ed7404bb576582cc39189f",
                item_type: ItemType::Trinket,
                name: "Assassin's Pin",
                order_priority: 50,
                max_quantity: 255,
                buy_price: 328,
                sell_price: 82,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_BAMBOOSLICER_NAME
        m.insert(
            "Bamboo Slicer",
            Item {
                guid: "0a9e0ac0bb822b349af356af38ad6a5d",
                item_type: ItemType::Weapon,
                name: "Bamboo Slicer",
                order_priority: 95,
                max_quantity: 255,
                buy_price: 140,
                sell_price: 35,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 22,
                magical_attack: 19,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0a9e0ac0bb822b349af356af38ad6a5d",
            Item {
                guid: "0a9e0ac0bb822b349af356af38ad6a5d",
                item_type: ItemType::Weapon,
                name: "Bamboo Slicer",
                order_priority: 95,
                max_quantity: 255,
                buy_price: 140,
                sell_price: 35,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 22,
                magical_attack: 19,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_AZURECAPE_NAME
        m.insert(
            "Azure Cape",
            Item {
                guid: "722415b4304ba6444b3f86b7b277d5c0",
                item_type: ItemType::Armor,
                name: "Azure Cape",
                order_priority: 180,
                max_quantity: 255,
                buy_price: 128,
                sell_price: 32,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 33,
                magical_defense: 39,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "722415b4304ba6444b3f86b7b277d5c0",
            Item {
                guid: "722415b4304ba6444b3f86b7b277d5c0",
                item_type: ItemType::Armor,
                name: "Azure Cape",
                order_priority: 180,
                max_quantity: 255,
                buy_price: 128,
                sell_price: 32,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 33,
                magical_defense: 39,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // Armor_BasicArmor_Name
        m.insert(
            "Basic Armor",
            Item {
                guid: "a31ee5ffc1b693148be7c48150ebff81",
                item_type: ItemType::Armor,
                name: "Basic Armor",
                order_priority: 5,
                max_quantity: 255,
                buy_price: 28,
                sell_price: 7,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 13,
                magical_defense: 9,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a31ee5ffc1b693148be7c48150ebff81",
            Item {
                guid: "a31ee5ffc1b693148be7c48150ebff81",
                item_type: ItemType::Armor,
                name: "Basic Armor",
                order_priority: 5,
                max_quantity: 255,
                buy_price: 28,
                sell_price: 7,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 13,
                magical_defense: 9,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_BLUE_LEAF_NAME
        m.insert(
            "Blue Leaf",
            Item {
                guid: "588f0765e48c5124a8d9f671b57db116",
                item_type: ItemType::Trinket,
                name: "Blue Leaf",
                order_priority: 45,
                max_quantity: 255,
                buy_price: 220,
                sell_price: 55,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "588f0765e48c5124a8d9f671b57db116",
            Item {
                guid: "588f0765e48c5124a8d9f671b57db116",
                item_type: ItemType::Trinket,
                name: "Blue Leaf",
                order_priority: 45,
                max_quantity: 255,
                buy_price: 220,
                sell_price: 55,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_BONEARMOR_NAME
        m.insert(
            "Bone Armor",
            Item {
                guid: "581aaf64cd0270b4bbf12d9ce9ea4ec0",
                item_type: ItemType::Armor,
                name: "Bone Armor",
                order_priority: 125,
                max_quantity: 255,
                buy_price: 48,
                sell_price: 12,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 22,
                magical_defense: 20,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "581aaf64cd0270b4bbf12d9ce9ea4ec0",
            Item {
                guid: "581aaf64cd0270b4bbf12d9ce9ea4ec0",
                item_type: ItemType::Armor,
                name: "Bone Armor",
                order_priority: 125,
                max_quantity: 255,
                buy_price: 48,
                sell_price: 12,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 22,
                magical_defense: 20,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_CALCITELID_NAME
        m.insert(
            "Calcite Lid",
            Item {
                guid: "108a40870288c934d94c1661286ff1f0",
                item_type: ItemType::Weapon,
                name: "Calcite Lid",
                order_priority: 65,
                max_quantity: 255,
                buy_price: 56,
                sell_price: 14,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 15,
                magical_attack: 13,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "108a40870288c934d94c1661286ff1f0",
            Item {
                guid: "108a40870288c934d94c1661286ff1f0",
                item_type: ItemType::Weapon,
                name: "Calcite Lid",
                order_priority: 65,
                max_quantity: 255,
                buy_price: 56,
                sell_price: 14,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 15,
                magical_attack: 13,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_BOXERSHORTS_NAME
        m.insert(
            "Leather Pauldrons",
            Item {
                guid: "a1d886ffc9682c54ebda876deeed58b4",
                item_type: ItemType::Armor,
                name: "Leather Pauldrons",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 60,
                sell_price: 35,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a1d886ffc9682c54ebda876deeed58b4",
            Item {
                guid: "a1d886ffc9682c54ebda876deeed58b4",
                item_type: ItemType::Armor,
                name: "Leather Pauldrons",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 60,
                sell_price: 35,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_GARL_NAME
        m.insert(
            "Cauldron Lid",
            Item {
                guid: "b9995b298ada14e4887faf134942b564",
                item_type: ItemType::Weapon,
                name: "Cauldron Lid",
                order_priority: 5,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 7,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 6,
                magical_attack: 7,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b9995b298ada14e4887faf134942b564",
            Item {
                guid: "b9995b298ada14e4887faf134942b564",
                item_type: ItemType::Weapon,
                name: "Cauldron Lid",
                order_priority: 5,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 7,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 6,
                magical_attack: 7,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_CELESTIALRAY_NAME
        m.insert(
            "Celestial Ray",
            Item {
                guid: "23cd65231a0ab0c45be5987e87455136",
                item_type: ItemType::Trinket,
                name: "Celestial Ray",
                order_priority: 65,
                max_quantity: 255,
                buy_price: 220,
                sell_price: 55,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "23cd65231a0ab0c45be5987e87455136",
            Item {
                guid: "23cd65231a0ab0c45be5987e87455136",
                item_type: ItemType::Trinket,
                name: "Celestial Ray",
                order_priority: 65,
                max_quantity: 255,
                buy_price: 220,
                sell_price: 55,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_CLOUDSWORD_NAME
        m.insert(
            "Cloud Sword",
            Item {
                guid: "5877570357bf9f441ae01cd06bccd9f4",
                item_type: ItemType::Weapon,
                name: "Cloud Sword",
                order_priority: 115,
                max_quantity: 255,
                buy_price: 156,
                sell_price: 39,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 29,
                magical_attack: 24,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "5877570357bf9f441ae01cd06bccd9f4",
            Item {
                guid: "5877570357bf9f441ae01cd06bccd9f4",
                item_type: ItemType::Weapon,
                name: "Cloud Sword",
                order_priority: 115,
                max_quantity: 255,
                buy_price: 156,
                sell_price: 39,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 29,
                magical_attack: 24,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_CLOUDYSIMULACRUM_NAME
        m.insert(
            "Cloudy Simulacrum",
            Item {
                guid: "2e09170437a631e46ac9b9669464618c",
                item_type: ItemType::Armor,
                name: "Cloudy Simulacrum",
                order_priority: 210,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[PlayerPartyCharacter::Bst]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 45,
                magical_defense: 42,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "2e09170437a631e46ac9b9669464618c",
            Item {
                guid: "2e09170437a631e46ac9b9669464618c",
                item_type: ItemType::Armor,
                name: "Cloudy Simulacrum",
                order_priority: 210,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[PlayerPartyCharacter::Bst]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 45,
                magical_defense: 42,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // CONCHITEM_RAINBOWSTAR_NAME
        m.insert(
            "Rainbow Star",
            Item {
                guid: "d810ad9125b56864b92ea9b3bdadad51",
                item_type: ItemType::Key,
                name: "Rainbow Star",
                order_priority: 11020,
                max_quantity: 1,
                buy_price: 1,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d810ad9125b56864b92ea9b3bdadad51",
            Item {
                guid: "d810ad9125b56864b92ea9b3bdadad51",
                item_type: ItemType::Key,
                name: "Rainbow Star",
                order_priority: 11020,
                max_quantity: 1,
                buy_price: 1,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_COPPER_STAFF_NAME
        m.insert(
            "Copper Staff",
            Item {
                guid: "10d0cb7c4836f794bb6f4d965f2b76c8",
                item_type: ItemType::Weapon,
                name: "Copper Staff",
                order_priority: 15,
                max_quantity: 255,
                buy_price: 36,
                sell_price: 9,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 7,
                magical_attack: 4,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "10d0cb7c4836f794bb6f4d965f2b76c8",
            Item {
                guid: "10d0cb7c4836f794bb6f4d965f2b76c8",
                item_type: ItemType::Weapon,
                name: "Copper Staff",
                order_priority: 15,
                max_quantity: 255,
                buy_price: 36,
                sell_price: 9,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 7,
                magical_attack: 4,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_CORALDAGGERS_NAME
        m.insert(
            "Coral Daggers",
            Item {
                guid: "d78f39987eaedcc42b938fc1763bba59",
                item_type: ItemType::Weapon,
                name: "Coral Daggers",
                order_priority: 75,
                max_quantity: 255,
                buy_price: 124,
                sell_price: 31,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 14,
                magical_attack: 20,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d78f39987eaedcc42b938fc1763bba59",
            Item {
                guid: "d78f39987eaedcc42b938fc1763bba59",
                item_type: ItemType::Weapon,
                name: "Coral Daggers",
                order_priority: 75,
                max_quantity: 255,
                buy_price: 124,
                sell_price: 31,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 14,
                magical_attack: 20,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_CORALSWORD_NAME
        m.insert(
            "Coral Sword",
            Item {
                guid: "16d199bda90813f40a28fd9203865374",
                item_type: ItemType::Weapon,
                name: "Coral Sword",
                order_priority: 70,
                max_quantity: 255,
                buy_price: 112,
                sell_price: 28,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 19,
                magical_attack: 15,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "16d199bda90813f40a28fd9203865374",
            Item {
                guid: "16d199bda90813f40a28fd9203865374",
                item_type: ItemType::Weapon,
                name: "Coral Sword",
                order_priority: 70,
                max_quantity: 255,
                buy_price: 112,
                sell_price: 28,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 19,
                magical_attack: 15,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // GROUP_TRINKET_CORNUCOPIA_NAME
        m.insert(
            "Cornucopia",
            Item {
                guid: "fb067e7ba3b744342bbca13afcfb2a50",
                item_type: ItemType::GroupTrinket,
                name: "Cornucopia",
                order_priority: 35,
                max_quantity: 255,
                buy_price: 300,
                sell_price: 0,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "fb067e7ba3b744342bbca13afcfb2a50",
            Item {
                guid: "fb067e7ba3b744342bbca13afcfb2a50",
                item_type: ItemType::GroupTrinket,
                name: "Cornucopia",
                order_priority: 35,
                max_quantity: 255,
                buy_price: 300,
                sell_price: 0,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_COSMICCAPE_NAME
        m.insert(
            "Cosmic Cape",
            Item {
                guid: "d1d1bcd5dbc41d648a812fbc7d1861b2",
                item_type: ItemType::Armor,
                name: "Cosmic Cape",
                order_priority: 215,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 45,
                magical_defense: 50,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d1d1bcd5dbc41d648a812fbc7d1861b2",
            Item {
                guid: "d1d1bcd5dbc41d648a812fbc7d1861b2",
                item_type: ItemType::Armor,
                name: "Cosmic Cape",
                order_priority: 215,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 45,
                magical_defense: 50,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_CYPRESSCORK_NAME
        m.insert(
            "Cypress Cork",
            Item {
                guid: "1a8eae5901ae6754a8899a58a692a2b3",
                item_type: ItemType::Weapon,
                name: "Cypress Cork",
                order_priority: 130,
                max_quantity: 255,
                buy_price: 164,
                sell_price: 41,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 26,
                magical_attack: 32,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "1a8eae5901ae6754a8899a58a692a2b3",
            Item {
                guid: "1a8eae5901ae6754a8899a58a692a2b3",
                item_type: ItemType::Weapon,
                name: "Cypress Cork",
                order_priority: 130,
                max_quantity: 255,
                buy_price: 164,
                sell_price: 41,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 26,
                magical_attack: 32,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_DEXTROUS_BANGLE_NAME
        m.insert(
            "Dextrous Bangle",
            Item {
                guid: "f93c6f846bc2f0b4484e896221759b94",
                item_type: ItemType::Trinket,
                name: "Dextrous Bangle",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 150,
                sell_price: 38,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f93c6f846bc2f0b4484e896221759b94",
            Item {
                guid: "f93c6f846bc2f0b4484e896221759b94",
                item_type: ItemType::Trinket,
                name: "Dextrous Bangle",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 150,
                sell_price: 38,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_DOCARRIARMOR_NAME
        m.insert(
            "Docarri Armor",
            Item {
                guid: "98223d461dbc41a409926f2a1409f7ec",
                item_type: ItemType::Armor,
                name: "Docarri Armor",
                order_priority: 140,
                max_quantity: 255,
                buy_price: 92,
                sell_price: 23,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 28,
                magical_defense: 22,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "98223d461dbc41a409926f2a1409f7ec",
            Item {
                guid: "98223d461dbc41a409926f2a1409f7ec",
                item_type: ItemType::Armor,
                name: "Docarri Armor",
                order_priority: 140,
                max_quantity: 255,
                buy_price: 92,
                sell_price: 23,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 28,
                magical_defense: 22,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_DULLSIMULACRUM_NAME
        m.insert(
            "Dull Simulacrum",
            Item {
                guid: "e26fa36b128e76041938bf3c86ad8eef",
                item_type: ItemType::Armor,
                name: "Dull Simulacrum",
                order_priority: 195,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[PlayerPartyCharacter::Bst]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 37,
                magical_defense: 37,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e26fa36b128e76041938bf3c86ad8eef",
            Item {
                guid: "e26fa36b128e76041938bf3c86ad8eef",
                item_type: ItemType::Armor,
                name: "Dull Simulacrum",
                order_priority: 195,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[PlayerPartyCharacter::Bst]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 37,
                magical_defense: 37,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_EARTHSHINESTAFF_NAME
        m.insert(
            "Earthshine Staff",
            Item {
                guid: "27a72590849651b4ba1cf80fe6be8d4a",
                item_type: ItemType::Weapon,
                name: "Earthshine Staff",
                order_priority: 175,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 47,
                magical_attack: 25,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "27a72590849651b4ba1cf80fe6be8d4a",
            Item {
                guid: "27a72590849651b4ba1cf80fe6be8d4a",
                item_type: ItemType::Weapon,
                name: "Earthshine Staff",
                order_priority: 175,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 47,
                magical_attack: 25,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_ECLIPSEARMOR_NAME
        m.insert(
            "Eclipse Armor",
            Item {
                guid: "911324cc7daad5b41a0af317180a42db",
                item_type: ItemType::Armor,
                name: "Eclipse Armor",
                order_priority: 225,
                max_quantity: 255,
                buy_price: 152,
                sell_price: 38,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 53,
                magical_defense: 49,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "911324cc7daad5b41a0af317180a42db",
            Item {
                guid: "911324cc7daad5b41a0af317180a42db",
                item_type: ItemType::Armor,
                name: "Eclipse Armor",
                order_priority: 225,
                max_quantity: 255,
                buy_price: 152,
                sell_price: 38,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 53,
                magical_defense: 49,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_ENCHANTEDCHAINLINK_NAME
        m.insert(
            "Enchanted Chain-Link",
            Item {
                guid: "d311de20fa85a5148a7b523f0af76f47",
                item_type: ItemType::Trinket,
                name: "Enchanted Chain-Link",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 150,
                sell_price: 120,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d311de20fa85a5148a7b523f0af76f47",
            Item {
                guid: "d311de20fa85a5148a7b523f0af76f47",
                item_type: ItemType::Trinket,
                name: "Enchanted Chain-Link",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 150,
                sell_price: 120,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_ENCHANTED_SCARF_NAME
        m.insert(
            "Enchanted Scarf",
            Item {
                guid: "9e03ef3b70751944abae3f3880c733b7",
                item_type: ItemType::Trinket,
                name: "Enchanted Scarf",
                order_priority: 30,
                max_quantity: 255,
                buy_price: 190,
                sell_price: 45,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "9e03ef3b70751944abae3f3880c733b7",
            Item {
                guid: "9e03ef3b70751944abae3f3880c733b7",
                item_type: ItemType::Trinket,
                name: "Enchanted Scarf",
                order_priority: 30,
                max_quantity: 255,
                buy_price: 190,
                sell_price: 45,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_EVERGREEN_LEAF_NAME
        m.insert(
            "Evergreen Leaf",
            Item {
                guid: "af3e10fe5b7abd940baadad96a4e5e7c",
                item_type: ItemType::Trinket,
                name: "Evergreen Leaf",
                order_priority: 40,
                max_quantity: 255,
                buy_price: 220,
                sell_price: 55,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "af3e10fe5b7abd940baadad96a4e5e7c",
            Item {
                guid: "af3e10fe5b7abd940baadad96a4e5e7c",
                item_type: ItemType::Trinket,
                name: "Evergreen Leaf",
                order_priority: 40,
                max_quantity: 255,
                buy_price: 220,
                sell_price: 55,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_EYEYOMARA_NAME
        m.insert(
            "Eye of Yomara",
            Item {
                guid: "e7e2fc32454eda44d9d93ba6009d8752",
                item_type: ItemType::Trinket,
                name: "Eye of Yomara",
                order_priority: 15,
                max_quantity: 255,
                buy_price: 888,
                sell_price: 87,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e7e2fc32454eda44d9d93ba6009d8752",
            Item {
                guid: "e7e2fc32454eda44d9d93ba6009d8752",
                item_type: ItemType::Trinket,
                name: "Eye of Yomara",
                order_priority: 15,
                max_quantity: 255,
                buy_price: 888,
                sell_price: 87,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_FIRMAMENTSEDGE_NAME
        m.insert(
            "Firmament's Edge",
            Item {
                guid: "7d6c888ffacc1cd4ab3ce414b294c67e",
                item_type: ItemType::Weapon,
                name: "Firmament's Edge",
                order_priority: 180,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 42,
                magical_attack: 32,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "7d6c888ffacc1cd4ab3ce414b294c67e",
            Item {
                guid: "7d6c888ffacc1cd4ab3ce414b294c67e",
                item_type: ItemType::Weapon,
                name: "Firmament's Edge",
                order_priority: 180,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 42,
                magical_attack: 32,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MAGICHOOK_NAME
        m.insert(
            "Hook of Time",
            Item {
                guid: "7b4c2ced735d9ff41ad7fae6e7dc71d6",
                item_type: ItemType::Key,
                name: "Hook of Time",
                order_priority: 11045,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "7b4c2ced735d9ff41ad7fae6e7dc71d6",
            Item {
                guid: "7b4c2ced735d9ff41ad7fae6e7dc71d6",
                item_type: ItemType::Key,
                name: "Hook of Time",
                order_priority: 11045,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_PASSVIP_NAME
        m.insert(
            "Gilded Invitation",
            Item {
                guid: "7ba76f034da41cd4d9de1c4267a031ef",
                item_type: ItemType::Key,
                name: "Gilded Invitation",
                order_priority: 11050,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "7ba76f034da41cd4d9de1c4267a031ef",
            Item {
                guid: "7ba76f034da41cd4d9de1c4267a031ef",
                item_type: ItemType::Key,
                name: "Gilded Invitation",
                order_priority: 11050,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_GAMBLEREARRING_NAME
        m.insert(
            "Gambler's Earring",
            Item {
                guid: "478d680b15f759d44987b2cef327ce2f",
                item_type: ItemType::Trinket,
                name: "Gambler's Earring",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "478d680b15f759d44987b2cef327ce2f",
            Item {
                guid: "478d680b15f759d44987b2cef327ce2f",
                item_type: ItemType::Trinket,
                name: "Gambler's Earring",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_CORALSTAFF_NAME
        m.insert(
            "Coral Staff",
            Item {
                guid: "70a7707a760f40046b71b4b2e111a708",
                item_type: ItemType::Weapon,
                name: "Coral Staff",
                order_priority: 85,
                max_quantity: 255,
                buy_price: 112,
                sell_price: 28,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 22,
                magical_attack: 10,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "70a7707a760f40046b71b4b2e111a708",
            Item {
                guid: "70a7707a760f40046b71b4b2e111a708",
                item_type: ItemType::Weapon,
                name: "Coral Staff",
                order_priority: 85,
                max_quantity: 255,
                buy_price: 112,
                sell_price: 28,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 22,
                magical_attack: 10,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_GREEN_LEAF_NAME
        m.insert(
            "Green Leaf",
            Item {
                guid: "0ee892db1da195541bcbe578898265b5",
                item_type: ItemType::Trinket,
                name: "Green Leaf",
                order_priority: 5,
                max_quantity: 255,
                buy_price: 28,
                sell_price: 7,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0ee892db1da195541bcbe578898265b5",
            Item {
                guid: "0ee892db1da195541bcbe578898265b5",
                item_type: ItemType::Trinket,
                name: "Green Leaf",
                order_priority: 5,
                max_quantity: 255,
                buy_price: 28,
                sell_price: 7,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_GARLSAPRON_NAME
        m.insert(
            "Garl's Apron",
            Item {
                guid: "86737d6ef697c204187aea8533ec4244",
                item_type: ItemType::Armor,
                name: "Garl's Apron",
                order_priority: 224,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 50,
                magical_defense: 50,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "86737d6ef697c204187aea8533ec4244",
            Item {
                guid: "86737d6ef697c204187aea8533ec4244",
                item_type: ItemType::Armor,
                name: "Garl's Apron",
                order_priority: 224,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 50,
                magical_defense: 50,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_HELIACALEARRINGS_NAME
        m.insert(
            "Heliacal Earrings",
            Item {
                guid: "acadc7386a182444692a4622393aac6b",
                item_type: ItemType::Trinket,
                name: "Heliacal Earrings",
                order_priority: 55,
                max_quantity: 255,
                buy_price: 200,
                sell_price: 50,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "acadc7386a182444692a4622393aac6b",
            Item {
                guid: "acadc7386a182444692a4622393aac6b",
                item_type: ItemType::Trinket,
                name: "Heliacal Earrings",
                order_priority: 55,
                max_quantity: 255,
                buy_price: 200,
                sell_price: 50,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_IGNEOUSSTAFF_NAME
        m.insert(
            "Igneous Staff",
            Item {
                guid: "e93c519d3c18fa245937cc461e4de24d",
                item_type: ItemType::Weapon,
                name: "Igneous Staff",
                order_priority: 120,
                max_quantity: 255,
                buy_price: 156,
                sell_price: 39,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 32,
                magical_attack: 20,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e93c519d3c18fa245937cc461e4de24d",
            Item {
                guid: "e93c519d3c18fa245937cc461e4de24d",
                item_type: ItemType::Weapon,
                name: "Igneous Staff",
                order_priority: 120,
                max_quantity: 255,
                buy_price: 156,
                sell_price: 39,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 32,
                magical_attack: 20,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_APPLE_NAME
        m.insert(
            "Apple",
            Item {
                guid: "1050b67b61f94084d970b6ae4c33dc01",
                item_type: ItemType::Ingredient,
                name: "Apple",
                order_priority: 140,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "1050b67b61f94084d970b6ae4c33dc01",
            Item {
                guid: "1050b67b61f94084d970b6ae4c33dc01",
                item_type: ItemType::Ingredient,
                name: "Apple",
                order_priority: 140,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_EGG_NAME
        m.insert(
            "Egg",
            Item {
                guid: "a17b0505c55affc45a066431eb30c655",
                item_type: ItemType::Ingredient,
                name: "Egg",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 6,
                sell_price: 3,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a17b0505c55affc45a066431eb30c655",
            Item {
                guid: "a17b0505c55affc45a066431eb30c655",
                item_type: ItemType::Ingredient,
                name: "Egg",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 6,
                sell_price: 3,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_FISH_NAME
        m.insert(
            "Fish",
            Item {
                guid: "7f49bbd04a7b4b84092aec1d8f54d1cb",
                item_type: ItemType::Ingredient,
                name: "Fish",
                order_priority: 60,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "7f49bbd04a7b4b84092aec1d8f54d1cb",
            Item {
                guid: "7f49bbd04a7b4b84092aec1d8f54d1cb",
                item_type: ItemType::Ingredient,
                name: "Fish",
                order_priority: 60,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_BONE_NAME
        m.insert(
            "Bone",
            Item {
                guid: "9a6abc7114f0eb9409f27b6816830fff",
                item_type: ItemType::Ingredient,
                name: "Bone",
                order_priority: 40,
                max_quantity: 255,
                buy_price: 6,
                sell_price: 3,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "9a6abc7114f0eb9409f27b6816830fff",
            Item {
                guid: "9a6abc7114f0eb9409f27b6816830fff",
                item_type: ItemType::Ingredient,
                name: "Bone",
                order_priority: 40,
                max_quantity: 255,
                buy_price: 6,
                sell_price: 3,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_GRAINS_NAME
        m.insert(
            "Grains",
            Item {
                guid: "841382ac016092346b0cf860a932c3c7",
                item_type: ItemType::Ingredient,
                name: "Grains",
                order_priority: 35,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "841382ac016092346b0cf860a932c3c7",
            Item {
                guid: "841382ac016092346b0cf860a932c3c7",
                item_type: ItemType::Ingredient,
                name: "Grains",
                order_priority: 35,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_LETTUCE_NAME
        m.insert(
            "Lettuce",
            Item {
                guid: "21471472fcbc68149b8cf7a9dac2b785",
                item_type: ItemType::Ingredient,
                name: "Lettuce",
                order_priority: 120,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "21471472fcbc68149b8cf7a9dac2b785",
            Item {
                guid: "21471472fcbc68149b8cf7a9dac2b785",
                item_type: ItemType::Ingredient,
                name: "Lettuce",
                order_priority: 120,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_MEAT_NAME
        m.insert(
            "Meat",
            Item {
                guid: "0b7efbffd76a27f489ea26df56bbc5ad",
                item_type: ItemType::Ingredient,
                name: "Meat",
                order_priority: 50,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0b7efbffd76a27f489ea26df56bbc5ad",
            Item {
                guid: "0b7efbffd76a27f489ea26df56bbc5ad",
                item_type: ItemType::Ingredient,
                name: "Meat",
                order_priority: 50,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_MAPLESYRUP_NAME
        m.insert(
            "Maple Syrup",
            Item {
                guid: "4b5c4bf16feb4b448bdc3cf8057f2be0",
                item_type: ItemType::Ingredient,
                name: "Maple Syrup",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 8,
                sell_price: 4,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "4b5c4bf16feb4b448bdc3cf8057f2be0",
            Item {
                guid: "4b5c4bf16feb4b448bdc3cf8057f2be0",
                item_type: ItemType::Ingredient,
                name: "Maple Syrup",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 8,
                sell_price: 4,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_MUSHROOM_NAME
        m.insert(
            "Mushroom",
            Item {
                guid: "8a5e759b4475ca04d882e7d2b0c9b47a",
                item_type: ItemType::Ingredient,
                name: "Mushroom",
                order_priority: 110,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "8a5e759b4475ca04d882e7d2b0c9b47a",
            Item {
                guid: "8a5e759b4475ca04d882e7d2b0c9b47a",
                item_type: ItemType::Ingredient,
                name: "Mushroom",
                order_priority: 110,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_MILK_NAME
        m.insert(
            "Dairy",
            Item {
                guid: "b8f81159f93c18740ba72fffdd12a13e",
                item_type: ItemType::Ingredient,
                name: "Dairy",
                order_priority: 30,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b8f81159f93c18740ba72fffdd12a13e",
            Item {
                guid: "b8f81159f93c18740ba72fffdd12a13e",
                item_type: ItemType::Ingredient,
                name: "Dairy",
                order_priority: 30,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_ONION_NAME
        m.insert(
            "Onion",
            Item {
                guid: "4ffc27dc684ae1e48b5a48909f79c3ca",
                item_type: ItemType::Ingredient,
                name: "Onion",
                order_priority: 80,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "4ffc27dc684ae1e48b5a48909f79c3ca",
            Item {
                guid: "4ffc27dc684ae1e48b5a48909f79c3ca",
                item_type: ItemType::Ingredient,
                name: "Onion",
                order_priority: 80,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_PEACH_NAME
        m.insert(
            "Peach",
            Item {
                guid: "47336b026e675b34f99d949b4c2f67d0",
                item_type: ItemType::Ingredient,
                name: "Peach",
                order_priority: 150,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "47336b026e675b34f99d949b4c2f67d0",
            Item {
                guid: "47336b026e675b34f99d949b4c2f67d0",
                item_type: ItemType::Ingredient,
                name: "Peach",
                order_priority: 150,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_PEPPER_NAME
        m.insert(
            "Bell Pepper",
            Item {
                guid: "094de5a55e6f5444aaf870aad7e78297",
                item_type: ItemType::Ingredient,
                name: "Bell Pepper",
                order_priority: 85,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "094de5a55e6f5444aaf870aad7e78297",
            Item {
                guid: "094de5a55e6f5444aaf870aad7e78297",
                item_type: ItemType::Ingredient,
                name: "Bell Pepper",
                order_priority: 85,
                max_quantity: 255,
                buy_price: 4,
                sell_price: 2,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_POTATO_NAME
        m.insert(
            "Potato",
            Item {
                guid: "13dea75b6b52c814b939519cb2b0675c",
                item_type: ItemType::Ingredient,
                name: "Potato",
                order_priority: 90,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "13dea75b6b52c814b939519cb2b0675c",
            Item {
                guid: "13dea75b6b52c814b939519cb2b0675c",
                item_type: ItemType::Ingredient,
                name: "Potato",
                order_priority: 90,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_REDBERRY_NAME
        m.insert(
            "Red Berry",
            Item {
                guid: "1f83e925faef7584b9f34b0723137d76",
                item_type: ItemType::Ingredient,
                name: "Red Berry",
                order_priority: 160,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "1f83e925faef7584b9f34b0723137d76",
            Item {
                guid: "1f83e925faef7584b9f34b0723137d76",
                item_type: ItemType::Ingredient,
                name: "Red Berry",
                order_priority: 160,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_SEAFOOD_NAME
        m.insert(
            "Seafood",
            Item {
                guid: "76b491199a5df8745a77f2a20ae8bc38",
                item_type: ItemType::Ingredient,
                name: "Seafood",
                order_priority: 70,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "76b491199a5df8745a77f2a20ae8bc38",
            Item {
                guid: "76b491199a5df8745a77f2a20ae8bc38",
                item_type: ItemType::Ingredient,
                name: "Seafood",
                order_priority: 70,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_TOMATO_NAME
        m.insert(
            "Tomato",
            Item {
                guid: "f8bf6d10c6c6da840883935fa4aa8c9c",
                item_type: ItemType::Ingredient,
                name: "Tomato",
                order_priority: 130,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f8bf6d10c6c6da840883935fa4aa8c9c",
            Item {
                guid: "f8bf6d10c6c6da840883935fa4aa8c9c",
                item_type: ItemType::Ingredient,
                name: "Tomato",
                order_priority: 130,
                max_quantity: 255,
                buy_price: 2,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // DEFAULTCHESTCUTSCENE_1
        m.insert(
            "INTERNAL[DEFAULT CHEST CUTSCENE]",
            Item {
                guid: "68ab74a0318d0824fa8f2e0426562469",
                item_type: ItemType::Unknown,
                name: "INTERNAL[DEFAULT CHEST CUTSCENE]",
                order_priority: 0,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "68ab74a0318d0824fa8f2e0426562469",
            Item {
                guid: "68ab74a0318d0824fa8f2e0426562469",
                item_type: ItemType::Unknown,
                name: "INTERNAL[DEFAULT CHEST CUTSCENE]",
                order_priority: 0,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WIZARDLAB_COIN_NAME
        m.insert(
            "Coin of Undeath Accord",
            Item {
                guid: "791d1b17113c71a42a4b0a08f5be7676",
                item_type: ItemType::Key,
                name: "Coin of Undeath Accord",
                order_priority: 2040,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "791d1b17113c71a42a4b0a08f5be7676",
            Item {
                guid: "791d1b17113c71a42a4b0a08f5be7676",
                item_type: ItemType::Key,
                name: "Coin of Undeath Accord",
                order_priority: 2040,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WIZARDLAB_REDCRYSTAL_NAME
        m.insert(
            "Red Crystal",
            Item {
                guid: "d0ecc064753be7d48b7818b0113d10f8",
                item_type: ItemType::Key,
                name: "Red Crystal",
                order_priority: 2030,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d0ecc064753be7d48b7818b0113d10f8",
            Item {
                guid: "d0ecc064753be7d48b7818b0113d10f8",
                item_type: ItemType::Key,
                name: "Red Crystal",
                order_priority: 2030,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WELLITEM_NAME
        m.insert(
            "Golden Watch",
            Item {
                guid: "2cb456c00ee26754895cab1ef09f5024",
                item_type: ItemType::Key,
                name: "Golden Watch",
                order_priority: 2035,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "2cb456c00ee26754895cab1ef09f5024",
            Item {
                guid: "2cb456c00ee26754895cab1ef09f5024",
                item_type: ItemType::Key,
                name: "Golden Watch",
                order_priority: 2035,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_AIRELEMENTALHEXAGONALSOCKETKEY_NAME
        m.insert(
            "Hexagonal Slab",
            Item {
                guid: "d605805dcc79f554faf8af072b133c8e",
                item_type: ItemType::Key,
                name: "Hexagonal Slab",
                order_priority: 7000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d605805dcc79f554faf8af072b133c8e",
            Item {
                guid: "d605805dcc79f554faf8af072b133c8e",
                item_type: ItemType::Key,
                name: "Hexagonal Slab",
                order_priority: 7000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_AIRELEMENTALTRIANGULARSOCKETKEY_NAME
        m.insert(
            "Triangular Slab",
            Item {
                guid: "91949467e3e270d459a609213183ecaf",
                item_type: ItemType::Key,
                name: "Triangular Slab",
                order_priority: 7005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "91949467e3e270d459a609213183ecaf",
            Item {
                guid: "91949467e3e270d459a609213183ecaf",
                item_type: ItemType::Key,
                name: "Triangular Slab",
                order_priority: 7005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // VALUABLEITEM_ALUMNITOKEN_NAME
        m.insert(
            "Alumni Token",
            Item {
                guid: "d15dc445af50b0d46b185ca1e3b6d67e",
                item_type: ItemType::Valuable,
                name: "Alumni Token",
                order_priority: 0,
                max_quantity: 99,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d15dc445af50b0d46b185ca1e3b6d67e",
            Item {
                guid: "d15dc445af50b0d46b185ca1e3b6d67e",
                item_type: ItemType::Valuable,
                name: "Alumni Token",
                order_priority: 0,
                max_quantity: 99,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_BLUEPEARL_NAME
        m.insert(
            "Blue Pearl",
            Item {
                guid: "2125d95821df6824a828a69bd902bf14",
                item_type: ItemType::Key,
                name: "Blue Pearl",
                order_priority: 5025,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "2125d95821df6824a828a69bd902bf14",
            Item {
                guid: "2125d95821df6824a828a69bd902bf14",
                item_type: ItemType::Key,
                name: "Blue Pearl",
                order_priority: 5025,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_CLOCKWORKGARDENKEY_NAME
        m.insert(
            "Garden Key",
            Item {
                guid: "fc97e4a59d8cf79419322000ce72afb8",
                item_type: ItemType::Key,
                name: "Garden Key",
                order_priority: 6000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "fc97e4a59d8cf79419322000ce72afb8",
            Item {
                guid: "fc97e4a59d8cf79419322000ce72afb8",
                item_type: ItemType::Key,
                name: "Garden Key",
                order_priority: 6000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_CONCH_NAME
        m.insert(
            "Rainbow Conch",
            Item {
                guid: "5eb73f88e6c96f94797fa11b7d11b061",
                item_type: ItemType::Key,
                name: "Rainbow Conch",
                order_priority: 0,
                max_quantity: 999,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "5eb73f88e6c96f94797fa11b7d11b061",
            Item {
                guid: "5eb73f88e6c96f94797fa11b7d11b061",
                item_type: ItemType::Key,
                name: "Rainbow Conch",
                order_priority: 0,
                max_quantity: 999,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_COOKIEJAR_NAME
        m.insert(
            "Cookie Jar",
            Item {
                guid: "08ae384f758e8be4ba4e7d58975838e6",
                item_type: ItemType::Key,
                name: "Cookie Jar",
                order_priority: 100000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "08ae384f758e8be4ba4e7d58975838e6",
            Item {
                guid: "08ae384f758e8be4ba4e7d58975838e6",
                item_type: ItemType::Key,
                name: "Cookie Jar",
                order_priority: 100000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_CORALHAMMER_NAME
        m.insert(
            "Coral Hammer",
            Item {
                guid: "3de9715179fe13d44bf0ac2d38ce5db4",
                item_type: ItemType::Key,
                name: "Coral Hammer",
                order_priority: 5020,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "3de9715179fe13d44bf0ac2d38ce5db4",
            Item {
                guid: "3de9715179fe13d44bf0ac2d38ce5db4",
                item_type: ItemType::Key,
                name: "Coral Hammer",
                order_priority: 5020,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_CORALHAMMERUPGRADED_NAME
        m.insert(
            "Cobalt Hammer",
            Item {
                guid: "f0faa32c4f2b19e4d9a1c825c5a23771",
                item_type: ItemType::Key,
                name: "Cobalt Hammer",
                order_priority: 7015,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f0faa32c4f2b19e4d9a1c825c5a23771",
            Item {
                guid: "f0faa32c4f2b19e4d9a1c825c5a23771",
                item_type: ItemType::Key,
                name: "Cobalt Hammer",
                order_priority: 7015,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_DUKEKEY_NAME
        m.insert(
            "Duke Aventry's Key",
            Item {
                guid: "e9e25324a369a4d4e9331778a4deb8b5",
                item_type: ItemType::Key,
                name: "Duke Aventry's Key",
                order_priority: 11050,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e9e25324a369a4d4e9331778a4deb8b5",
            Item {
                guid: "e9e25324a369a4d4e9331778a4deb8b5",
                item_type: ItemType::Key,
                name: "Duke Aventry's Key",
                order_priority: 11050,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_DECRYPTEDDATACHIP_NAME
        m.insert(
            "Cypher",
            Item {
                guid: "b139bc7db13b9ab4d93f2d1b10816c1d",
                item_type: ItemType::Key,
                name: "Cypher",
                order_priority: 11015,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b139bc7db13b9ab4d93f2d1b10816c1d",
            Item {
                guid: "b139bc7db13b9ab4d93f2d1b10816c1d",
                item_type: ItemType::Key,
                name: "Cypher",
                order_priority: 11015,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FACTORYBLUEPRINTDISC_NAME
        m.insert(
            "Blueprint Disc",
            Item {
                guid: "20e59602e0dabaf4eb02fc6826cc780b",
                item_type: ItemType::Key,
                name: "Blueprint Disc",
                order_priority: 8000,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "20e59602e0dabaf4eb02fc6826cc780b",
            Item {
                guid: "20e59602e0dabaf4eb02fc6826cc780b",
                item_type: ItemType::Key,
                name: "Blueprint Disc",
                order_priority: 8000,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FACTORYCRANEKEY_NAME
        m.insert(
            "Crane Operator ID Card",
            Item {
                guid: "503a7b18abfa01249b986399fe588505",
                item_type: ItemType::Key,
                name: "Crane Operator ID Card",
                order_priority: 8005,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "503a7b18abfa01249b986399fe588505",
            Item {
                guid: "503a7b18abfa01249b986399fe588505",
                item_type: ItemType::Key,
                name: "Crane Operator ID Card",
                order_priority: 8005,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FACTORYPOWERBLOCK_NAME
        m.insert(
            "Power Unit",
            Item {
                guid: "503f82df95dcc16408b338ebf3974c39",
                item_type: ItemType::Key,
                name: "Power Unit",
                order_priority: 8010,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "503f82df95dcc16408b338ebf3974c39",
            Item {
                guid: "503f82df95dcc16408b338ebf3974c39",
                item_type: ItemType::Key,
                name: "Power Unit",
                order_priority: 8010,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FACTORYRAWMATERIAL_NAME
        m.insert(
            "Kyber Ore",
            Item {
                guid: "e1bd356c4c92f9b4a9ae9557740a8b7c",
                item_type: ItemType::Key,
                name: "Kyber Ore",
                order_priority: 8015,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e1bd356c4c92f9b4a9ae9557740a8b7c",
            Item {
                guid: "e1bd356c4c92f9b4a9ae9557740a8b7c",
                item_type: ItemType::Key,
                name: "Kyber Ore",
                order_priority: 8015,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FINEBRUSH_NAME
        m.insert(
            "Fine Brush",
            Item {
                guid: "55277ab6cbce74147937da9db8e530f4",
                item_type: ItemType::Key,
                name: "Fine Brush",
                order_priority: 2000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "55277ab6cbce74147937da9db8e530f4",
            Item {
                guid: "55277ab6cbce74147937da9db8e530f4",
                item_type: ItemType::Key,
                name: "Fine Brush",
                order_priority: 2000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FACTORYWIREPLATESHEET_NAME
        m.insert(
            "Wireplate Sheet",
            Item {
                guid: "01e9b3e17b5f4c647b43afaba1305bc4",
                item_type: ItemType::Key,
                name: "Wireplate Sheet",
                order_priority: 8020,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "01e9b3e17b5f4c647b43afaba1305bc4",
            Item {
                guid: "01e9b3e17b5f4c647b43afaba1305bc4",
                item_type: ItemType::Key,
                name: "Wireplate Sheet",
                order_priority: 8020,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FISHINGDUNGEON_DOORKEY_NAME
        m.insert(
            "Fishing Dungeon Key",
            Item {
                guid: "e6efe37cf14f7114db73cda5972f8da3",
                item_type: ItemType::Key,
                name: "Fishing Dungeon Key",
                order_priority: 11030,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e6efe37cf14f7114db73cda5972f8da3",
            Item {
                guid: "e6efe37cf14f7114db73cda5972f8da3",
                item_type: ItemType::Key,
                name: "Fishing Dungeon Key",
                order_priority: 11030,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FISHINGHUTBLUEPRINT_NAME
        m.insert(
            "Fishing Hut Plans",
            Item {
                guid: "33547d6f7d3ba254e876b1256e5c6e53",
                item_type: ItemType::Key,
                name: "Fishing Hut Plans",
                order_priority: 600,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "33547d6f7d3ba254e876b1256e5c6e53",
            Item {
                guid: "33547d6f7d3ba254e876b1256e5c6e53",
                item_type: ItemType::Key,
                name: "Fishing Hut Plans",
                order_priority: 600,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FLESHMANCERLAIR_FISH_NAME
        m.insert(
            "Fish?",
            Item {
                guid: "b6181ca295fc4964da34df0379e5ead0",
                item_type: ItemType::Key,
                name: "Fish?",
                order_priority: 10000,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b6181ca295fc4964da34df0379e5ead0",
            Item {
                guid: "b6181ca295fc4964da34df0379e5ead0",
                item_type: ItemType::Key,
                name: "Fish?",
                order_priority: 10000,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FLESHMANCERLAIR_VASE01_NAME
        m.insert(
            "Big Jar",
            Item {
                guid: "3002629170d8ec448a27d1bfc69bc9b2",
                item_type: ItemType::Key,
                name: "Big Jar",
                order_priority: 10005,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "3002629170d8ec448a27d1bfc69bc9b2",
            Item {
                guid: "3002629170d8ec448a27d1bfc69bc9b2",
                item_type: ItemType::Key,
                name: "Big Jar",
                order_priority: 10005,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FLESHMANCERLAIR_VASE02_NAME
        m.insert(
            "Big Jar",
            Item {
                guid: "349d0c0d0fec3d845bf2dbb16b14b160",
                item_type: ItemType::Key,
                name: "Big Jar",
                order_priority: 10010,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "349d0c0d0fec3d845bf2dbb16b14b160",
            Item {
                guid: "349d0c0d0fec3d845bf2dbb16b14b160",
                item_type: ItemType::Key,
                name: "Big Jar",
                order_priority: 10010,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FLESHMANCERLAIR_VASE03_NAME
        m.insert(
            "Big Jar",
            Item {
                guid: "ae301ae0d0834f14e8ed770c2c9cfba6",
                item_type: ItemType::Key,
                name: "Big Jar",
                order_priority: 10015,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "ae301ae0d0834f14e8ed770c2c9cfba6",
            Item {
                guid: "ae301ae0d0834f14e8ed770c2c9cfba6",
                item_type: ItemType::Key,
                name: "Big Jar",
                order_priority: 10015,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // VALUABLEITEM_FLIMSYHAMMER_NAME
        m.insert(
            "Flimsy Hammer",
            Item {
                guid: "aae87498b3763054086daf64fec4c1ed",
                item_type: ItemType::Valuable,
                name: "Flimsy Hammer",
                order_priority: 0,
                max_quantity: 4,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "aae87498b3763054086daf64fec4c1ed",
            Item {
                guid: "aae87498b3763054086daf64fec4c1ed",
                item_type: ItemType::Valuable,
                name: "Flimsy Hammer",
                order_priority: 0,
                max_quantity: 4,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FORBIDDENCAVERNKEY_NAME
        m.insert(
            "Forbidden Cavern Key",
            Item {
                guid: "cc1057ec537f0c842bdbb572265b9a17",
                item_type: ItemType::Key,
                name: "Forbidden Cavern Key",
                order_priority: 1000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "cc1057ec537f0c842bdbb572265b9a17",
            Item {
                guid: "cc1057ec537f0c842bdbb572265b9a17",
                item_type: ItemType::Key,
                name: "Forbidden Cavern Key",
                order_priority: 1000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_GOLDENFISH_NAME
        m.insert(
            "Fish Key",
            Item {
                guid: "8b255d22228b85244a4bb673b2b8586b",
                item_type: ItemType::Key,
                name: "Fish Key",
                order_priority: 11035,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "8b255d22228b85244a4bb673b2b8586b",
            Item {
                guid: "8b255d22228b85244a4bb673b2b8586b",
                item_type: ItemType::Key,
                name: "Fish Key",
                order_priority: 11035,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_HIDDENMARKETPASS_NAME
        m.insert(
            "Trader's Signet",
            Item {
                guid: "f3fddcfa74a471740830ff34bbb256d2",
                item_type: ItemType::Key,
                name: "Trader's Signet",
                order_priority: 11000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f3fddcfa74a471740830ff34bbb256d2",
            Item {
                guid: "f3fddcfa74a471740830ff34bbb256d2",
                item_type: ItemType::Key,
                name: "Trader's Signet",
                order_priority: 11000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_GRAPLOU_NAME
        m.insert(
            "Graplou",
            Item {
                guid: "c9447122a421a2640b315d36b2562ad2",
                item_type: ItemType::Key,
                name: "Graplou",
                order_priority: 3040,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c9447122a421a2640b315d36b2562ad2",
            Item {
                guid: "c9447122a421a2640b315d36b2562ad2",
                item_type: ItemType::Key,
                name: "Graplou",
                order_priority: 3040,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_GREENPEARL_NAME
        m.insert(
            "Green Pearl",
            Item {
                guid: "aa4d810d27c12864eb74a976a24a6c5a",
                item_type: ItemType::Key,
                name: "Green Pearl",
                order_priority: 5030,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "aa4d810d27c12864eb74a976a24a6c5a",
            Item {
                guid: "aa4d810d27c12864eb74a976a24a6c5a",
                item_type: ItemType::Key,
                name: "Green Pearl",
                order_priority: 5030,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_GREENFLAME_NAME
        m.insert(
            "Green Flame",
            Item {
                guid: "70b52ef97031bd74fa467314fbcb11e6",
                item_type: ItemType::Key,
                name: "Green Flame",
                order_priority: 3000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "70b52ef97031bd74fa467314fbcb11e6",
            Item {
                guid: "70b52ef97031bd74fa467314fbcb11e6",
                item_type: ItemType::Key,
                name: "Green Flame",
                order_priority: 3000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_HAUNTEDMANSION_BAD_SANDWICH_NAME
        m.insert(
            "Bad Ghost Sandwich",
            Item {
                guid: "970faa3ecbd4a6f4bb50719ba804d3ec",
                item_type: ItemType::Key,
                name: "Bad Ghost Sandwich",
                order_priority: 3015,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "970faa3ecbd4a6f4bb50719ba804d3ec",
            Item {
                guid: "970faa3ecbd4a6f4bb50719ba804d3ec",
                item_type: ItemType::Key,
                name: "Bad Ghost Sandwich",
                order_priority: 3015,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_HAUNTEDMANSION_CROWN_NAME
        m.insert(
            "Crown",
            Item {
                guid: "d476f6c7455257948854ed1cc65afee7",
                item_type: ItemType::Key,
                name: "Crown",
                order_priority: 3020,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d476f6c7455257948854ed1cc65afee7",
            Item {
                guid: "d476f6c7455257948854ed1cc65afee7",
                item_type: ItemType::Key,
                name: "Crown",
                order_priority: 3020,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_HAUNTEDMANSION_KNIGHTHELM_NAME
        m.insert(
            "Knight's Helm",
            Item {
                guid: "759ad2684304f1f41ab1c7ca59a1ef9f",
                item_type: ItemType::Key,
                name: "Knight's Helm",
                order_priority: 3025,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "759ad2684304f1f41ab1c7ca59a1ef9f",
            Item {
                guid: "759ad2684304f1f41ab1c7ca59a1ef9f",
                item_type: ItemType::Key,
                name: "Knight's Helm",
                order_priority: 3025,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_HAUNTEDMANSION_SUPREME_SANDWICH_NAME
        m.insert(
            "Master Ghost Sandwich",
            Item {
                guid: "e94e5414de65af34a810b8f89c117b6b",
                item_type: ItemType::Key,
                name: "Master Ghost Sandwich",
                order_priority: 3035,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e94e5414de65af34a810b8f89c117b6b",
            Item {
                guid: "e94e5414de65af34a810b8f89c117b6b",
                item_type: ItemType::Key,
                name: "Master Ghost Sandwich",
                order_priority: 3035,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_HAUNTEDMANSION_REGULAR_SANDWICH_NAME
        m.insert(
            "Ordinary Ghost Sandwich",
            Item {
                guid: "e3fb5a2625fe837428d704747054077c",
                item_type: ItemType::Key,
                name: "Ordinary Ghost Sandwich",
                order_priority: 3030,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e3fb5a2625fe837428d704747054077c",
            Item {
                guid: "e3fb5a2625fe837428d704747054077c",
                item_type: ItemType::Key,
                name: "Ordinary Ghost Sandwich",
                order_priority: 3030,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_HIDDENMARKETPASS_NAME
        m.insert(
            "Trader's Signet",
            Item {
                guid: "31525df92a16eb94ba068dff6876c562",
                item_type: ItemType::Key,
                name: "Trader's Signet",
                order_priority: 11005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "31525df92a16eb94ba068dff6876c562",
            Item {
                guid: "31525df92a16eb94ba068dff6876c562",
                item_type: ItemType::Key,
                name: "Trader's Signet",
                order_priority: 11005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_HYDRALIONPEARL_NAME
        m.insert(
            "Immaculate Pearl",
            Item {
                guid: "0e74dbdddec12d64f9bc85152d4331b8",
                item_type: ItemType::Key,
                name: "Immaculate Pearl",
                order_priority: 7020,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0e74dbdddec12d64f9bc85152d4331b8",
            Item {
                guid: "0e74dbdddec12d64f9bc85152d4331b8",
                item_type: ItemType::Key,
                name: "Immaculate Pearl",
                order_priority: 7020,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_INCOMPLETEDATASTRIP_NAME
        m.insert(
            "Data Strip",
            Item {
                guid: "da09de33b7886b14aa651746c28689d4",
                item_type: ItemType::Key,
                name: "Data Strip",
                order_priority: 11010,
                max_quantity: 4,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "da09de33b7886b14aa651746c28689d4",
            Item {
                guid: "da09de33b7886b14aa651746c28689d4",
                item_type: ItemType::Key,
                name: "Data Strip",
                order_priority: 11010,
                max_quantity: 4,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_LUCENT_GHOSTCOOKIE_NAME
        m.insert(
            "Turbo Cookie",
            Item {
                guid: "5dc17c895fa172b47b22fba330496a8e",
                item_type: ItemType::Key,
                name: "Turbo Cookie",
                order_priority: 3050,
                max_quantity: 6,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "5dc17c895fa172b47b22fba330496a8e",
            Item {
                guid: "5dc17c895fa172b47b22fba330496a8e",
                item_type: ItemType::Key,
                name: "Turbo Cookie",
                order_priority: 3050,
                max_quantity: 6,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_LOCKET_NAME
        m.insert(
            "Aventry Locket",
            Item {
                guid: "b901a02219536cf43af09aa7ae1d7326",
                item_type: ItemType::Key,
                name: "Aventry Locket",
                order_priority: 11055,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b901a02219536cf43af09aa7ae1d7326",
            Item {
                guid: "b901a02219536cf43af09aa7ae1d7326",
                item_type: ItemType::Key,
                name: "Aventry Locket",
                order_priority: 11055,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_LUCENT_SHED_KEY_NAME
        m.insert(
            "Shed Key",
            Item {
                guid: "db37449de93d8dd4ba50111ced8f89e1",
                item_type: ItemType::Key,
                name: "Shed Key",
                order_priority: 3055,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "db37449de93d8dd4ba50111ced8f89e1",
            Item {
                guid: "db37449de93d8dd4ba50111ced8f89e1",
                item_type: ItemType::Key,
                name: "Shed Key",
                order_priority: 3055,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MAGICSEASHELL_NAME
        m.insert(
            "Magic Seashell",
            Item {
                guid: "b391f0c6724c8d443adec6105be71661",
                item_type: ItemType::Key,
                name: "Magic Seashell",
                order_priority: 5000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b391f0c6724c8d443adec6105be71661",
            Item {
                guid: "b391f0c6724c8d443adec6105be71661",
                item_type: ItemType::Key,
                name: "Magic Seashell",
                order_priority: 5000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_GOLDENFISH_NAME
        m.insert(
            "Fish Key",
            Item {
                guid: "5c63abeb5da9f114ba2c2da954e59d82",
                item_type: ItemType::Key,
                name: "Fish Key",
                order_priority: 11040,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "5c63abeb5da9f114ba2c2da954e59d82",
            Item {
                guid: "5c63abeb5da9f114ba2c2da954e59d82",
                item_type: ItemType::Key,
                name: "Fish Key",
                order_priority: 11040,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_COMPASS_NAME
        m.insert(
            "Map",
            Item {
                guid: "aefb6b3d640e4804d85814203c8baa2c",
                item_type: ItemType::Key,
                name: "Map",
                order_priority: 4000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "aefb6b3d640e4804d85814203c8baa2c",
            Item {
                guid: "aefb6b3d640e4804d85814203c8baa2c",
                item_type: ItemType::Key,
                name: "Map",
                order_priority: 4000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MASTERKEY_NAME
        m.insert(
            "Master Key",
            Item {
                guid: "5430316fcbe43364191d2c0ba415e415",
                item_type: ItemType::Key,
                name: "Master Key",
                order_priority: 6005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "5430316fcbe43364191d2c0ba415e415",
            Item {
                guid: "5430316fcbe43364191d2c0ba415e415",
                item_type: ItemType::Key,
                name: "Master Key",
                order_priority: 6005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MINES_SECRETDOORKEY_NAME
        m.insert(
            "Mines Key",
            Item {
                guid: "f7046c4e3f1a9fa45ae10bb5f7c76013",
                item_type: ItemType::Key,
                name: "Mines Key",
                order_priority: 2005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f7046c4e3f1a9fa45ae10bb5f7c76013",
            Item {
                guid: "f7046c4e3f1a9fa45ae10bb5f7c76013",
                item_type: ItemType::Key,
                name: "Mines Key",
                order_priority: 2005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MOONCRADLEDUNGEONKEY_NAME
        m.insert(
            "Ornate Stone",
            Item {
                guid: "1f912127975af7b4f8c7316fe4d6a625",
                item_type: ItemType::Key,
                name: "Ornate Stone",
                order_priority: 1005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "1f912127975af7b4f8c7316fe4d6a625",
            Item {
                guid: "1f912127975af7b4f8c7316fe4d6a625",
                item_type: ItemType::Key,
                name: "Ornate Stone",
                order_priority: 1005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MORAINEOFFICEKEY_NAME
        m.insert(
            "Moraine's Office Key",
            Item {
                guid: "c5f18f0632cff954eb6a3bc67eb9c7fe",
                item_type: ItemType::Key,
                name: "Moraine's Office Key",
                order_priority: 5015,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c5f18f0632cff954eb6a3bc67eb9c7fe",
            Item {
                guid: "c5f18f0632cff954eb6a3bc67eb9c7fe",
                item_type: ItemType::Key,
                name: "Moraine's Office Key",
                order_priority: 5015,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MORAINE_KEY_NAME
        m.insert(
            "Evermist Shrine Key",
            Item {
                guid: "ba8e4259fdc96ac48aebd98867165d9c",
                item_type: ItemType::Key,
                name: "Evermist Shrine Key",
                order_priority: 5010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "ba8e4259fdc96ac48aebd98867165d9c",
            Item {
                guid: "ba8e4259fdc96ac48aebd98867165d9c",
                item_type: ItemType::Key,
                name: "Evermist Shrine Key",
                order_priority: 5010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_FLESHMANCERLAIR_SOULSTONE_NAME
        m.insert(
            "Soul Stone",
            Item {
                guid: "3df62cf7dbeaa4a4ebc243672b8dacb8",
                item_type: ItemType::Key,
                name: "Soul Stone",
                order_priority: 3005,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "3df62cf7dbeaa4a4ebc243672b8dacb8",
            Item {
                guid: "3df62cf7dbeaa4a4ebc243672b8dacb8",
                item_type: ItemType::Key,
                name: "Soul Stone",
                order_priority: 3005,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_PINKPEARL_NAME
        m.insert(
            "Pink Pearl",
            Item {
                guid: "efa941d37f7e8a741bffda3674865874",
                item_type: ItemType::Key,
                name: "Pink Pearl",
                order_priority: 5035,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "efa941d37f7e8a741bffda3674865874",
            Item {
                guid: "efa941d37f7e8a741bffda3674865874",
                item_type: ItemType::Key,
                name: "Pink Pearl",
                order_priority: 5035,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_PROBE_NAME
        m.insert(
            "Probe",
            Item {
                guid: "4ca8a003a14a791458c38e8739f32344",
                item_type: ItemType::Key,
                name: "Probe",
                order_priority: 8025,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "4ca8a003a14a791458c38e8739f32344",
            Item {
                guid: "4ca8a003a14a791458c38e8739f32344",
                item_type: ItemType::Key,
                name: "Probe",
                order_priority: 8025,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_PURPLEPEARL_NAME
        m.insert(
            "Purple Pearl",
            Item {
                guid: "0aed24b71deeec34689a568297b730ba",
                item_type: ItemType::Key,
                name: "Purple Pearl",
                order_priority: 5040,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0aed24b71deeec34689a568297b730ba",
            Item {
                guid: "0aed24b71deeec34689a568297b730ba",
                item_type: ItemType::Key,
                name: "Purple Pearl",
                order_priority: 5040,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_PUSHBRACELET_NAME
        m.insert(
            "Mistral Bracelet",
            Item {
                guid: "5b77e66a1d52fce4cbab840d6dd157c4",
                item_type: ItemType::Key,
                name: "Mistral Bracelet",
                order_priority: 2010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "5b77e66a1d52fce4cbab840d6dd157c4",
            Item {
                guid: "5b77e66a1d52fce4cbab840d6dd157c4",
                item_type: ItemType::Key,
                name: "Mistral Bracelet",
                order_priority: 2010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_QUIZZ_QUESTIONPACK01_NAME
        m.insert(
            "Question Pack #1",
            Item {
                guid: "92577167b3635ef45b5bb4449e0db946",
                item_type: ItemType::Key,
                name: "Question Pack #1",
                order_priority: 960,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "92577167b3635ef45b5bb4449e0db946",
            Item {
                guid: "92577167b3635ef45b5bb4449e0db946",
                item_type: ItemType::Key,
                name: "Question Pack #1",
                order_priority: 960,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_QUIZZ_QUESTIONPACK03_NAME
        m.insert(
            "Question Pack #3",
            Item {
                guid: "dc7c66a209c025f4697e8e8491e098cd",
                item_type: ItemType::Key,
                name: "Question Pack #3",
                order_priority: 950,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "dc7c66a209c025f4697e8e8491e098cd",
            Item {
                guid: "dc7c66a209c025f4697e8e8491e098cd",
                item_type: ItemType::Key,
                name: "Question Pack #3",
                order_priority: 950,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_QUIZZ_QUESTIONPACK02_NAME
        m.insert(
            "Question Pack #2",
            Item {
                guid: "24ff003b0e4c78047b638dccac197484",
                item_type: ItemType::Key,
                name: "Question Pack #2",
                order_priority: 955,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "24ff003b0e4c78047b638dccac197484",
            Item {
                guid: "24ff003b0e4c78047b638dccac197484",
                item_type: ItemType::Key,
                name: "Question Pack #2",
                order_priority: 955,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_QUIZZ_QUESTIONPACK04_NAME
        m.insert(
            "Question Pack #4",
            Item {
                guid: "33d4a1a7ec0d9b741a8a175d2c24e8c7",
                item_type: ItemType::Key,
                name: "Question Pack #4",
                order_priority: 945,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "33d4a1a7ec0d9b741a8a175d2c24e8c7",
            Item {
                guid: "33d4a1a7ec0d9b741a8a175d2c24e8c7",
                item_type: ItemType::Key,
                name: "Question Pack #4",
                order_priority: 945,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_QUIZZ_QUESTIONPACK05_NAME
        m.insert(
            "Question Pack #5",
            Item {
                guid: "8cae32b464ca8e844b378263d8d34011",
                item_type: ItemType::Key,
                name: "Question Pack #5",
                order_priority: 940,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "8cae32b464ca8e844b378263d8d34011",
            Item {
                guid: "8cae32b464ca8e844b378263d8d34011",
                item_type: ItemType::Key,
                name: "Question Pack #5",
                order_priority: 940,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_QUIZZ_QUESTIONPACK06_NAME
        m.insert(
            "Question Pack #6",
            Item {
                guid: "1e31e8cf36ae60841a47f3d9f2fce3bf",
                item_type: ItemType::Key,
                name: "Question Pack #6",
                order_priority: 935,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "1e31e8cf36ae60841a47f3d9f2fce3bf",
            Item {
                guid: "1e31e8cf36ae60841a47f3d9f2fce3bf",
                item_type: ItemType::Key,
                name: "Question Pack #6",
                order_priority: 935,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_QUIZZ_QUESTIONPACK08_NAME
        m.insert(
            "Question Pack #8",
            Item {
                guid: "b299ede2f42dd2e458fc1fa65fbe3863",
                item_type: ItemType::Key,
                name: "Question Pack #8",
                order_priority: 925,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b299ede2f42dd2e458fc1fa65fbe3863",
            Item {
                guid: "b299ede2f42dd2e458fc1fa65fbe3863",
                item_type: ItemType::Key,
                name: "Question Pack #8",
                order_priority: 925,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_QUIZZ_QUESTIONPACK07_NAME
        m.insert(
            "Question Pack #7",
            Item {
                guid: "fe30e0ddff22ff94081ceecff3a5d5fd",
                item_type: ItemType::Key,
                name: "Question Pack #7",
                order_priority: 930,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "fe30e0ddff22ff94081ceecff3a5d5fd",
            Item {
                guid: "fe30e0ddff22ff94081ceecff3a5d5fd",
                item_type: ItemType::Key,
                name: "Question Pack #7",
                order_priority: 930,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_QUIZZ_QUESTIONPACK09_NAME
        m.insert(
            "Question Pack #9",
            Item {
                guid: "08ec181ba4b1e0147be8d62ce1973ab1",
                item_type: ItemType::Key,
                name: "Question Pack #9",
                order_priority: 920,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "08ec181ba4b1e0147be8d62ce1973ab1",
            Item {
                guid: "08ec181ba4b1e0147be8d62ce1973ab1",
                item_type: ItemType::Key,
                name: "Question Pack #9",
                order_priority: 920,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_QUIZZ_QUESTIONPACK10_NAME
        m.insert(
            "Question Pack #10",
            Item {
                guid: "98606956383da154fb4c64737d96c898",
                item_type: ItemType::Key,
                name: "Question Pack #10",
                order_priority: 915,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "98606956383da154fb4c64737d96c898",
            Item {
                guid: "98606956383da154fb4c64737d96c898",
                item_type: ItemType::Key,
                name: "Question Pack #10",
                order_priority: 915,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_QUIZZ_QUESTIONPACK11_NAME
        m.insert(
            "Question Pack #11",
            Item {
                guid: "ace6400b0e4dcd344b58181d9a10994f",
                item_type: ItemType::Key,
                name: "Question Pack #11",
                order_priority: 910,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "ace6400b0e4dcd344b58181d9a10994f",
            Item {
                guid: "ace6400b0e4dcd344b58181d9a10994f",
                item_type: ItemType::Key,
                name: "Question Pack #11",
                order_priority: 910,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_SS2_KEYA_NAME
        m.insert(
            "Solstice Shrine Key A",
            Item {
                guid: "42595a0119f967845b9397365508ba64",
                item_type: ItemType::Key,
                name: "Solstice Shrine Key A",
                order_priority: 11060,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "42595a0119f967845b9397365508ba64",
            Item {
                guid: "42595a0119f967845b9397365508ba64",
                item_type: ItemType::Key,
                name: "Solstice Shrine Key A",
                order_priority: 11060,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_ROMAYAJOURNAL_NAME
        m.insert(
            "Romaya's Journal",
            Item {
                guid: "ca355ae6b0e5289469961e23ecc819d1",
                item_type: ItemType::Key,
                name: "Romaya's Journal",
                order_priority: 11025,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "ca355ae6b0e5289469961e23ecc819d1",
            Item {
                guid: "ca355ae6b0e5289469961e23ecc819d1",
                item_type: ItemType::Key,
                name: "Romaya's Journal",
                order_priority: 11025,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_SS2_KEYB_NAME
        m.insert(
            "Solstice Shrine Key B",
            Item {
                guid: "bc821a537ec005c498861cbea8cffe92",
                item_type: ItemType::Key,
                name: "Solstice Shrine Key B",
                order_priority: 11055,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "bc821a537ec005c498861cbea8cffe92",
            Item {
                guid: "bc821a537ec005c498861cbea8cffe92",
                item_type: ItemType::Key,
                name: "Solstice Shrine Key B",
                order_priority: 11055,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_SEASHELL_NAME
        m.insert(
            "Seashell",
            Item {
                guid: "2295d1bfeec0f8844b477f95c919c74f",
                item_type: ItemType::Key,
                name: "Seashell",
                order_priority: 5005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "2295d1bfeec0f8844b477f95c919c74f",
            Item {
                guid: "2295d1bfeec0f8844b477f95c919c74f",
                item_type: ItemType::Key,
                name: "Seashell",
                order_priority: 5005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MUSICSHEET1_NAME
        m.insert(
            "Music Sheet #1",
            Item {
                guid: "46742572097b4014fb8da9f198d0942b",
                item_type: ItemType::Key,
                name: "Music Sheet #1",
                order_priority: 830,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "46742572097b4014fb8da9f198d0942b",
            Item {
                guid: "46742572097b4014fb8da9f198d0942b",
                item_type: ItemType::Key,
                name: "Music Sheet #1",
                order_priority: 830,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MUSICSHEET3_NAME
        m.insert(
            "Music Sheet #3",
            Item {
                guid: "25e534eb1a920ed4897ad72ae4ff9453",
                item_type: ItemType::Key,
                name: "Music Sheet #3",
                order_priority: 820,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "25e534eb1a920ed4897ad72ae4ff9453",
            Item {
                guid: "25e534eb1a920ed4897ad72ae4ff9453",
                item_type: ItemType::Key,
                name: "Music Sheet #3",
                order_priority: 820,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MUSICSHEET2_NAME
        m.insert(
            "Music Sheet #2",
            Item {
                guid: "3b000cb7c50ea4a4f85dad1ad78a12a4",
                item_type: ItemType::Key,
                name: "Music Sheet #2",
                order_priority: 825,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "3b000cb7c50ea4a4f85dad1ad78a12a4",
            Item {
                guid: "3b000cb7c50ea4a4f85dad1ad78a12a4",
                item_type: ItemType::Key,
                name: "Music Sheet #2",
                order_priority: 825,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MUSICSHEET4_NAME
        m.insert(
            "Music Sheet #4",
            Item {
                guid: "86cd787e0f837ff458f6be93fbb995d3",
                item_type: ItemType::Key,
                name: "Music Sheet #4",
                order_priority: 815,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "86cd787e0f837ff458f6be93fbb995d3",
            Item {
                guid: "86cd787e0f837ff458f6be93fbb995d3",
                item_type: ItemType::Key,
                name: "Music Sheet #4",
                order_priority: 815,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MUSICSHEET5_NAME
        m.insert(
            "Music Sheet #5",
            Item {
                guid: "43eddaeda003ea148b7dc68a09ef80cc",
                item_type: ItemType::Key,
                name: "Music Sheet #5",
                order_priority: 810,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "43eddaeda003ea148b7dc68a09ef80cc",
            Item {
                guid: "43eddaeda003ea148b7dc68a09ef80cc",
                item_type: ItemType::Key,
                name: "Music Sheet #5",
                order_priority: 810,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MUSICSHEET6_NAME
        m.insert(
            "Music Sheet #6",
            Item {
                guid: "e7d15540dc5a0d44ba8b43a1160dc534",
                item_type: ItemType::Key,
                name: "Music Sheet #6",
                order_priority: 805,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e7d15540dc5a0d44ba8b43a1160dc534",
            Item {
                guid: "e7d15540dc5a0d44ba8b43a1160dc534",
                item_type: ItemType::Key,
                name: "Music Sheet #6",
                order_priority: 805,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_MUSICSHEET7_NAME
        m.insert(
            "Music Sheet #7",
            Item {
                guid: "a9c546d8c82f54048a911c909db5bb93",
                item_type: ItemType::Key,
                name: "Music Sheet #7",
                order_priority: 800,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a9c546d8c82f54048a911c909db5bb93",
            Item {
                guid: "a9c546d8c82f54048a911c909db5bb93",
                item_type: ItemType::Key,
                name: "Music Sheet #7",
                order_priority: 800,
                max_quantity: 1,
                buy_price: 56,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_SHOPBLUEPRINT_NAME
        m.insert(
            "Shop Plans",
            Item {
                guid: "c7697b21842b62b41968ca1d03749259",
                item_type: ItemType::Key,
                name: "Shop Plans",
                order_priority: 605,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c7697b21842b62b41968ca1d03749259",
            Item {
                guid: "c7697b21842b62b41968ca1d03749259",
                item_type: ItemType::Key,
                name: "Shop Plans",
                order_priority: 605,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_AICORE_NAME
        m.insert(
            "AI Core",
            Item {
                guid: "8c001ec3683345a4cbc7c43782dacbdd",
                item_type: ItemType::Key,
                name: "AI Core",
                order_priority: 9005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "8c001ec3683345a4cbc7c43782dacbdd",
            Item {
                guid: "8c001ec3683345a4cbc7c43782dacbdd",
                item_type: ItemType::Key,
                name: "AI Core",
                order_priority: 9005,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_SOLSTICEAMULET_NAME
        m.insert(
            "Solstice Amulet",
            Item {
                guid: "4586380c0f09cc943bdacf7acf50a3a2",
                item_type: ItemType::Key,
                name: "Solstice Amulet",
                order_priority: 5060,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "4586380c0f09cc943bdacf7acf50a3a2",
            Item {
                guid: "4586380c0f09cc943bdacf7acf50a3a2",
                item_type: ItemType::Key,
                name: "Solstice Amulet",
                order_priority: 5060,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_SONGSHROOMMARSH_NAME
        m.insert(
            "Yomara's Key",
            Item {
                guid: "c9e2bf06d9f76f442b45863d8909ced5",
                item_type: ItemType::Key,
                name: "Yomara's Key",
                order_priority: 6002,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c9e2bf06d9f76f442b45863d8909ced5",
            Item {
                guid: "c9e2bf06d9f76f442b45863d8909ced5",
                item_type: ItemType::Key,
                name: "Yomara's Key",
                order_priority: 6002,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_SPABLUEPRINT_NAME
        m.insert(
            "Spa Plans",
            Item {
                guid: "542f2e8deac123745ab1e57442a956cc",
                item_type: ItemType::Key,
                name: "Spa Plans",
                order_priority: 610,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "542f2e8deac123745ab1e57442a956cc",
            Item {
                guid: "542f2e8deac123745ab1e57442a956cc",
                item_type: ItemType::Key,
                name: "Spa Plans",
                order_priority: 610,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_SEAOFNIGHTMARE_STORMKEY_NAME
        m.insert(
            "Cursed Doubloons",
            Item {
                guid: "0616ef249bcbe5b42abfd07c541cee31",
                item_type: ItemType::Key,
                name: "Cursed Doubloons",
                order_priority: 4005,
                max_quantity: 6,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0616ef249bcbe5b42abfd07c541cee31",
            Item {
                guid: "0616ef249bcbe5b42abfd07c541cee31",
                item_type: ItemType::Key,
                name: "Cursed Doubloons",
                order_priority: 4005,
                max_quantity: 6,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_SUNKENRUINDISK_A_NAME
        m.insert(
            "Docarri Seal Fragment A",
            Item {
                guid: "e488a0e3327d6214a9ba85ec2c34a5eb",
                item_type: ItemType::Key,
                name: "Docarri Seal Fragment A",
                order_priority: 11075,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e488a0e3327d6214a9ba85ec2c34a5eb",
            Item {
                guid: "e488a0e3327d6214a9ba85ec2c34a5eb",
                item_type: ItemType::Key,
                name: "Docarri Seal Fragment A",
                order_priority: 11075,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_SUNKENRUINDISK_C_NAME
        m.insert(
            "Docarri Seal Fragment C",
            Item {
                guid: "5523dc3139186874695c75c920c337a4",
                item_type: ItemType::Key,
                name: "Docarri Seal Fragment C",
                order_priority: 11065,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "5523dc3139186874695c75c920c337a4",
            Item {
                guid: "5523dc3139186874695c75c920c337a4",
                item_type: ItemType::Key,
                name: "Docarri Seal Fragment C",
                order_priority: 11065,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_SUNKENRUINDISK_B_NAME
        m.insert(
            "Docarri Seal Fragment B",
            Item {
                guid: "892c6828010cd0442a3393c5dd45e6c6",
                item_type: ItemType::Key,
                name: "Docarri Seal Fragment B",
                order_priority: 11070,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "892c6828010cd0442a3393c5dd45e6c6",
            Item {
                guid: "892c6828010cd0442a3393c5dd45e6c6",
                item_type: ItemType::Key,
                name: "Docarri Seal Fragment B",
                order_priority: 11070,
                max_quantity: 20,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_TAVERNBLUEPRINT_NAME
        m.insert(
            "Inn Plans",
            Item {
                guid: "bcd71fdd76d64e443b7b47364271b044",
                item_type: ItemType::Key,
                name: "Inn Plans",
                order_priority: 615,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "bcd71fdd76d64e443b7b47364271b044",
            Item {
                guid: "bcd71fdd76d64e443b7b47364271b044",
                item_type: ItemType::Key,
                name: "Inn Plans",
                order_priority: 615,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_TOWER3_KEY_NAME
        m.insert(
            "Crystal Egg",
            Item {
                guid: "6cdb3cdb6dd53bf488486c7fa7f6bb34",
                item_type: ItemType::Key,
                name: "Crystal Egg",
                order_priority: 9000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "6cdb3cdb6dd53bf488486c7fa7f6bb34",
            Item {
                guid: "6cdb3cdb6dd53bf488486c7fa7f6bb34",
                item_type: ItemType::Key,
                name: "Crystal Egg",
                order_priority: 9000,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_VALVE_A_NAME
        m.insert(
            "Valve",
            Item {
                guid: "aed455e0a258631408f2694e1de9aea2",
                item_type: ItemType::Key,
                name: "Valve",
                order_priority: 5050,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "aed455e0a258631408f2694e1de9aea2",
            Item {
                guid: "aed455e0a258631408f2694e1de9aea2",
                item_type: ItemType::Key,
                name: "Valve",
                order_priority: 5050,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_VALVE_B_NAME
        m.insert(
            "Valve",
            Item {
                guid: "f39ff4369129dde46a0e1fb61dca33ba",
                item_type: ItemType::Key,
                name: "Valve",
                order_priority: 5045,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f39ff4369129dde46a0e1fb61dca33ba",
            Item {
                guid: "f39ff4369129dde46a0e1fb61dca33ba",
                item_type: ItemType::Key,
                name: "Valve",
                order_priority: 5045,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_VIALOFTIME_NAME
        m.insert(
            "Vial of Time",
            Item {
                guid: "375a19f5e47b951419795eaf409f3169",
                item_type: ItemType::Key,
                name: "Vial of Time",
                order_priority: 5065,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "375a19f5e47b951419795eaf409f3169",
            Item {
                guid: "375a19f5e47b951419795eaf409f3169",
                item_type: ItemType::Key,
                name: "Vial of Time",
                order_priority: 5065,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_BOOSTGATEKEY_NAME
        m.insert(
            "Sapphire Key",
            Item {
                guid: "e554a0e60158d6a45a6fbaa76b7a149d",
                item_type: ItemType::Key,
                name: "Sapphire Key",
                order_priority: 5023,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e554a0e60158d6a45a6fbaa76b7a149d",
            Item {
                guid: "e554a0e60158d6a45a6fbaa76b7a149d",
                item_type: ItemType::Key,
                name: "Sapphire Key",
                order_priority: 5023,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSCHARACTERARCHER_NAME
        m.insert(
            "Archer Figurine",
            Item {
                guid: "eab7dd1ce97e5f549a124d6508121943",
                item_type: ItemType::Key,
                name: "Archer Figurine",
                order_priority: 35,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "eab7dd1ce97e5f549a124d6508121943",
            Item {
                guid: "eab7dd1ce97e5f549a124d6508121943",
                item_type: ItemType::Key,
                name: "Archer Figurine",
                order_priority: 35,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSCHARACTERASSASSIN_NAME
        m.insert(
            "Assassin Figurine",
            Item {
                guid: "b29f7387520ec284aadf210a3d96eeef",
                item_type: ItemType::Key,
                name: "Assassin Figurine",
                order_priority: 50,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b29f7387520ec284aadf210a3d96eeef",
            Item {
                guid: "b29f7387520ec284aadf210a3d96eeef",
                item_type: ItemType::Key,
                name: "Assassin Figurine",
                order_priority: 50,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSCHARACTERENGINEER_NAME
        m.insert(
            "Engineer Figurine",
            Item {
                guid: "1ffdc826f25d33f418925e7a9838e3e1",
                item_type: ItemType::Key,
                name: "Engineer Figurine",
                order_priority: 40,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "1ffdc826f25d33f418925e7a9838e3e1",
            Item {
                guid: "1ffdc826f25d33f418925e7a9838e3e1",
                item_type: ItemType::Key,
                name: "Engineer Figurine",
                order_priority: 40,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSCHARACTERMAGE_NAME
        m.insert(
            "Mage Figurine",
            Item {
                guid: "d7f92ecb9e4da7b418070285c25d86e1",
                item_type: ItemType::Key,
                name: "Mage Figurine",
                order_priority: 30,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d7f92ecb9e4da7b418070285c25d86e1",
            Item {
                guid: "d7f92ecb9e4da7b418070285c25d86e1",
                item_type: ItemType::Key,
                name: "Mage Figurine",
                order_priority: 30,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSCHARACTERPRIEST_NAME
        m.insert(
            "Priest Figurine",
            Item {
                guid: "0fba7ae2d8a1d2d4aa49a2c9a6fb61e7",
                item_type: ItemType::Key,
                name: "Priest Figurine",
                order_priority: 45,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0fba7ae2d8a1d2d4aa49a2c9a6fb61e7",
            Item {
                guid: "0fba7ae2d8a1d2d4aa49a2c9a6fb61e7",
                item_type: ItemType::Key,
                name: "Priest Figurine",
                order_priority: 45,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSCHARACTERWARRIOR_NAME
        m.insert(
            "Warrior Figurine",
            Item {
                guid: "e972fad3d8d63a745bdfc1dc7dc6d2f7",
                item_type: ItemType::Key,
                name: "Warrior Figurine",
                order_priority: 25,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e972fad3d8d63a745bdfc1dc7dc6d2f7",
            Item {
                guid: "e972fad3d8d63a745bdfc1dc7dc6d2f7",
                item_type: ItemType::Key,
                name: "Warrior Figurine",
                order_priority: 25,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSRANKBRONZE_NAME
        m.insert(
            "Bronze Wheel",
            Item {
                guid: "a81551d5aa1d0fc419c2023113b5c0ab",
                item_type: ItemType::Key,
                name: "Bronze Wheel",
                order_priority: 1,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a81551d5aa1d0fc419c2023113b5c0ab",
            Item {
                guid: "a81551d5aa1d0fc419c2023113b5c0ab",
                item_type: ItemType::Key,
                name: "Bronze Wheel",
                order_priority: 1,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSRANKCOPPER_NAME
        m.insert(
            "Copper Wheel",
            Item {
                guid: "76241bd9c19934344a8274665fb67f2b",
                item_type: ItemType::Key,
                name: "Copper Wheel",
                order_priority: 1,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "76241bd9c19934344a8274665fb67f2b",
            Item {
                guid: "76241bd9c19934344a8274665fb67f2b",
                item_type: ItemType::Key,
                name: "Copper Wheel",
                order_priority: 1,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSRANKDIAMOND_NAME
        m.insert(
            "Diamond Wheel",
            Item {
                guid: "56ca0aa8b72609d45a65481a6290ee75",
                item_type: ItemType::Key,
                name: "Diamond Wheel",
                order_priority: 15,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "56ca0aa8b72609d45a65481a6290ee75",
            Item {
                guid: "56ca0aa8b72609d45a65481a6290ee75",
                item_type: ItemType::Key,
                name: "Diamond Wheel",
                order_priority: 15,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSRANKGOLD_NAME
        m.insert(
            "Gold Wheel",
            Item {
                guid: "b5e8130725d015140a492ef72e660cd3",
                item_type: ItemType::Key,
                name: "Gold Wheel",
                order_priority: 10,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b5e8130725d015140a492ef72e660cd3",
            Item {
                guid: "b5e8130725d015140a492ef72e660cd3",
                item_type: ItemType::Key,
                name: "Gold Wheel",
                order_priority: 10,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSRANKPLATINUM_NAME
        m.insert(
            "Platinum Wheel",
            Item {
                guid: "f959b0aa9e4fd9649a096875b81ebe82",
                item_type: ItemType::Key,
                name: "Platinum Wheel",
                order_priority: 20,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f959b0aa9e4fd9649a096875b81ebe82",
            Item {
                guid: "f959b0aa9e4fd9649a096875b81ebe82",
                item_type: ItemType::Key,
                name: "Platinum Wheel",
                order_priority: 20,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSRANKSILVER_NAME
        m.insert(
            "Silver Wheel",
            Item {
                guid: "e024e3aebc5d1424ea1d69be9df5d4cc",
                item_type: ItemType::Key,
                name: "Silver Wheel",
                order_priority: 5,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e024e3aebc5d1424ea1d69be9df5d4cc",
            Item {
                guid: "e024e3aebc5d1424ea1d69be9df5d4cc",
                item_type: ItemType::Key,
                name: "Silver Wheel",
                order_priority: 5,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WHEELSTABLE_NAME
        m.insert(
            "Deployable Wheels Table",
            Item {
                guid: "2b0d434789480b1439e5458f158e5043",
                item_type: ItemType::Key,
                name: "Deployable Wheels Table",
                order_priority: 6010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "2b0d434789480b1439e5458f158e5043",
            Item {
                guid: "2b0d434789480b1439e5458f158e5043",
                item_type: ItemType::Key,
                name: "Deployable Wheels Table",
                order_priority: 6010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WIZARDLAB_BLUECRYSTAL_NAME
        m.insert(
            "Blue Crystal",
            Item {
                guid: "7072b9fc59fa699458b7acf5597d5d97",
                item_type: ItemType::Key,
                name: "Blue Crystal",
                order_priority: 2025,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "7072b9fc59fa699458b7acf5597d5d97",
            Item {
                guid: "7072b9fc59fa699458b7acf5597d5d97",
                item_type: ItemType::Key,
                name: "Blue Crystal",
                order_priority: 2025,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WINDKEY_NAME
        m.insert(
            "Wind Key",
            Item {
                guid: "24632686b928dbc42b744f96d59a6cf0",
                item_type: ItemType::Key,
                name: "Wind Key",
                order_priority: 7010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "24632686b928dbc42b744f96d59a6cf0",
            Item {
                guid: "24632686b928dbc42b744f96d59a6cf0",
                item_type: ItemType::Key,
                name: "Wind Key",
                order_priority: 7010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_WIZARDLAB_GREENCRYSTAL_NAME
        m.insert(
            "Green Crystal",
            Item {
                guid: "37ed1b808bebec1458cbb90a2e2021d5",
                item_type: ItemType::Key,
                name: "Green Crystal",
                order_priority: 2020,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "37ed1b808bebec1458cbb90a2e2021d5",
            Item {
                guid: "37ed1b808bebec1458cbb90a2e2021d5",
                item_type: ItemType::Key,
                name: "Green Crystal",
                order_priority: 2020,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_YELLOWFLAME_NAME
        m.insert(
            "Yellow Flame",
            Item {
                guid: "64bf3ab125f071e4785b7a00150ac1b7",
                item_type: ItemType::Key,
                name: "Yellow Flame",
                order_priority: 3010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "64bf3ab125f071e4785b7a00150ac1b7",
            Item {
                guid: "64bf3ab125f071e4785b7a00150ac1b7",
                item_type: ItemType::Key,
                name: "Yellow Flame",
                order_priority: 3010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // KEYITEM_YELLOWPEARL_NAME
        m.insert(
            "Yellow Pearl",
            Item {
                guid: "56ddec906232705469ab9a0a52fea8f2",
                item_type: ItemType::Key,
                name: "Yellow Pearl",
                order_priority: 5044,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "56ddec906232705469ab9a0a52fea8f2",
            Item {
                guid: "56ddec906232705469ab9a0a52fea8f2",
                item_type: ItemType::Key,
                name: "Yellow Pearl",
                order_priority: 5044,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_CRYOSTEELBLADE_NAME
        m.insert(
            "Kybersteel Blade",
            Item {
                guid: "0dddb9e8063bbdd4fb724929ee784f38",
                item_type: ItemType::Weapon,
                name: "Kybersteel Blade",
                order_priority: 140,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 35,
                magical_attack: 29,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0dddb9e8063bbdd4fb724929ee784f38",
            Item {
                guid: "0dddb9e8063bbdd4fb724929ee784f38",
                item_type: ItemType::Weapon,
                name: "Kybersteel Blade",
                order_priority: 140,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 35,
                magical_attack: 29,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_LEAFCAPE_NAME
        m.insert(
            "Leaf Cape",
            Item {
                guid: "b441ba29ad2257445a55095e6cb1d537",
                item_type: ItemType::Armor,
                name: "Leaf Cape",
                order_priority: 165,
                max_quantity: 255,
                buy_price: 96,
                sell_price: 24,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 28,
                magical_defense: 33,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b441ba29ad2257445a55095e6cb1d537",
            Item {
                guid: "b441ba29ad2257445a55095e6cb1d537",
                item_type: ItemType::Armor,
                name: "Leaf Cape",
                order_priority: 165,
                max_quantity: 255,
                buy_price: 96,
                sell_price: 24,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 28,
                magical_defense: 33,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_CRYOSTEELDAGGERS_NAME
        m.insert(
            "Kybersteel Daggers",
            Item {
                guid: "e24926f361bf385478653d34ac9f6a41",
                item_type: ItemType::Weapon,
                name: "Kybersteel Daggers",
                order_priority: 145,
                max_quantity: 255,
                buy_price: 184,
                sell_price: 46,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 32,
                magical_attack: 32,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e24926f361bf385478653d34ac9f6a41",
            Item {
                guid: "e24926f361bf385478653d34ac9f6a41",
                item_type: ItemType::Weapon,
                name: "Kybersteel Daggers",
                order_priority: 145,
                max_quantity: 255,
                buy_price: 184,
                sell_price: 46,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 32,
                magical_attack: 32,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_LEECHING_THORN_NAME
        m.insert(
            "Leeching Thorn",
            Item {
                guid: "933db0df85333b341a7d58da22f3b39c",
                item_type: ItemType::Trinket,
                name: "Leeching Thorn",
                order_priority: 10,
                max_quantity: 255,
                buy_price: 44,
                sell_price: 11,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "933db0df85333b341a7d58da22f3b39c",
            Item {
                guid: "933db0df85333b341a7d58da22f3b39c",
                item_type: ItemType::Trinket,
                name: "Leeching Thorn",
                order_priority: 10,
                max_quantity: 255,
                buy_price: 44,
                sell_price: 11,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_LUANASMIGHT_NAME
        m.insert(
            "Moon Bo",
            Item {
                guid: "95ac264461a94a0458a14a39954e3afc",
                item_type: ItemType::Weapon,
                name: "Moon Bo",
                order_priority: 255,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 58,
                magical_attack: 28,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "95ac264461a94a0458a14a39954e3afc",
            Item {
                guid: "95ac264461a94a0458a14a39954e3afc",
                item_type: ItemType::Weapon,
                name: "Moon Bo",
                order_priority: 255,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 58,
                magical_attack: 28,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_LUCENTSHARD_NAME
        m.insert(
            "Lucent Crystal",
            Item {
                guid: "92e2af189aebd9949a662314b01ab847",
                item_type: ItemType::Trinket,
                name: "Lucent Crystal",
                order_priority: 10,
                max_quantity: 255,
                buy_price: 150,
                sell_price: 120,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "92e2af189aebd9949a662314b01ab847",
            Item {
                guid: "92e2af189aebd9949a662314b01ab847",
                item_type: ItemType::Trinket,
                name: "Lucent Crystal",
                order_priority: 10,
                max_quantity: 255,
                buy_price: 150,
                sell_price: 120,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_MAGEKNIGHTARMOR_NAME
        m.insert(
            "Mage-Knight Armor",
            Item {
                guid: "1b92e304b4f82144eb3e123ccb755b2c",
                item_type: ItemType::Armor,
                name: "Mage-Knight Armor",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 60,
                sell_price: 15,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 15,
                magical_defense: 28,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "1b92e304b4f82144eb3e123ccb755b2c",
            Item {
                guid: "1b92e304b4f82144eb3e123ccb755b2c",
                item_type: ItemType::Armor,
                name: "Mage-Knight Armor",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 60,
                sell_price: 15,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 15,
                magical_defense: 28,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // GROUP_TRINKET_MAGICPOCKET_NAME
        m.insert(
            "Magic Pocket",
            Item {
                guid: "8c32a9ec62778cc4f8cef577c6d80812",
                item_type: ItemType::GroupTrinket,
                name: "Magic Pocket",
                order_priority: 25,
                max_quantity: 255,
                buy_price: 328,
                sell_price: 82,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "8c32a9ec62778cc4f8cef577c6d80812",
            Item {
                guid: "8c32a9ec62778cc4f8cef577c6d80812",
                item_type: ItemType::GroupTrinket,
                name: "Magic Pocket",
                order_priority: 25,
                max_quantity: 255,
                buy_price: 328,
                sell_price: 82,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_MAPLECORK_NAME
        m.insert(
            "Maple Cork",
            Item {
                guid: "8f50c130484e7c44a8d90860eaaa2806",
                item_type: ItemType::Weapon,
                name: "Maple Cork",
                order_priority: 90,
                max_quantity: 255,
                buy_price: 136,
                sell_price: 34,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 20,
                magical_attack: 22,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "8f50c130484e7c44a8d90860eaaa2806",
            Item {
                guid: "8f50c130484e7c44a8d90860eaaa2806",
                item_type: ItemType::Weapon,
                name: "Maple Cork",
                order_priority: 90,
                max_quantity: 255,
                buy_price: 136,
                sell_price: 34,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 20,
                magical_attack: 22,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_MINERSSMOCK_NAME
        m.insert(
            "Miner's Smock",
            Item {
                guid: "f23fec51c068c00418db6127c54fcc8e",
                item_type: ItemType::Armor,
                name: "Miner's Smock",
                order_priority: 10,
                max_quantity: 255,
                buy_price: 32,
                sell_price: 8,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 18,
                magical_defense: 12,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f23fec51c068c00418db6127c54fcc8e",
            Item {
                guid: "f23fec51c068c00418db6127c54fcc8e",
                item_type: ItemType::Armor,
                name: "Miner's Smock",
                order_priority: 10,
                max_quantity: 255,
                buy_price: 32,
                sell_price: 8,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 18,
                magical_defense: 12,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_MOONCRADLEBOYLID_NAME
        m.insert(
            "Mooncradle Boy's Lid",
            Item {
                guid: "0f0053be4fa1bf84db338a626d751eb2",
                item_type: ItemType::Weapon,
                name: "Mooncradle Boy's Lid",
                order_priority: 205,
                max_quantity: 255,
                buy_price: 136,
                sell_price: 34,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 50,
                magical_attack: 35,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0f0053be4fa1bf84db338a626d751eb2",
            Item {
                guid: "0f0053be4fa1bf84db338a626d751eb2",
                item_type: ItemType::Weapon,
                name: "Mooncradle Boy's Lid",
                order_priority: 205,
                max_quantity: 255,
                buy_price: 136,
                sell_price: 34,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 50,
                magical_attack: 35,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_MOONSTONEBRACER_NAME
        m.insert(
            "Moonstone Bracer",
            Item {
                guid: "4441ac030d9882445867b6ee06fbe941",
                item_type: ItemType::Trinket,
                name: "Moonstone Bracer",
                order_priority: 60,
                max_quantity: 255,
                buy_price: 200,
                sell_price: 50,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "4441ac030d9882445867b6ee06fbe941",
            Item {
                guid: "4441ac030d9882445867b6ee06fbe941",
                item_type: ItemType::Trinket,
                name: "Moonstone Bracer",
                order_priority: 60,
                max_quantity: 255,
                buy_price: 200,
                sell_price: 50,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_NEOBSIDIANSTAFF_NAME
        m.insert(
            "Neobsidian Staff",
            Item {
                guid: "d25d22b957a9c5c478e188e1f03f7107",
                item_type: ItemType::Weapon,
                name: "Neobsidian Staff",
                order_priority: 120,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 44,
                magical_attack: 23,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d25d22b957a9c5c478e188e1f03f7107",
            Item {
                guid: "d25d22b957a9c5c478e188e1f03f7107",
                item_type: ItemType::Weapon,
                name: "Neobsidian Staff",
                order_priority: 120,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 44,
                magical_attack: 23,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // GROUP_TRINKET_NANOINJECTOR_NAME
        m.insert(
            "Nano Injector",
            Item {
                guid: "8c4cbe75847cc6249b3bd79cb5a29e8c",
                item_type: ItemType::GroupTrinket,
                name: "Nano Injector",
                order_priority: 30,
                max_quantity: 255,
                buy_price: 344,
                sell_price: 86,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "8c4cbe75847cc6249b3bd79cb5a29e8c",
            Item {
                guid: "8c4cbe75847cc6249b3bd79cb5a29e8c",
                item_type: ItemType::GroupTrinket,
                name: "Nano Injector",
                order_priority: 30,
                max_quantity: 255,
                buy_price: 344,
                sell_price: 86,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // GROUP_TRINKET_NINJASPIRIT_NAME
        m.insert(
            "Spirit of Ninja",
            Item {
                guid: "1d744837c9ee072498a3e668104325ac",
                item_type: ItemType::GroupTrinket,
                name: "Spirit of Ninja",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 300,
                sell_price: 75,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "1d744837c9ee072498a3e668104325ac",
            Item {
                guid: "1d744837c9ee072498a3e668104325ac",
                item_type: ItemType::GroupTrinket,
                name: "Spirit of Ninja",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 300,
                sell_price: 75,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_OAKENARMOR_NAME
        m.insert(
            "Oaken Armor",
            Item {
                guid: "ebb0e8958a73d394da97e174d0b5b018",
                item_type: ItemType::Armor,
                name: "Oaken Armor",
                order_priority: 160,
                max_quantity: 255,
                buy_price: 104,
                sell_price: 26,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 34,
                magical_defense: 27,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "ebb0e8958a73d394da97e174d0b5b018",
            Item {
                guid: "ebb0e8958a73d394da97e174d0b5b018",
                item_type: ItemType::Armor,
                name: "Oaken Armor",
                order_priority: 160,
                max_quantity: 255,
                buy_price: 104,
                sell_price: 26,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 34,
                magical_defense: 27,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_OBSIDIAN_LID_NAME
        m.insert(
            "Obsidian Cauldron Lid",
            Item {
                guid: "c16988af55e565545b978150228fc5e7",
                item_type: ItemType::Weapon,
                name: "Obsidian Cauldron Lid",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 56,
                sell_price: 14,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 10,
                magical_attack: 8,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c16988af55e565545b978150228fc5e7",
            Item {
                guid: "c16988af55e565545b978150228fc5e7",
                item_type: ItemType::Weapon,
                name: "Obsidian Cauldron Lid",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 56,
                sell_price: 14,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 10,
                magical_attack: 8,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_ORNATEBO_NAME
        m.insert(
            "Ornate Bo",
            Item {
                guid: "c82873a6208c95241834faa4a0c90321",
                item_type: ItemType::Weapon,
                name: "Ornate Bo",
                order_priority: 105,
                max_quantity: 255,
                buy_price: 140,
                sell_price: 35,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 25,
                magical_attack: 14,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c82873a6208c95241834faa4a0c90321",
            Item {
                guid: "c82873a6208c95241834faa4a0c90321",
                item_type: ItemType::Weapon,
                name: "Ornate Bo",
                order_priority: 105,
                max_quantity: 255,
                buy_price: 140,
                sell_price: 35,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 25,
                magical_attack: 14,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_OSSEOUSSTAFF_NAME
        m.insert(
            "Osseous Staff",
            Item {
                guid: "3e2210133b9ad0940bddde38c647e3e7",
                item_type: ItemType::Weapon,
                name: "Osseous Staff",
                order_priority: 45,
                max_quantity: 255,
                buy_price: 76,
                sell_price: 19,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 15,
                magical_attack: 8,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "3e2210133b9ad0940bddde38c647e3e7",
            Item {
                guid: "3e2210133b9ad0940bddde38c647e3e7",
                item_type: ItemType::Weapon,
                name: "Osseous Staff",
                order_priority: 45,
                max_quantity: 255,
                buy_price: 76,
                sell_price: 19,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 15,
                magical_attack: 8,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_PEARLESCENTAPRON_NAME
        m.insert(
            "Pearlescent Apron",
            Item {
                guid: "bd7a12ffa698146418723ed43e6a5eea",
                item_type: ItemType::Armor,
                name: "Pearlescent Apron",
                order_priority: 145,
                max_quantity: 255,
                buy_price: 88,
                sell_price: 22,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 25,
                magical_defense: 25,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "bd7a12ffa698146418723ed43e6a5eea",
            Item {
                guid: "bd7a12ffa698146418723ed43e6a5eea",
                item_type: ItemType::Armor,
                name: "Pearlescent Apron",
                order_priority: 145,
                max_quantity: 255,
                buy_price: 88,
                sell_price: 22,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 25,
                magical_defense: 25,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_PHANTOMDAGGERS_NAME
        m.insert(
            "Phantom Daggers",
            Item {
                guid: "3f851aff158a63042bd188b378bc3d81",
                item_type: ItemType::Weapon,
                name: "Phantom Daggers",
                order_priority: 90,
                max_quantity: 255,
                buy_price: 130,
                sell_price: 30,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 20,
                magical_attack: 20,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "3f851aff158a63042bd188b378bc3d81",
            Item {
                guid: "3f851aff158a63042bd188b378bc3d81",
                item_type: ItemType::Weapon,
                name: "Phantom Daggers",
                order_priority: 90,
                max_quantity: 255,
                buy_price: 130,
                sell_price: 30,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 20,
                magical_attack: 20,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_PIRATEGARB_NAME
        m.insert(
            "Pirate Garb",
            Item {
                guid: "0626cb583af926649b435bcf63c101a7",
                item_type: ItemType::Armor,
                name: "Pirate Garb",
                order_priority: 15,
                max_quantity: 255,
                buy_price: 36,
                sell_price: 9,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 19,
                magical_defense: 15,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0626cb583af926649b435bcf63c101a7",
            Item {
                guid: "0626cb583af926649b435bcf63c101a7",
                item_type: ItemType::Armor,
                name: "Pirate Garb",
                order_priority: 15,
                max_quantity: 255,
                buy_price: 36,
                sell_price: 9,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 19,
                magical_defense: 15,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_PHOSPHORITELID_NAME
        m.insert(
            "Phosphorite Lid",
            Item {
                guid: "8df0767e67aa4e34584a5066469765de",
                item_type: ItemType::Weapon,
                name: "Phosphorite Lid",
                order_priority: 80,
                max_quantity: 255,
                buy_price: 112,
                sell_price: 28,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 21,
                magical_attack: 15,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "8df0767e67aa4e34584a5066469765de",
            Item {
                guid: "8df0767e67aa4e34584a5066469765de",
                item_type: ItemType::Weapon,
                name: "Phosphorite Lid",
                order_priority: 80,
                max_quantity: 255,
                buy_price: 112,
                sell_price: 28,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 21,
                magical_attack: 15,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_PLASMABLADE_NAME
        m.insert(
            "Plasma Blade",
            Item {
                guid: "301554cfa6770af4d9e0791f01488999",
                item_type: ItemType::Weapon,
                name: "Plasma Blade",
                order_priority: 155,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 40,
                magical_attack: 29,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "301554cfa6770af4d9e0791f01488999",
            Item {
                guid: "301554cfa6770af4d9e0791f01488999",
                item_type: ItemType::Weapon,
                name: "Plasma Blade",
                order_priority: 155,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 40,
                magical_attack: 29,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_PLASMADAGGERS_NAME
        m.insert(
            "Plasma Daggers",
            Item {
                guid: "1ca6171b638ffdb49bda2f4cfcb2e564",
                item_type: ItemType::Weapon,
                name: "Plasma Daggers",
                order_priority: 165,
                max_quantity: 255,
                buy_price: 184,
                sell_price: 46,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 36,
                magical_attack: 32,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "1ca6171b638ffdb49bda2f4cfcb2e564",
            Item {
                guid: "1ca6171b638ffdb49bda2f4cfcb2e564",
                item_type: ItemType::Weapon,
                name: "Plasma Daggers",
                order_priority: 165,
                max_quantity: 255,
                buy_price: 184,
                sell_price: 46,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 36,
                magical_attack: 32,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_PLIANTSHELLVEST_NAME
        m.insert(
            "Pliantshell Vest",
            Item {
                guid: "7fdba9db0aa4cbd4e8fa3315f94876ab",
                item_type: ItemType::Armor,
                name: "Pliantshell Vest",
                order_priority: 190,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 37,
                magical_defense: 41,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "7fdba9db0aa4cbd4e8fa3315f94876ab",
            Item {
                guid: "7fdba9db0aa4cbd4e8fa3315f94876ab",
                item_type: ItemType::Armor,
                name: "Pliantshell Vest",
                order_priority: 190,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 37,
                magical_defense: 41,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_POWER_BELT_NAME
        m.insert(
            "Power Belt",
            Item {
                guid: "154461019876b7741bbd74980276c852",
                item_type: ItemType::Trinket,
                name: "Power Belt",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 24,
                sell_price: 6,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "154461019876b7741bbd74980276c852",
            Item {
                guid: "154461019876b7741bbd74980276c852",
                item_type: ItemType::Trinket,
                name: "Power Belt",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 24,
                sell_price: 6,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // GROUP_TRINKET_REAPERSMERCY_NAME
        m.insert(
            "Reaper's Mercy",
            Item {
                guid: "427eaeae77b247442a8ec3e014542890",
                item_type: ItemType::GroupTrinket,
                name: "Reaper's Mercy",
                order_priority: 40,
                max_quantity: 255,
                buy_price: 300,
                sell_price: 75,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "427eaeae77b247442a8ec3e014542890",
            Item {
                guid: "427eaeae77b247442a8ec3e014542890",
                item_type: ItemType::GroupTrinket,
                name: "Reaper's Mercy",
                order_priority: 40,
                max_quantity: 255,
                buy_price: 300,
                sell_price: 75,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_BASICSALAD_NAME
        m.insert(
            "Recipe: Basic Salad",
            Item {
                guid: "3ad1eaff8549fe94c897ac387f8ae4c9",
                item_type: ItemType::Recipe,
                name: "Recipe: Basic Salad",
                order_priority: 20,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "3ad1eaff8549fe94c897ac387f8ae4c9",
            Item {
                guid: "3ad1eaff8549fe94c897ac387f8ae4c9",
                item_type: ItemType::Recipe,
                name: "Recipe: Basic Salad",
                order_priority: 20,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_BERRYJAM_NAME
        m.insert(
            "Recipe: Berry Jam",
            Item {
                guid: "bb98188ba554ed240a3161762390017f",
                item_type: ItemType::Recipe,
                name: "Recipe: Berry Jam",
                order_priority: 9,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "bb98188ba554ed240a3161762390017f",
            Item {
                guid: "bb98188ba554ed240a3161762390017f",
                item_type: ItemType::Recipe,
                name: "Recipe: Berry Jam",
                order_priority: 9,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_BOUILLABAISSE_NAME
        m.insert(
            "Recipe: Bouillabaisse",
            Item {
                guid: "04901995730d6e8459b705872897f088",
                item_type: ItemType::Recipe,
                name: "Recipe: Bouillabaisse",
                order_priority: 45,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "04901995730d6e8459b705872897f088",
            Item {
                guid: "04901995730d6e8459b705872897f088",
                item_type: ItemType::Recipe,
                name: "Recipe: Bouillabaisse",
                order_priority: 45,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_BRAISE_NAME
        m.insert(
            "Recipe: Braisé",
            Item {
                guid: "3088f0f4cd968e34fa3781eb9933ddf8",
                item_type: ItemType::Recipe,
                name: "Recipe: Braisé",
                order_priority: 79,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "3088f0f4cd968e34fa3781eb9933ddf8",
            Item {
                guid: "3088f0f4cd968e34fa3781eb9933ddf8",
                item_type: ItemType::Recipe,
                name: "Recipe: Braisé",
                order_priority: 79,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_CHAMPIONOMELETTE_NAME
        m.insert(
            "Recipe: Champion's Omelette",
            Item {
                guid: "3e0d11751c5fcf847ba5adbcbe4e751e",
                item_type: ItemType::Recipe,
                name: "Recipe: Champion's Omelette",
                order_priority: 70,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "3e0d11751c5fcf847ba5adbcbe4e751e",
            Item {
                guid: "3e0d11751c5fcf847ba5adbcbe4e751e",
                item_type: ItemType::Recipe,
                name: "Recipe: Champion's Omelette",
                order_priority: 70,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_CHAUDREE_NAME
        m.insert(
            "Recipe: Chaudrée",
            Item {
                guid: "3dafda053c9fcb0439e0d5719cc11c8f",
                item_type: ItemType::Recipe,
                name: "Recipe: Chaudrée",
                order_priority: 24,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "3dafda053c9fcb0439e0d5719cc11c8f",
            Item {
                guid: "3dafda053c9fcb0439e0d5719cc11c8f",
                item_type: ItemType::Recipe,
                name: "Recipe: Chaudrée",
                order_priority: 24,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_APPLEPIE_NAME
        m.insert(
            "Recipe: Croustade",
            Item {
                guid: "44abbdb24e6269e4781d1d4e4ec3bef3",
                item_type: ItemType::Recipe,
                name: "Recipe: Croustade",
                order_priority: 47,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "44abbdb24e6269e4781d1d4e4ec3bef3",
            Item {
                guid: "44abbdb24e6269e4781d1d4e4ec3bef3",
                item_type: ItemType::Recipe,
                name: "Recipe: Croustade",
                order_priority: 47,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_FISHPIE_NAME
        m.insert(
            "Recipe: Mooncradle Fish Pie",
            Item {
                guid: "f5241ef7a0fa1a04ba780e73ac23ea8d",
                item_type: ItemType::Recipe,
                name: "Recipe: Mooncradle Fish Pie",
                order_priority: 38,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f5241ef7a0fa1a04ba780e73ac23ea8d",
            Item {
                guid: "f5241ef7a0fa1a04ba780e73ac23ea8d",
                item_type: ItemType::Recipe,
                name: "Recipe: Mooncradle Fish Pie",
                order_priority: 38,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_GOURMETBURGER_NAME
        m.insert(
            "Recipe: Gourmet Burger",
            Item {
                guid: "acfbbff667ad7ad47a95d82351a64648",
                item_type: ItemType::Recipe,
                name: "Recipe: Gourmet Burger",
                order_priority: 55,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "acfbbff667ad7ad47a95d82351a64648",
            Item {
                guid: "acfbbff667ad7ad47a95d82351a64648",
                item_type: ItemType::Recipe,
                name: "Recipe: Gourmet Burger",
                order_priority: 55,
                max_quantity: 1,
                buy_price: 160,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_HEARTYSTEW_NAME
        m.insert(
            "Recipe: Hearty Stew",
            Item {
                guid: "11e59219302f6b1479a5492650982272",
                item_type: ItemType::Recipe,
                name: "Recipe: Hearty Stew",
                order_priority: 40,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "11e59219302f6b1479a5492650982272",
            Item {
                guid: "11e59219302f6b1479a5492650982272",
                item_type: ItemType::Recipe,
                name: "Recipe: Hearty Stew",
                order_priority: 40,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_HERBEDFILET_NAME
        m.insert(
            "Recipe: Herbed Filet",
            Item {
                guid: "19e478ef186577e489f95767465a1a84",
                item_type: ItemType::Recipe,
                name: "Recipe: Herbed Filet",
                order_priority: 25,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "19e478ef186577e489f95767465a1a84",
            Item {
                guid: "19e478ef186577e489f95767465a1a84",
                item_type: ItemType::Recipe,
                name: "Recipe: Herbed Filet",
                order_priority: 25,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_LASAGNA_NAME
        m.insert(
            "Recipe: Lasagna",
            Item {
                guid: "c576cca46c0189d458f53feca4fb131a",
                item_type: ItemType::Recipe,
                name: "Recipe: Lasagna",
                order_priority: 68,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c576cca46c0189d458f53feca4fb131a",
            Item {
                guid: "c576cca46c0189d458f53feca4fb131a",
                item_type: ItemType::Recipe,
                name: "Recipe: Lasagna",
                order_priority: 68,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_LEGENDARYFEAST_NAME
        m.insert(
            "Recipe: Legendary Feast",
            Item {
                guid: "06ed69ffa96524b47a68fd2fc63e8fd7",
                item_type: ItemType::Recipe,
                name: "Recipe: Legendary Feast",
                order_priority: 80,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "06ed69ffa96524b47a68fd2fc63e8fd7",
            Item {
                guid: "06ed69ffa96524b47a68fd2fc63e8fd7",
                item_type: ItemType::Recipe,
                name: "Recipe: Legendary Feast",
                order_priority: 80,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_MUSHROOMSCRAMBLE_NAME
        m.insert(
            "Recipe: Mushroom Scramble",
            Item {
                guid: "8b102534541d1f24e9f958a5041b139c",
                item_type: ItemType::Recipe,
                name: "Recipe: Mushroom Scramble",
                order_priority: 21,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "8b102534541d1f24e9f958a5041b139c",
            Item {
                guid: "8b102534541d1f24e9f958a5041b139c",
                item_type: ItemType::Recipe,
                name: "Recipe: Mushroom Scramble",
                order_priority: 21,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_PAINDORE_NAME
        m.insert(
            "Recipe: Pain Doré",
            Item {
                guid: "88fd9a7fcfbd0854dbc95a58128b09c2",
                item_type: ItemType::Recipe,
                name: "Recipe: Pain Doré",
                order_priority: 49,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "88fd9a7fcfbd0854dbc95a58128b09c2",
            Item {
                guid: "88fd9a7fcfbd0854dbc95a58128b09c2",
                item_type: ItemType::Recipe,
                name: "Recipe: Pain Doré",
                order_priority: 49,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_MUSHROOMSOUP_NAME
        m.insert(
            "Recipe: Mushroom Soup",
            Item {
                guid: "fe44b430ac413ec4f970e9e908841c55",
                item_type: ItemType::Recipe,
                name: "Recipe: Mushroom Soup",
                order_priority: 35,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "fe44b430ac413ec4f970e9e908841c55",
            Item {
                guid: "fe44b430ac413ec4f970e9e908841c55",
                item_type: ItemType::Recipe,
                name: "Recipe: Mushroom Soup",
                order_priority: 35,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_PAPILLOTTE_NAME
        m.insert(
            "Recipe: Papillote",
            Item {
                guid: "32ea2a86f22b90f4989ee64eb06c58a8",
                item_type: ItemType::Recipe,
                name: "Recipe: Papillote",
                order_priority: 31,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "32ea2a86f22b90f4989ee64eb06c58a8",
            Item {
                guid: "32ea2a86f22b90f4989ee64eb06c58a8",
                item_type: ItemType::Recipe,
                name: "Recipe: Papillote",
                order_priority: 31,
                max_quantity: 1,
                buy_price: 110,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_PARFAIT_NAME
        m.insert(
            "Recipe: Parfait",
            Item {
                guid: "168fbde19e968eb418b8053c5814637b",
                item_type: ItemType::Recipe,
                name: "Recipe: Parfait",
                order_priority: 48,
                max_quantity: 1,
                buy_price: 225,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "168fbde19e968eb418b8053c5814637b",
            Item {
                guid: "168fbde19e968eb418b8053c5814637b",
                item_type: ItemType::Recipe,
                name: "Recipe: Parfait",
                order_priority: 48,
                max_quantity: 1,
                buy_price: 225,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_PEACHSTRUDEL_NAME
        m.insert(
            "Recipe: Peach Strudel",
            Item {
                guid: "c203b6eef768b63449d485b7d5779723",
                item_type: ItemType::Recipe,
                name: "Recipe: Peach Strudel",
                order_priority: 44,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c203b6eef768b63449d485b7d5779723",
            Item {
                guid: "c203b6eef768b63449d485b7d5779723",
                item_type: ItemType::Recipe,
                name: "Recipe: Peach Strudel",
                order_priority: 44,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_POUTINE_NAME
        m.insert(
            "Recipe: Poutine",
            Item {
                guid: "e5cf696cec8c0d5419ce05ed66a8a7c4",
                item_type: ItemType::Recipe,
                name: "Recipe: Poutine",
                order_priority: 78,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e5cf696cec8c0d5419ce05ed66a8a7c4",
            Item {
                guid: "e5cf696cec8c0d5419ce05ed66a8a7c4",
                item_type: ItemType::Recipe,
                name: "Recipe: Poutine",
                order_priority: 78,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_PUDDINGCHOMEUR_NAME
        m.insert(
            "Recipe: Pudding Chômeur",
            Item {
                guid: "d3931ad465684074e8fd82c32f7498b6",
                item_type: ItemType::Recipe,
                name: "Recipe: Pudding Chômeur",
                order_priority: 69,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d3931ad465684074e8fd82c32f7498b6",
            Item {
                guid: "d3931ad465684074e8fd82c32f7498b6",
                item_type: ItemType::Recipe,
                name: "Recipe: Pudding Chômeur",
                order_priority: 69,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_ROASTSANDWICH_NAME
        m.insert(
            "Recipe: Roast Sandwich",
            Item {
                guid: "212f14bab84c08745bbc35b36daa7c77",
                item_type: ItemType::Recipe,
                name: "Recipe: Roast Sandwich",
                order_priority: 30,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "212f14bab84c08745bbc35b36daa7c77",
            Item {
                guid: "212f14bab84c08745bbc35b36daa7c77",
                item_type: ItemType::Recipe,
                name: "Recipe: Roast Sandwich",
                order_priority: 30,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_SASHIMI_NAME
        m.insert(
            "Recipe: Sashimi",
            Item {
                guid: "30bdce2e73ebcb140929cad623a54ea2",
                item_type: ItemType::Recipe,
                name: "Recipe: Sashimi",
                order_priority: 50,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "30bdce2e73ebcb140929cad623a54ea2",
            Item {
                guid: "30bdce2e73ebcb140929cad623a54ea2",
                item_type: ItemType::Recipe,
                name: "Recipe: Sashimi",
                order_priority: 50,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_TOMATOCLUB_NAME
        m.insert(
            "Recipe: Tomato Club",
            Item {
                guid: "64f77efd09e519d4999ffb01b9ff328f",
                item_type: ItemType::Recipe,
                name: "Recipe: Tomato Club",
                order_priority: 10,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "64f77efd09e519d4999ffb01b9ff328f",
            Item {
                guid: "64f77efd09e519d4999ffb01b9ff328f",
                item_type: ItemType::Recipe,
                name: "Recipe: Tomato Club",
                order_priority: 10,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_ADAMANT_SHARD_NAME
        m.insert(
            "Adamant Shard",
            Item {
                guid: "4133b72ae404b374987e2b434c7f5c10",
                item_type: ItemType::Relic,
                name: "Adamant Shard",
                order_priority: 950,
                max_quantity: 1,
                buy_price: 15,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "4133b72ae404b374987e2b434c7f5c10",
            Item {
                guid: "4133b72ae404b374987e2b434c7f5c10",
                item_type: ItemType::Relic,
                name: "Adamant Shard",
                order_priority: 950,
                max_quantity: 1,
                buy_price: 15,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_YAKITORISHRIMP_NAME
        m.insert(
            "Recipe: Yakitori Shrimp",
            Item {
                guid: "254124e5f00a4734a9802bd36d84b622",
                item_type: ItemType::Recipe,
                name: "Recipe: Yakitori Shrimp",
                order_priority: 60,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "254124e5f00a4734a9802bd36d84b622",
            Item {
                guid: "254124e5f00a4734a9802bd36d84b622",
                item_type: ItemType::Recipe,
                name: "Recipe: Yakitori Shrimp",
                order_priority: 60,
                max_quantity: 1,
                buy_price: 100,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RECIPEUNLOCK_SURFANDTURFTATAKI_NAME
        m.insert(
            "Recipe: Surf and Turf Tataki",
            Item {
                guid: "e44fdfcaf1b80d64b9b9e35ab29bddd7",
                item_type: ItemType::Recipe,
                name: "Recipe: Surf and Turf Tataki",
                order_priority: 65,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e44fdfcaf1b80d64b9b9e35ab29bddd7",
            Item {
                guid: "e44fdfcaf1b80d64b9b9e35ab29bddd7",
                item_type: ItemType::Recipe,
                name: "Recipe: Surf and Turf Tataki",
                order_priority: 65,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_AMULET_OF_STORYTELLING_NAME
        m.insert(
            "Amulet of Storytelling",
            Item {
                guid: "a91e6b011845d77418d3bae4766a8a9d",
                item_type: ItemType::Relic,
                name: "Amulet of Storytelling",
                order_priority: 1000,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a91e6b011845d77418d3bae4766a8a9d",
            Item {
                guid: "a91e6b011845d77418d3bae4766a8a9d",
                item_type: ItemType::Relic,
                name: "Amulet of Storytelling",
                order_priority: 1000,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_ARTFUL_GAMBIT_NAME
        m.insert(
            "Artful Gambit",
            Item {
                guid: "884bf88d04075264c98ab588ae5221cf",
                item_type: ItemType::Relic,
                name: "Artful Gambit",
                order_priority: 30,
                max_quantity: 1,
                buy_price: 350,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "884bf88d04075264c98ab588ae5221cf",
            Item {
                guid: "884bf88d04075264c98ab588ae5221cf",
                item_type: ItemType::Relic,
                name: "Artful Gambit",
                order_priority: 30,
                max_quantity: 1,
                buy_price: 350,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_DOUBLE_EDGE_NAME
        m.insert(
            "Double Edge",
            Item {
                guid: "a153f84abd9ac3b4ab7746c1d43095f2",
                item_type: ItemType::Relic,
                name: "Double Edge",
                order_priority: 25,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a153f84abd9ac3b4ab7746c1d43095f2",
            Item {
                guid: "a153f84abd9ac3b4ab7746c1d43095f2",
                item_type: ItemType::Relic,
                name: "Double Edge",
                order_priority: 25,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_SPARKMESHARMOR_NAME
        m.insert(
            "Sparkmesh Armor",
            Item {
                guid: "ccc69fb007cbda342844f25126c90377",
                item_type: ItemType::Armor,
                name: "Sparkmesh Armor",
                order_priority: 200,
                max_quantity: 255,
                buy_price: 152,
                sell_price: 38,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 46,
                magical_defense: 41,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "ccc69fb007cbda342844f25126c90377",
            Item {
                guid: "ccc69fb007cbda342844f25126c90377",
                item_type: ItemType::Armor,
                name: "Sparkmesh Armor",
                order_priority: 200,
                max_quantity: 255,
                buy_price: 152,
                sell_price: 38,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 46,
                magical_defense: 41,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_SPECTRALCAPE_NAME
        m.insert(
            "Spectral Cape",
            Item {
                guid: "d2fd56a9405f9c04fb303795c7840f2f",
                item_type: ItemType::Armor,
                name: "Spectral Cape",
                order_priority: 130,
                max_quantity: 255,
                buy_price: 44,
                sell_price: 11,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 19,
                magical_defense: 23,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d2fd56a9405f9c04fb303795c7840f2f",
            Item {
                guid: "d2fd56a9405f9c04fb303795c7840f2f",
                item_type: ItemType::Armor,
                name: "Spectral Cape",
                order_priority: 130,
                max_quantity: 255,
                buy_price: 44,
                sell_price: 11,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 19,
                magical_defense: 23,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_STARSHARDS_NAME
        m.insert(
            "Star Shards",
            Item {
                guid: "a27614e2379ef9e4dac1950a51b3f71d",
                item_type: ItemType::Weapon,
                name: "Star Shards",
                order_priority: 210,
                max_quantity: 255,
                buy_price: 184,
                sell_price: 46,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 49,
                magical_attack: 38,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a27614e2379ef9e4dac1950a51b3f71d",
            Item {
                guid: "a27614e2379ef9e4dac1950a51b3f71d",
                item_type: ItemType::Weapon,
                name: "Star Shards",
                order_priority: 210,
                max_quantity: 255,
                buy_price: 184,
                sell_price: 46,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 49,
                magical_attack: 38,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_SQUIRE_SWORD_NAME
        m.insert(
            "Squire Sword",
            Item {
                guid: "4a1710ba97ae350428f18704e36fe234",
                item_type: ItemType::Weapon,
                name: "Squire Sword",
                order_priority: 10,
                max_quantity: 255,
                buy_price: 32,
                sell_price: 8,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 5,
                magical_attack: 5,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "4a1710ba97ae350428f18704e36fe234",
            Item {
                guid: "4a1710ba97ae350428f18704e36fe234",
                item_type: ItemType::Weapon,
                name: "Squire Sword",
                order_priority: 10,
                max_quantity: 255,
                buy_price: 32,
                sell_price: 8,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 5,
                magical_attack: 5,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_SOLSTICESASH_NAME
        m.insert(
            "Solstice Sash",
            Item {
                guid: "076aaeb73b8283748accfb24f68a5c64",
                item_type: ItemType::Trinket,
                name: "Solstice Sash",
                order_priority: 35,
                max_quantity: 255,
                buy_price: 220,
                sell_price: 55,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "076aaeb73b8283748accfb24f68a5c64",
            Item {
                guid: "076aaeb73b8283748accfb24f68a5c64",
                item_type: ItemType::Trinket,
                name: "Solstice Sash",
                order_priority: 35,
                max_quantity: 255,
                buy_price: 220,
                sell_price: 55,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // GROUP_TRINKET_SOLSTICE_MAGE_RING_NAME
        m.insert(
            "Solstice Mage Ring",
            Item {
                guid: "c7eb103c76a21974c844159ab0f5a0d2",
                item_type: ItemType::GroupTrinket,
                name: "Solstice Mage Ring",
                order_priority: 5,
                max_quantity: 1,
                buy_price: 999,
                sell_price: 125,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c7eb103c76a21974c844159ab0f5a0d2",
            Item {
                guid: "c7eb103c76a21974c844159ab0f5a0d2",
                item_type: ItemType::GroupTrinket,
                name: "Solstice Mage Ring",
                order_priority: 5,
                max_quantity: 1,
                buy_price: 999,
                sell_price: 125,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_SPARKMESHCAPE_NAME
        m.insert(
            "Sparkmesh Cape",
            Item {
                guid: "fe4a8a4cada2d4a4999403d31f73a50a",
                item_type: ItemType::Armor,
                name: "Sparkmesh Cape",
                order_priority: 205,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 43,
                magical_defense: 44,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "fe4a8a4cada2d4a4999403d31f73a50a",
            Item {
                guid: "fe4a8a4cada2d4a4999403d31f73a50a",
                item_type: ItemType::Armor,
                name: "Sparkmesh Cape",
                order_priority: 205,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 43,
                magical_defense: 44,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // STORYARTIFACT_CLOCKWORKANDKIDENGINEERS_NAME
        m.insert(
            "Cog Prototype",
            Item {
                guid: "9fffcbe75bd0a61439e3546edd2d1049",
                item_type: ItemType::StoryArtifact,
                name: "Cog Prototype",
                order_priority: 705,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "9fffcbe75bd0a61439e3546edd2d1049",
            Item {
                guid: "9fffcbe75bd0a61439e3546edd2d1049",
                item_type: ItemType::StoryArtifact,
                name: "Cog Prototype",
                order_priority: 705,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // STORYARTIFACT_ESTRISTAE_NAME
        m.insert(
            "Broken Handle",
            Item {
                guid: "027715b8b6eb986459da52f290a22c45",
                item_type: ItemType::StoryArtifact,
                name: "Broken Handle",
                order_priority: 710,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "027715b8b6eb986459da52f290a22c45",
            Item {
                guid: "027715b8b6eb986459da52f290a22c45",
                item_type: ItemType::StoryArtifact,
                name: "Broken Handle",
                order_priority: 710,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // STORYARTIFACT_MOONCRADLEELDERMIST_NAME
        m.insert(
            "Celestial Willow Leaf",
            Item {
                guid: "a079a093d27333e4e99cd0b4abe3b6a1",
                item_type: ItemType::StoryArtifact,
                name: "Celestial Willow Leaf",
                order_priority: 720,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a079a093d27333e4e99cd0b4abe3b6a1",
            Item {
                guid: "a079a093d27333e4e99cd0b4abe3b6a1",
                item_type: ItemType::StoryArtifact,
                name: "Celestial Willow Leaf",
                order_priority: 720,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // STORYARTIFACT_LUCENTSHROUDNIGHT_NAME
        m.insert(
            "Aventry Crest",
            Item {
                guid: "2cbaec690b20378448407f18f53c7607",
                item_type: ItemType::StoryArtifact,
                name: "Aventry Crest",
                order_priority: 715,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "2cbaec690b20378448407f18f53c7607",
            Item {
                guid: "2cbaec690b20378448407f18f53c7607",
                item_type: ItemType::StoryArtifact,
                name: "Aventry Crest",
                order_priority: 715,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // STORYARTIFACT_GUNGODDESS_NAME
        m.insert(
            "Joce's Belt Buckle",
            Item {
                guid: "a7eb75ef484df8748a4c507fba097b85",
                item_type: ItemType::StoryArtifact,
                name: "Joce's Belt Buckle",
                order_priority: 700,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a7eb75ef484df8748a4c507fba097b85",
            Item {
                guid: "a7eb75ef484df8748a4c507fba097b85",
                item_type: ItemType::StoryArtifact,
                name: "Joce's Belt Buckle",
                order_priority: 700,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // STORYARTIFACT_NOMADANDSEA_NAME
        m.insert(
            "Old Coral Vase",
            Item {
                guid: "6086a4552abd9ab4dbd76f1cfb4b6812",
                item_type: ItemType::StoryArtifact,
                name: "Old Coral Vase",
                order_priority: 725,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "6086a4552abd9ab4dbd76f1cfb4b6812",
            Item {
                guid: "6086a4552abd9ab4dbd76f1cfb4b6812",
                item_type: ItemType::StoryArtifact,
                name: "Old Coral Vase",
                order_priority: 725,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // STORYARTIFACT_SLEEPERANDMOLEKINS_NAME
        m.insert(
            "Ancient Molekin Cloak",
            Item {
                guid: "c7e3009a90c46164e86fd8fb5d08cbce",
                item_type: ItemType::StoryArtifact,
                name: "Ancient Molekin Cloak",
                order_priority: 735,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c7e3009a90c46164e86fd8fb5d08cbce",
            Item {
                guid: "c7e3009a90c46164e86fd8fb5d08cbce",
                item_type: ItemType::StoryArtifact,
                name: "Ancient Molekin Cloak",
                order_priority: 735,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // STORYARTIFACT_THREESISTERS_NAME
        m.insert(
            "Romaya's Hairpin",
            Item {
                guid: "f65753c0374254346afc37b1f75cd0ee",
                item_type: ItemType::StoryArtifact,
                name: "Romaya's Hairpin",
                order_priority: 740,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f65753c0374254346afc37b1f75cd0ee",
            Item {
                guid: "f65753c0374254346afc37b1f75cd0ee",
                item_type: ItemType::StoryArtifact,
                name: "Romaya's Hairpin",
                order_priority: 740,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // STORYARTIFACT_VAMPIREROSE_NAME
        m.insert(
            "Stone Tablet Piece",
            Item {
                guid: "5c4863eb4f266574e937fb34e3031ed4",
                item_type: ItemType::StoryArtifact,
                name: "Stone Tablet Piece",
                order_priority: 745,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "5c4863eb4f266574e937fb34e3031ed4",
            Item {
                guid: "5c4863eb4f266574e937fb34e3031ed4",
                item_type: ItemType::StoryArtifact,
                name: "Stone Tablet Piece",
                order_priority: 745,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_STURDYCOG_NAME
        m.insert(
            "Sturdy Cog",
            Item {
                guid: "68d99c8b95dbf7c41a00d562b41b289a",
                item_type: ItemType::Weapon,
                name: "Sturdy Cog",
                order_priority: 110,
                max_quantity: 255,
                buy_price: 136,
                sell_price: 34,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 27,
                magical_attack: 19,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "68d99c8b95dbf7c41a00d562b41b289a",
            Item {
                guid: "68d99c8b95dbf7c41a00d562b41b289a",
                item_type: ItemType::Weapon,
                name: "Sturdy Cog",
                order_priority: 110,
                max_quantity: 255,
                buy_price: 136,
                sell_price: 34,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 27,
                magical_attack: 19,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_TATTEREDCAPE_NAME
        m.insert(
            "Tattered Cape",
            Item {
                guid: "ecf26467374cebc4b812538e7fbe44a4",
                item_type: ItemType::Armor,
                name: "Tattered Cape",
                order_priority: 155,
                max_quantity: 255,
                buy_price: 92,
                sell_price: 23,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 26,
                magical_defense: 33,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "ecf26467374cebc4b812538e7fbe44a4",
            Item {
                guid: "ecf26467374cebc4b812538e7fbe44a4",
                item_type: ItemType::Armor,
                name: "Tattered Cape",
                order_priority: 155,
                max_quantity: 255,
                buy_price: 92,
                sell_price: 23,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 26,
                magical_defense: 33,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_AMBER_STAFF_NAME
        m.insert(
            "Teal Amber Staff",
            Item {
                guid: "e27094f36cb7bb140b614b89fde60550",
                item_type: ItemType::Weapon,
                name: "Teal Amber Staff",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 48,
                sell_price: 12,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 11,
                magical_attack: 6,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e27094f36cb7bb140b614b89fde60550",
            Item {
                guid: "e27094f36cb7bb140b614b89fde60550",
                item_type: ItemType::Weapon,
                name: "Teal Amber Staff",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 48,
                sell_price: 12,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 11,
                magical_attack: 6,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // INGREDIENT_TESTINGREDIENT_NAME
        m.insert(
            "Test Ingredient",
            Item {
                guid: "8c9604711530463408f53774a8c44177",
                item_type: ItemType::Ingredient,
                name: "Test Ingredient",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 5,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "8c9604711530463408f53774a8c44177",
            Item {
                guid: "8c9604711530463408f53774a8c44177",
                item_type: ItemType::Ingredient,
                name: "Test Ingredient",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 5,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_THALASSICCLOAK_NAME
        m.insert(
            "Thalassic Cloak",
            Item {
                guid: "0e08751170c68ca4faae2b08ea913275",
                item_type: ItemType::Armor,
                name: "Thalassic Cloak",
                order_priority: 150,
                max_quantity: 255,
                buy_price: 96,
                sell_price: 24,
                equippable_by: Some(&[PlayerPartyCharacter::Serai, PlayerPartyCharacter::Reshan]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 25,
                magical_defense: 33,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0e08751170c68ca4faae2b08ea913275",
            Item {
                guid: "0e08751170c68ca4faae2b08ea913275",
                item_type: ItemType::Armor,
                name: "Thalassic Cloak",
                order_priority: 150,
                max_quantity: 255,
                buy_price: 96,
                sell_price: 24,
                equippable_by: Some(&[PlayerPartyCharacter::Serai, PlayerPartyCharacter::Reshan]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 25,
                magical_defense: 33,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_MOONGIRL_NAME
        m.insert(
            "Training Staff",
            Item {
                guid: "40b7062ac812c5d47bb1ff0df4987e8e",
                item_type: ItemType::Weapon,
                name: "Training Staff",
                order_priority: 1,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 5,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 3,
                magical_attack: 1,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "40b7062ac812c5d47bb1ff0df4987e8e",
            Item {
                guid: "40b7062ac812c5d47bb1ff0df4987e8e",
                item_type: ItemType::Weapon,
                name: "Training Staff",
                order_priority: 1,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 5,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 3,
                magical_attack: 1,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_SUNBOY_NAME
        m.insert(
            "Training Sword",
            Item {
                guid: "e3098c0169021924a97713b57a009928",
                item_type: ItemType::Weapon,
                name: "Training Sword",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 5,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 1,
                magical_attack: 3,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e3098c0169021924a97713b57a009928",
            Item {
                guid: "e3098c0169021924a97713b57a009928",
                item_type: ItemType::Weapon,
                name: "Training Sword",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 5,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 1,
                magical_attack: 3,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_TRUESILVERDAGGERS_NAME
        m.insert(
            "Truesilver Daggers",
            Item {
                guid: "0d86cee64cf8be44985fcee7a34c49f2",
                item_type: ItemType::Weapon,
                name: "Truesilver Daggers",
                order_priority: 125,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 27,
                magical_attack: 29,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0d86cee64cf8be44985fcee7a34c49f2",
            Item {
                guid: "0d86cee64cf8be44985fcee7a34c49f2",
                item_type: ItemType::Weapon,
                name: "Truesilver Daggers",
                order_priority: 125,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 27,
                magical_attack: 29,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // STORYARTIFACT_SKYGIANTANDKHUKKAR_NAME
        m.insert(
            "Cloudy Shard",
            Item {
                guid: "bcedb2e46cfcb9a45a9736062dbd6e47",
                item_type: ItemType::StoryArtifact,
                name: "Cloudy Shard",
                order_priority: 730,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "bcedb2e46cfcb9a45a9736062dbd6e47",
            Item {
                guid: "bcedb2e46cfcb9a45a9736062dbd6e47",
                item_type: ItemType::StoryArtifact,
                name: "Cloudy Shard",
                order_priority: 730,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // VALUABLEITEM_ADAMANTITEORE_NAME
        m.insert(
            "Adamantite Ore",
            Item {
                guid: "77a0a1cac957b4b4d8cfbb1ee9b2548c",
                item_type: ItemType::Valuable,
                name: "Adamantite Ore",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 8,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "77a0a1cac957b4b4d8cfbb1ee9b2548c",
            Item {
                guid: "77a0a1cac957b4b4d8cfbb1ee9b2548c",
                item_type: ItemType::Valuable,
                name: "Adamantite Ore",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 8,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // VALUABLEITEM_ALUMNITOKEN_NAME
        m.insert(
            "Alumni Token",
            Item {
                guid: "c067bbcbb9deb85488d779cf334e50db",
                item_type: ItemType::Valuable,
                name: "Alumni Token",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c067bbcbb9deb85488d779cf334e50db",
            Item {
                guid: "c067bbcbb9deb85488d779cf334e50db",
                item_type: ItemType::Valuable,
                name: "Alumni Token",
                order_priority: 0,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 1,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // VALUABLEITEM_OBSIDIANINGOT_NAME
        m.insert(
            "Obsidian Ingot",
            Item {
                guid: "4717dc86cf98f2d4794b40e910b63e61",
                item_type: ItemType::Valuable,
                name: "Obsidian Ingot",
                order_priority: 985,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "4717dc86cf98f2d4794b40e910b63e61",
            Item {
                guid: "4717dc86cf98f2d4794b40e910b63e61",
                item_type: ItemType::Valuable,
                name: "Obsidian Ingot",
                order_priority: 985,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // VALUABLEITEM_OBSIDIANORE_NAME
        m.insert(
            "Obsidian Ore",
            Item {
                guid: "7a9b2938fabc5434da21134b18b4e0ee",
                item_type: ItemType::Valuable,
                name: "Obsidian Ore",
                order_priority: 990,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 45,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "7a9b2938fabc5434da21134b18b4e0ee",
            Item {
                guid: "7a9b2938fabc5434da21134b18b4e0ee",
                item_type: ItemType::Valuable,
                name: "Obsidian Ore",
                order_priority: 990,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 45,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // VALUABLEITEM_SAPPHIREINGOT_NAME
        m.insert(
            "Sapphire Ingot",
            Item {
                guid: "79b159a1b2b69834a96cd56d5e02058c",
                item_type: ItemType::Valuable,
                name: "Sapphire Ingot",
                order_priority: 975,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 140,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "79b159a1b2b69834a96cd56d5e02058c",
            Item {
                guid: "79b159a1b2b69834a96cd56d5e02058c",
                item_type: ItemType::Valuable,
                name: "Sapphire Ingot",
                order_priority: 975,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 140,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // VALUABLEITEM_SAPPHIREORE_NAME
        m.insert(
            "Sapphire Ore",
            Item {
                guid: "c43421b626574174e9b5156141213250",
                item_type: ItemType::Valuable,
                name: "Sapphire Ore",
                order_priority: 980,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 55,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c43421b626574174e9b5156141213250",
            Item {
                guid: "c43421b626574174e9b5156141213250",
                item_type: ItemType::Valuable,
                name: "Sapphire Ore",
                order_priority: 980,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 55,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // VALUABLEITEM_TEALAMBERORE_NAME
        m.insert(
            "Teal Amber Ore",
            Item {
                guid: "33e568f23e2581d448041ac97fb8358d",
                item_type: ItemType::Valuable,
                name: "Teal Amber Ore",
                order_priority: 995,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 35,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "33e568f23e2581d448041ac97fb8358d",
            Item {
                guid: "33e568f23e2581d448041ac97fb8358d",
                item_type: ItemType::Valuable,
                name: "Teal Amber Ore",
                order_priority: 995,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 35,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // VALUABLEITEM_SHINYPEARL_NAME
        m.insert(
            "Shiny Pearl",
            Item {
                guid: "7337dc21759feae4a92aba8200c42898",
                item_type: ItemType::Valuable,
                name: "Shiny Pearl",
                order_priority: 1000,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 20,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "7337dc21759feae4a92aba8200c42898",
            Item {
                guid: "7337dc21759feae4a92aba8200c42898",
                item_type: ItemType::Valuable,
                name: "Shiny Pearl",
                order_priority: 1000,
                max_quantity: 255,
                buy_price: 0,
                sell_price: 20,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_VITRICSIMULACRUM_NAME
        m.insert(
            "Vitric Simulacrum",
            Item {
                guid: "ebe89a4a81e46c245bd98a337df06eb8",
                item_type: ItemType::Armor,
                name: "Vitric Simulacrum",
                order_priority: 220,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[PlayerPartyCharacter::Bst]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 53,
                magical_defense: 51,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "ebe89a4a81e46c245bd98a337df06eb8",
            Item {
                guid: "ebe89a4a81e46c245bd98a337df06eb8",
                item_type: ItemType::Armor,
                name: "Vitric Simulacrum",
                order_priority: 220,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[PlayerPartyCharacter::Bst]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 53,
                magical_defense: 51,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_VOLCANICARMOR_NAME
        m.insert(
            "Volcanic Armor",
            Item {
                guid: "0c8b9c021791a734892aaafde3b847b6",
                item_type: ItemType::Armor,
                name: "Volcanic Armor",
                order_priority: 175,
                max_quantity: 255,
                buy_price: 136,
                sell_price: 34,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 40,
                magical_defense: 32,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "0c8b9c021791a734892aaafde3b847b6",
            Item {
                guid: "0c8b9c021791a734892aaafde3b847b6",
                item_type: ItemType::Armor,
                name: "Volcanic Armor",
                order_priority: 175,
                max_quantity: 255,
                buy_price: 136,
                sell_price: 34,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 40,
                magical_defense: 32,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_WALNUTCORK_NAME
        m.insert(
            "Walnut Cork",
            Item {
                guid: "c3b22a46f99faba498bfbc7519cce88c",
                item_type: ItemType::Weapon,
                name: "Walnut Cork",
                order_priority: 150,
                max_quantity: 255,
                buy_price: 172,
                sell_price: 43,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 29,
                magical_attack: 39,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c3b22a46f99faba498bfbc7519cce88c",
            Item {
                guid: "c3b22a46f99faba498bfbc7519cce88c",
                item_type: ItemType::Weapon,
                name: "Walnut Cork",
                order_priority: 150,
                max_quantity: 255,
                buy_price: 172,
                sell_price: 43,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 29,
                magical_attack: 39,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_WITHEREDCORK_NAME
        m.insert(
            "Withered Cork",
            Item {
                guid: "441bec8771714944eb32ff53c082f8da",
                item_type: ItemType::Weapon,
                name: "Withered Cork",
                order_priority: 95,
                max_quantity: 255,
                buy_price: 112,
                sell_price: 28,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 17,
                magical_attack: 18,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "441bec8771714944eb32ff53c082f8da",
            Item {
                guid: "441bec8771714944eb32ff53c082f8da",
                item_type: ItemType::Weapon,
                name: "Withered Cork",
                order_priority: 95,
                max_quantity: 255,
                buy_price: 112,
                sell_price: 28,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 17,
                magical_attack: 18,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_DUBIOUS_DARE_NAME
        m.insert(
            "Dubious Dare",
            Item {
                guid: "5969aba2037e09b46823584da7ee0d15",
                item_type: ItemType::Relic,
                name: "Dubious Dare",
                order_priority: 20,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "5969aba2037e09b46823584da7ee0d15",
            Item {
                guid: "5969aba2037e09b46823584da7ee0d15",
                item_type: ItemType::Relic,
                name: "Dubious Dare",
                order_priority: 20,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_EXTRA_POCKETS_NAME
        m.insert(
            "Hidden Pockets",
            Item {
                guid: "4e0db461b8b133c4ab2536bc1016cf93",
                item_type: ItemType::Relic,
                name: "Hidden Pockets",
                order_priority: 945,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "4e0db461b8b133c4ab2536bc1016cf93",
            Item {
                guid: "4e0db461b8b133c4ab2536bc1016cf93",
                item_type: ItemType::Relic,
                name: "Hidden Pockets",
                order_priority: 945,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_SUPER_FISHING_LINE_NAME
        m.insert(
            "Stereofilament Line",
            Item {
                guid: "a8c2b79eb9a4bb84ba5876ddb96ec55b",
                item_type: ItemType::Relic,
                name: "Stereofilament Line",
                order_priority: 900,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a8c2b79eb9a4bb84ba5876ddb96ec55b",
            Item {
                guid: "a8c2b79eb9a4bb84ba5876ddb96ec55b",
                item_type: ItemType::Relic,
                name: "Stereofilament Line",
                order_priority: 900,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_SUPER_FISHING_REEL_NAME
        m.insert(
            "Bearing Reel",
            Item {
                guid: "662aa1ede20e4a14db1e29e258893a6c",
                item_type: ItemType::Relic,
                name: "Bearing Reel",
                order_priority: 905,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "662aa1ede20e4a14db1e29e258893a6c",
            Item {
                guid: "662aa1ede20e4a14db1e29e258893a6c",
                item_type: ItemType::Relic,
                name: "Bearing Reel",
                order_priority: 905,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_WIREPLATEARMOR_NAME
        m.insert(
            "Wireplate Armor",
            Item {
                guid: "67e51219a3a7ea14fa14e9eb4e8643e4",
                item_type: ItemType::Armor,
                name: "Wireplate Armor",
                order_priority: 185,
                max_quantity: 255,
                buy_price: 152,
                sell_price: 38,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 43,
                magical_defense: 36,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "67e51219a3a7ea14fa14e9eb4e8643e4",
            Item {
                guid: "67e51219a3a7ea14fa14e9eb4e8643e4",
                item_type: ItemType::Armor,
                name: "Wireplate Armor",
                order_priority: 185,
                max_quantity: 255,
                buy_price: 152,
                sell_price: 38,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 43,
                magical_defense: 36,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_SUPER_FISHING_POLE_HANDLE_NAME
        m.insert(
            "Mithreel Rod",
            Item {
                guid: "45ab00b468451b949ac17163040925fa",
                item_type: ItemType::Relic,
                name: "Mithreel Rod",
                order_priority: 910,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "45ab00b468451b949ac17163040925fa",
            Item {
                guid: "45ab00b468451b949ac17163040925fa",
                item_type: ItemType::Relic,
                name: "Mithreel Rod",
                order_priority: 910,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_GOLDEN_TOOTH_NAME
        m.insert(
            "Gold Tooth",
            Item {
                guid: "57597d27d25e6f74fb87c572b2b171d2",
                item_type: ItemType::Relic,
                name: "Gold Tooth",
                order_priority: 960,
                max_quantity: 1,
                buy_price: 225,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "57597d27d25e6f74fb87c572b2b171d2",
            Item {
                guid: "57597d27d25e6f74fb87c572b2b171d2",
                item_type: ItemType::Relic,
                name: "Gold Tooth",
                order_priority: 960,
                max_quantity: 1,
                buy_price: 225,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_GUARDIAN_AURA_NAME
        m.insert(
            "Guardian Aura",
            Item {
                guid: "def407b542a5c3d458a8c1fa59815693",
                item_type: ItemType::Relic,
                name: "Guardian Aura",
                order_priority: 995,
                max_quantity: 1,
                buy_price: 15,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "def407b542a5c3d458a8c1fa59815693",
            Item {
                guid: "def407b542a5c3d458a8c1fa59815693",
                item_type: ItemType::Relic,
                name: "Guardian Aura",
                order_priority: 995,
                max_quantity: 1,
                buy_price: 15,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_KNOWLEDGE_TOME_NAME
        m.insert(
            "Tome of Knowledge",
            Item {
                guid: "31de888ea125ddf458c85998a8958ef4",
                item_type: ItemType::Relic,
                name: "Tome of Knowledge",
                order_priority: 975,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "31de888ea125ddf458c85998a8958ef4",
            Item {
                guid: "31de888ea125ddf458c85998a8958ef4",
                item_type: ItemType::Relic,
                name: "Tome of Knowledge",
                order_priority: 975,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_PIRATE_PARROT_NAME
        m.insert(
            "Falcon-eyed Parrot",
            Item {
                guid: "e388991e8f7c5934993153ef7e50232f",
                item_type: ItemType::Relic,
                name: "Falcon-eyed Parrot",
                order_priority: 970,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e388991e8f7c5934993153ef7e50232f",
            Item {
                guid: "e388991e8f7c5934993153ef7e50232f",
                item_type: ItemType::Relic,
                name: "Falcon-eyed Parrot",
                order_priority: 970,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_SALIENT_SAILS_NAME
        m.insert(
            "Salient Sails",
            Item {
                guid: "1554ef53341beea43ab50edbe869f560",
                item_type: ItemType::Relic,
                name: "Salient Sails",
                order_priority: 965,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "1554ef53341beea43ab50edbe869f560",
            Item {
                guid: "1554ef53341beea43ab50edbe869f560",
                item_type: ItemType::Relic,
                name: "Salient Sails",
                order_priority: 965,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_SEQUENT_FLARE_NAME
        m.insert(
            "Sequent Flare",
            Item {
                guid: "1b0313fe821ba7c419b7b443416aa784",
                item_type: ItemType::Relic,
                name: "Sequent Flare",
                order_priority: 990,
                max_quantity: 1,
                buy_price: 15,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "1b0313fe821ba7c419b7b443416aa784",
            Item {
                guid: "1b0313fe821ba7c419b7b443416aa784",
                item_type: ItemType::Relic,
                name: "Sequent Flare",
                order_priority: 990,
                max_quantity: 1,
                buy_price: 15,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_SIXTH_SENSE_NAME
        m.insert(
            "Sixth Sense",
            Item {
                guid: "f2e2a73117e6d044d8b0ced0304293d1",
                item_type: ItemType::Relic,
                name: "Sixth Sense",
                order_priority: 955,
                max_quantity: 1,
                buy_price: 15,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f2e2a73117e6d044d8b0ced0304293d1",
            Item {
                guid: "f2e2a73117e6d044d8b0ced0304293d1",
                item_type: ItemType::Relic,
                name: "Sixth Sense",
                order_priority: 955,
                max_quantity: 1,
                buy_price: 15,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_SOLSTICEDIPLOMA_NAME
        m.insert(
            "Solstice Diploma",
            Item {
                guid: "28e41ac6234015840a9c976425d74ee9",
                item_type: ItemType::Relic,
                name: "Solstice Diploma",
                order_priority: 1010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "28e41ac6234015840a9c976425d74ee9",
            Item {
                guid: "28e41ac6234015840a9c976425d74ee9",
                item_type: ItemType::Relic,
                name: "Solstice Diploma",
                order_priority: 1010,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 0,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_SOLSTICEDOCTORATE_NAME
        m.insert(
            "Solstice Doctorate",
            Item {
                guid: "d804d17171feb8949927b61e871fc746",
                item_type: ItemType::Relic,
                name: "Solstice Doctorate",
                order_priority: 11,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "d804d17171feb8949927b61e871fc746",
            Item {
                guid: "d804d17171feb8949927b61e871fc746",
                item_type: ItemType::Relic,
                name: "Solstice Doctorate",
                order_priority: 11,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_SPEEDRUN_NAME
        m.insert(
            "Mark of the Speedrunner",
            Item {
                guid: "caf88d528688b284e80ce8860c9bb2b8",
                item_type: ItemType::Relic,
                name: "Mark of the Speedrunner",
                order_priority: 10,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "caf88d528688b284e80ce8860c9bb2b8",
            Item {
                guid: "caf88d528688b284e80ce8860c9bb2b8",
                item_type: ItemType::Relic,
                name: "Mark of the Speedrunner",
                order_priority: 10,
                max_quantity: 1,
                buy_price: 0,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_TACTICIAN_METTLE_NAME
        m.insert(
            "Tactician's Mettle",
            Item {
                guid: "c52a22809bcbb4443916cadd12aa01cd",
                item_type: ItemType::Relic,
                name: "Tactician's Mettle",
                order_priority: 35,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c52a22809bcbb4443916cadd12aa01cd",
            Item {
                guid: "c52a22809bcbb4443916cadd12aa01cd",
                item_type: ItemType::Relic,
                name: "Tactician's Mettle",
                order_priority: 35,
                max_quantity: 1,
                buy_price: 150,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // RELIC_TRUESTRIKE_PENDANT_NAME
        m.insert(
            "Truestrike Pendant",
            Item {
                guid: "c1962220eb589a642aa1a5856ef650d4",
                item_type: ItemType::Relic,
                name: "Truestrike Pendant",
                order_priority: 985,
                max_quantity: 1,
                buy_price: 45,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c1962220eb589a642aa1a5856ef650d4",
            Item {
                guid: "c1962220eb589a642aa1a5856ef650d4",
                item_type: ItemType::Relic,
                name: "Truestrike Pendant",
                order_priority: 985,
                max_quantity: 1,
                buy_price: 45,
                sell_price: 120,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_ROCK_LID_NAME
        m.insert(
            "Rock Lid",
            Item {
                guid: "7694b5b801204a74c888d67a49f06d42",
                item_type: ItemType::Weapon,
                name: "Rock Lid",
                order_priority: 30,
                max_quantity: 255,
                buy_price: 44,
                sell_price: 11,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 10,
                magical_attack: 9,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "7694b5b801204a74c888d67a49f06d42",
            Item {
                guid: "7694b5b801204a74c888d67a49f06d42",
                item_type: ItemType::Weapon,
                name: "Rock Lid",
                order_priority: 30,
                max_quantity: 255,
                buy_price: 44,
                sell_price: 11,
                equippable_by: Some(&[PlayerPartyCharacter::Garl]),
                physical_attack: 10,
                magical_attack: 9,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_REVENANTARMOR_NAME
        m.insert(
            "Revenant Armor",
            Item {
                guid: "a0890c10ae319b0409d43f08af673ffd",
                item_type: ItemType::Armor,
                name: "Revenant Armor",
                order_priority: 135,
                max_quantity: 255,
                buy_price: 48,
                sell_price: 12,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 23,
                magical_defense: 25,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "a0890c10ae319b0409d43f08af673ffd",
            Item {
                guid: "a0890c10ae319b0409d43f08af673ffd",
                item_type: ItemType::Armor,
                name: "Revenant Armor",
                order_priority: 135,
                max_quantity: 255,
                buy_price: 48,
                sell_price: 12,
                equippable_by: Some(&[PlayerPartyCharacter::Zale, PlayerPartyCharacter::Valere]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 23,
                magical_defense: 25,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_ROSEWOODCORK_NAME
        m.insert(
            "Rosewood Cork",
            Item {
                guid: "c2d85fc1508398e4a8c4db3e12594978",
                item_type: ItemType::Weapon,
                name: "Rosewood Cork",
                order_priority: 170,
                max_quantity: 255,
                buy_price: 172,
                sell_price: 43,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 34,
                magical_attack: 39,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "c2d85fc1508398e4a8c4db3e12594978",
            Item {
                guid: "c2d85fc1508398e4a8c4db3e12594978",
                item_type: ItemType::Weapon,
                name: "Rosewood Cork",
                order_priority: 170,
                max_quantity: 255,
                buy_price: 172,
                sell_price: 43,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 34,
                magical_attack: 39,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_SALIXCORK_NAME
        m.insert(
            "Salix Cork",
            Item {
                guid: "2a8606f2bf7f5224cadb8ae2c1d1b531",
                item_type: ItemType::Weapon,
                name: "Salix Cork",
                order_priority: 190,
                max_quantity: 255,
                buy_price: 172,
                sell_price: 43,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 37,
                magical_attack: 41,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "2a8606f2bf7f5224cadb8ae2c1d1b531",
            Item {
                guid: "2a8606f2bf7f5224cadb8ae2c1d1b531",
                item_type: ItemType::Weapon,
                name: "Salix Cork",
                order_priority: 190,
                max_quantity: 255,
                buy_price: 172,
                sell_price: 43,
                equippable_by: Some(&[PlayerPartyCharacter::Reshan]),
                physical_attack: 37,
                magical_attack: 41,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_SCRIMSHAWEDBLADE_NAME
        m.insert(
            "Scrimshawed Sword",
            Item {
                guid: "7ce9f696d3089434fbff10c7653d0026",
                item_type: ItemType::Weapon,
                name: "Scrimshawed Sword",
                order_priority: 40,
                max_quantity: 255,
                buy_price: 80,
                sell_price: 20,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 13,
                magical_attack: 10,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "7ce9f696d3089434fbff10c7653d0026",
            Item {
                guid: "7ce9f696d3089434fbff10c7653d0026",
                item_type: ItemType::Weapon,
                name: "Scrimshawed Sword",
                order_priority: 40,
                max_quantity: 255,
                buy_price: 80,
                sell_price: 20,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 13,
                magical_attack: 10,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_SERAI_NAME
        m.insert(
            "Silver Daggers",
            Item {
                guid: "6dfc4e7a1624a9c4eaaf2ae75c336b5c",
                item_type: ItemType::Weapon,
                name: "Silver Daggers",
                order_priority: 45,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 10,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 5,
                magical_attack: 8,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "6dfc4e7a1624a9c4eaaf2ae75c336b5c",
            Item {
                guid: "6dfc4e7a1624a9c4eaaf2ae75c336b5c",
                item_type: ItemType::Weapon,
                name: "Silver Daggers",
                order_priority: 45,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 10,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 5,
                magical_attack: 8,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_SHIMMERINGBLADE_NAME
        m.insert(
            "Shimmering Sword",
            Item {
                guid: "10e93ab756357f142b3c6ef3ec7a7c4b",
                item_type: ItemType::Weapon,
                name: "Shimmering Sword",
                order_priority: 60,
                max_quantity: 255,
                buy_price: 80,
                sell_price: 20,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 15,
                magical_attack: 10,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "10e93ab756357f142b3c6ef3ec7a7c4b",
            Item {
                guid: "10e93ab756357f142b3c6ef3ec7a7c4b",
                item_type: ItemType::Weapon,
                name: "Shimmering Sword",
                order_priority: 60,
                max_quantity: 255,
                buy_price: 80,
                sell_price: 20,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 15,
                magical_attack: 10,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_SHIMMERINGDAGGERS_NAME
        m.insert(
            "Shimmering Daggers",
            Item {
                guid: "b883d3f8ffbc78040ade7dc4675ac1c6",
                item_type: ItemType::Weapon,
                name: "Shimmering Daggers",
                order_priority: 50,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 10,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 9,
                magical_attack: 14,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "b883d3f8ffbc78040ade7dc4675ac1c6",
            Item {
                guid: "b883d3f8ffbc78040ade7dc4675ac1c6",
                item_type: ItemType::Weapon,
                name: "Shimmering Daggers",
                order_priority: 50,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 10,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 9,
                magical_attack: 14,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_SHIMMERING_SHARD_NAME
        m.insert(
            "Shimmering Shard",
            Item {
                guid: "71ff97ab95cbcd84985e62255cbca323",
                item_type: ItemType::Trinket,
                name: "Shimmering Shard",
                order_priority: 25,
                max_quantity: 255,
                buy_price: 195,
                sell_price: 45,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "71ff97ab95cbcd84985e62255cbca323",
            Item {
                guid: "71ff97ab95cbcd84985e62255cbca323",
                item_type: ItemType::Trinket,
                name: "Shimmering Shard",
                order_priority: 25,
                max_quantity: 255,
                buy_price: 195,
                sell_price: 45,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_SHIMMERINGSTAFF_NAME
        m.insert(
            "Shimmering Staff",
            Item {
                guid: "f7d49514f7bfca34b942d95722b76a32",
                item_type: ItemType::Weapon,
                name: "Shimmering Staff",
                order_priority: 55,
                max_quantity: 255,
                buy_price: 76,
                sell_price: 19,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 16,
                magical_attack: 8,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "f7d49514f7bfca34b942d95722b76a32",
            Item {
                guid: "f7d49514f7bfca34b942d95722b76a32",
                item_type: ItemType::Weapon,
                name: "Shimmering Staff",
                order_priority: 55,
                max_quantity: 255,
                buy_price: 76,
                sell_price: 19,
                equippable_by: Some(&[PlayerPartyCharacter::Valere]),
                physical_attack: 16,
                magical_attack: 8,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_SHROOMYSHIV_NAME
        m.insert(
            "Shroomy Shivs",
            Item {
                guid: "aa8b7c8001ca94d4eaa65d1ffb58a5e2",
                item_type: ItemType::Weapon,
                name: "Shroomy Shivs",
                order_priority: 100,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 20,
                magical_attack: 24,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "aa8b7c8001ca94d4eaa65d1ffb58a5e2",
            Item {
                guid: "aa8b7c8001ca94d4eaa65d1ffb58a5e2",
                item_type: ItemType::Weapon,
                name: "Shroomy Shivs",
                order_priority: 100,
                max_quantity: 255,
                buy_price: 144,
                sell_price: 36,
                equippable_by: Some(&[PlayerPartyCharacter::Serai]),
                physical_attack: 20,
                magical_attack: 24,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // TRINKET_SIGNETOFCLARITY_NAME
        m.insert(
            "Signet of Clarity",
            Item {
                guid: "62dcbc88129046140a17fb2790ad258b",
                item_type: ItemType::Trinket,
                name: "Signet of Clarity",
                order_priority: 70,
                max_quantity: 255,
                buy_price: 190,
                sell_price: 45,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "62dcbc88129046140a17fb2790ad258b",
            Item {
                guid: "62dcbc88129046140a17fb2790ad258b",
                item_type: ItemType::Trinket,
                name: "Signet of Clarity",
                order_priority: 70,
                max_quantity: 255,
                buy_price: 190,
                sell_price: 45,
                equippable_by: Some(&[
                    PlayerPartyCharacter::Zale,
                    PlayerPartyCharacter::Valere,
                    PlayerPartyCharacter::Garl,
                    PlayerPartyCharacter::Serai,
                    PlayerPartyCharacter::Reshan,
                    PlayerPartyCharacter::Bst,
                ]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // WEAPON_SILVER_BLADE_NAME
        m.insert(
            "Silver Sword",
            Item {
                guid: "71cfc6d93b292d3499c472b73e9daae1",
                item_type: ItemType::Weapon,
                name: "Silver Sword",
                order_priority: 25,
                max_quantity: 255,
                buy_price: 52,
                sell_price: 13,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 9,
                magical_attack: 7,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "71cfc6d93b292d3499c472b73e9daae1",
            Item {
                guid: "71cfc6d93b292d3499c472b73e9daae1",
                item_type: ItemType::Weapon,
                name: "Silver Sword",
                order_priority: 25,
                max_quantity: 255,
                buy_price: 52,
                sell_price: 13,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 9,
                magical_attack: 7,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // ARMOR_SKYARMOR_NAME
        m.insert(
            "Sky Armor",
            Item {
                guid: "e1e3c71bb3b7e034684d195c2fa5968c",
                item_type: ItemType::Armor,
                name: "Sky Armor",
                order_priority: 170,
                max_quantity: 255,
                buy_price: 136,
                sell_price: 34,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 38,
                magical_defense: 35,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "e1e3c71bb3b7e034684d195c2fa5968c",
            Item {
                guid: "e1e3c71bb3b7e034684d195c2fa5968c",
                item_type: ItemType::Armor,
                name: "Sky Armor",
                order_priority: 170,
                max_quantity: 255,
                buy_price: 136,
                sell_price: 34,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 38,
                magical_defense: 35,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        // SNACK_BASICSALAD_NAME
        m.insert(
            "Basic Salad",
            Item {
                guid: "68c337d503ebc7e4a837052ed38452b5",
                item_type: ItemType::Snack,
                name: "Basic Salad",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 5,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 35,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "68c337d503ebc7e4a837052ed38452b5",
            Item {
                guid: "68c337d503ebc7e4a837052ed38452b5",
                item_type: ItemType::Snack,
                name: "Basic Salad",
                order_priority: 20,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 5,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 35,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_STRAWBERRYJAM_NAME
        m.insert(
            "Berry Jam",
            Item {
                guid: "74a5e5d26b56b7e429c36d1acb84a120",
                item_type: ItemType::Snack,
                name: "Berry Jam",
                order_priority: 9,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 5,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 5,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "74a5e5d26b56b7e429c36d1acb84a120",
            Item {
                guid: "74a5e5d26b56b7e429c36d1acb84a120",
                item_type: ItemType::Snack,
                name: "Berry Jam",
                order_priority: 9,
                max_quantity: 255,
                buy_price: 10,
                sell_price: 5,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 5,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_BOUILLABAISSE_NAME
        m.insert(
            "Bouillabaisse",
            Item {
                guid: "f2868459715d3df45ad1da473d197ac6",
                item_type: ItemType::Snack,
                name: "Bouillabaisse",
                order_priority: 45,
                max_quantity: 255,
                buy_price: 28,
                sell_price: 14,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 110,
                mp_to_restore: 6,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "f2868459715d3df45ad1da473d197ac6",
            Item {
                guid: "f2868459715d3df45ad1da473d197ac6",
                item_type: ItemType::Snack,
                name: "Bouillabaisse",
                order_priority: 45,
                max_quantity: 255,
                buy_price: 28,
                sell_price: 14,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 110,
                mp_to_restore: 6,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_BRAISÉ_NAME
        m.insert(
            "Braisé",
            Item {
                guid: "512b3f049d35cbd4eb4648aa5a1105f6",
                item_type: ItemType::Snack,
                name: "Braisé",
                order_priority: 79,
                max_quantity: 255,
                buy_price: 76,
                sell_price: 38,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 299,
                mp_to_restore: 29,
                // Restore mode
                hp_use_percent: 1,
                mp_use_percent: 1,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "512b3f049d35cbd4eb4648aa5a1105f6",
            Item {
                guid: "512b3f049d35cbd4eb4648aa5a1105f6",
                item_type: ItemType::Snack,
                name: "Braisé",
                order_priority: 79,
                max_quantity: 255,
                buy_price: 76,
                sell_price: 38,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 299,
                mp_to_restore: 29,
                // Restore mode
                hp_use_percent: 1,
                mp_use_percent: 1,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_CHAMPIONOMELETTE_NAME
        m.insert(
            "Champion's Omelette",
            Item {
                guid: "5608bed7407aae34ba3012feba387b00",
                item_type: ItemType::Snack,
                name: "Champion's Omelette",
                order_priority: 70,
                max_quantity: 255,
                buy_price: 56,
                sell_price: 28,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 255,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "5608bed7407aae34ba3012feba387b00",
            Item {
                guid: "5608bed7407aae34ba3012feba387b00",
                item_type: ItemType::Snack,
                name: "Champion's Omelette",
                order_priority: 70,
                max_quantity: 255,
                buy_price: 56,
                sell_price: 28,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 255,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_APPLEPIE_NAME
        m.insert(
            "Croustade",
            Item {
                guid: "46bfb5ae7593f8f4bb870c9644635a99",
                item_type: ItemType::Snack,
                name: "Croustade",
                order_priority: 47,
                max_quantity: 255,
                buy_price: 64,
                sell_price: 32,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 80,
                mp_to_restore: 7,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "46bfb5ae7593f8f4bb870c9644635a99",
            Item {
                guid: "46bfb5ae7593f8f4bb870c9644635a99",
                item_type: ItemType::Snack,
                name: "Croustade",
                order_priority: 47,
                max_quantity: 255,
                buy_price: 64,
                sell_price: 32,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 80,
                mp_to_restore: 7,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_CHAUDRÉE_NAME
        m.insert(
            "Chaudrée",
            Item {
                guid: "71853a3bd516511428ea6d23bc7eb34f",
                item_type: ItemType::Snack,
                name: "Chaudrée",
                order_priority: 24,
                max_quantity: 255,
                buy_price: 12,
                sell_price: 6,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 8,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "71853a3bd516511428ea6d23bc7eb34f",
            Item {
                guid: "71853a3bd516511428ea6d23bc7eb34f",
                item_type: ItemType::Snack,
                name: "Chaudrée",
                order_priority: 24,
                max_quantity: 255,
                buy_price: 12,
                sell_price: 6,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 8,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_MFISHPIE_NAME
        m.insert(
            "Mooncradle Fish Pie",
            Item {
                guid: "6e613ca39c27bff49ae0da1773c6ab03",
                item_type: ItemType::Snack,
                name: "Mooncradle Fish Pie",
                order_priority: 38,
                max_quantity: 255,
                buy_price: 36,
                sell_price: 18,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 55,
                mp_to_restore: 5,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "6e613ca39c27bff49ae0da1773c6ab03",
            Item {
                guid: "6e613ca39c27bff49ae0da1773c6ab03",
                item_type: ItemType::Snack,
                name: "Mooncradle Fish Pie",
                order_priority: 38,
                max_quantity: 255,
                buy_price: 36,
                sell_price: 18,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 55,
                mp_to_restore: 5,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_GOURMETBURGER_NAME
        m.insert(
            "Gourmet Burger",
            Item {
                guid: "918caccd92d8f7540929427a743d1f15",
                item_type: ItemType::Snack,
                name: "Gourmet Burger",
                order_priority: 55,
                max_quantity: 255,
                buy_price: 40,
                sell_price: 20,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 175,
                mp_to_restore: 7,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "918caccd92d8f7540929427a743d1f15",
            Item {
                guid: "918caccd92d8f7540929427a743d1f15",
                item_type: ItemType::Snack,
                name: "Gourmet Burger",
                order_priority: 55,
                max_quantity: 255,
                buy_price: 40,
                sell_price: 20,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 175,
                mp_to_restore: 7,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_HERBEDFILET_NAME
        m.insert(
            "Herbed Filet",
            Item {
                guid: "88bc64e502aab364f800ff8b94427f5f",
                item_type: ItemType::Snack,
                name: "Herbed Filet",
                order_priority: 25,
                max_quantity: 255,
                buy_price: 8,
                sell_price: 4,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 50,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "88bc64e502aab364f800ff8b94427f5f",
            Item {
                guid: "88bc64e502aab364f800ff8b94427f5f",
                item_type: ItemType::Snack,
                name: "Herbed Filet",
                order_priority: 25,
                max_quantity: 255,
                buy_price: 8,
                sell_price: 4,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 50,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_HEARTYSTEW_NAME
        m.insert(
            "Hearty Stew",
            Item {
                guid: "a4a1c78380553e74bb92899ddcd62440",
                item_type: ItemType::Snack,
                name: "Hearty Stew",
                order_priority: 40,
                max_quantity: 255,
                buy_price: 32,
                sell_price: 16,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 90,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "a4a1c78380553e74bb92899ddcd62440",
            Item {
                guid: "a4a1c78380553e74bb92899ddcd62440",
                item_type: ItemType::Snack,
                name: "Hearty Stew",
                order_priority: 40,
                max_quantity: 255,
                buy_price: 32,
                sell_price: 16,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 90,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_LASAGNA_NAME
        m.insert(
            "Lasagna",
            Item {
                guid: "e8908ed6c2bd60f4bb4042067baa2a8c",
                item_type: ItemType::Snack,
                name: "Lasagna",
                order_priority: 68,
                max_quantity: 255,
                buy_price: 72,
                sell_price: 36,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 180,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "e8908ed6c2bd60f4bb4042067baa2a8c",
            Item {
                guid: "e8908ed6c2bd60f4bb4042067baa2a8c",
                item_type: ItemType::Snack,
                name: "Lasagna",
                order_priority: 68,
                max_quantity: 255,
                buy_price: 72,
                sell_price: 36,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 180,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_MUSHROOMSCRAMBLE_NAME
        m.insert(
            "Mushroom Scramble",
            Item {
                guid: "c1866d6de70d4834b951c06d4ae23306",
                item_type: ItemType::Snack,
                name: "Mushroom Scramble",
                order_priority: 21,
                max_quantity: 255,
                buy_price: 18,
                sell_price: 9,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 35,
                mp_to_restore: 5,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "c1866d6de70d4834b951c06d4ae23306",
            Item {
                guid: "c1866d6de70d4834b951c06d4ae23306",
                item_type: ItemType::Snack,
                name: "Mushroom Scramble",
                order_priority: 21,
                max_quantity: 255,
                buy_price: 18,
                sell_price: 9,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 35,
                mp_to_restore: 5,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_LEGENDARYFEAST_NAME
        m.insert(
            "Legendary Feast",
            Item {
                guid: "425a2ecc6c4b87c4d89735500c6c9d95",
                item_type: ItemType::Snack,
                name: "Legendary Feast",
                order_priority: 80,
                max_quantity: 255,
                buy_price: 90,
                sell_price: 45,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 299,
                mp_to_restore: 29,
                // Restore mode
                hp_use_percent: 1,
                mp_use_percent: 1,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "425a2ecc6c4b87c4d89735500c6c9d95",
            Item {
                guid: "425a2ecc6c4b87c4d89735500c6c9d95",
                item_type: ItemType::Snack,
                name: "Legendary Feast",
                order_priority: 80,
                max_quantity: 255,
                buy_price: 90,
                sell_price: 45,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 299,
                mp_to_restore: 29,
                // Restore mode
                hp_use_percent: 1,
                mp_use_percent: 1,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_TPMUSHROOMPIE_NAME
        m.insert(
            "Mushroom Soup",
            Item {
                guid: "df724719b2fbf6e40a85e8f9bc4ccae2",
                item_type: ItemType::Snack,
                name: "Mushroom Soup",
                order_priority: 35,
                max_quantity: 255,
                buy_price: 26,
                sell_price: 13,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 40,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "df724719b2fbf6e40a85e8f9bc4ccae2",
            Item {
                guid: "df724719b2fbf6e40a85e8f9bc4ccae2",
                item_type: ItemType::Snack,
                name: "Mushroom Soup",
                order_priority: 35,
                max_quantity: 255,
                buy_price: 26,
                sell_price: 13,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 40,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_PAINDORE_NAME
        m.insert(
            "Pain Doré",
            Item {
                guid: "f6b47783588bed64e9fa44dba59b4a49",
                item_type: ItemType::Snack,
                name: "Pain Doré",
                order_priority: 49,
                max_quantity: 255,
                buy_price: 50,
                sell_price: 25,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 9,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "f6b47783588bed64e9fa44dba59b4a49",
            Item {
                guid: "f6b47783588bed64e9fa44dba59b4a49",
                item_type: ItemType::Snack,
                name: "Pain Doré",
                order_priority: 49,
                max_quantity: 255,
                buy_price: 50,
                sell_price: 25,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 9,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_PAPILLOTTE_NAME
        m.insert(
            "Papillote",
            Item {
                guid: "9151230a300ed934bbd8d6c1c659fc34",
                item_type: ItemType::Snack,
                name: "Papillote",
                order_priority: 31,
                max_quantity: 255,
                buy_price: 14,
                sell_price: 7,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "9151230a300ed934bbd8d6c1c659fc34",
            Item {
                guid: "9151230a300ed934bbd8d6c1c659fc34",
                item_type: ItemType::Snack,
                name: "Papillote",
                order_priority: 31,
                max_quantity: 255,
                buy_price: 14,
                sell_price: 7,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_PARFAIT_NAME
        m.insert(
            "Parfait",
            Item {
                guid: "fceb00c496ecb5844b14022ba450de53",
                item_type: ItemType::Snack,
                name: "Parfait",
                order_priority: 48,
                max_quantity: 255,
                buy_price: 24,
                sell_price: 12,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "fceb00c496ecb5844b14022ba450de53",
            Item {
                guid: "fceb00c496ecb5844b14022ba450de53",
                item_type: ItemType::Snack,
                name: "Parfait",
                order_priority: 48,
                max_quantity: 255,
                buy_price: 24,
                sell_price: 12,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_PEACHSTRUDEL_NAME
        m.insert(
            "Peach Strudel",
            Item {
                guid: "d508421fae4fe5549a21d767a5d7d10c",
                item_type: ItemType::Snack,
                name: "Peach Strudel",
                order_priority: 44,
                max_quantity: 255,
                buy_price: 32,
                sell_price: 16,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 12,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "d508421fae4fe5549a21d767a5d7d10c",
            Item {
                guid: "d508421fae4fe5549a21d767a5d7d10c",
                item_type: ItemType::Snack,
                name: "Peach Strudel",
                order_priority: 44,
                max_quantity: 255,
                buy_price: 32,
                sell_price: 16,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 12,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_POUTINE_NAME
        m.insert(
            "Poutine",
            Item {
                guid: "8af2d2fccb35a0f43a965bbdd7a993f1",
                item_type: ItemType::Snack,
                name: "Poutine",
                order_priority: 78,
                max_quantity: 255,
                buy_price: 32,
                sell_price: 16,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "8af2d2fccb35a0f43a965bbdd7a993f1",
            Item {
                guid: "8af2d2fccb35a0f43a965bbdd7a993f1",
                item_type: ItemType::Snack,
                name: "Poutine",
                order_priority: 78,
                max_quantity: 255,
                buy_price: 32,
                sell_price: 16,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_PUDDINGCHÔMEUR_NAME
        m.insert(
            "Pudding Chômeur",
            Item {
                guid: "52004597d9485634dbd4fca16b151c89",
                item_type: ItemType::Snack,
                name: "Pudding Chômeur",
                order_priority: 69,
                max_quantity: 255,
                buy_price: 36,
                sell_price: 18,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 20,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 1,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "52004597d9485634dbd4fca16b151c89",
            Item {
                guid: "52004597d9485634dbd4fca16b151c89",
                item_type: ItemType::Snack,
                name: "Pudding Chômeur",
                order_priority: 69,
                max_quantity: 255,
                buy_price: 36,
                sell_price: 18,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 20,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 1,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_ROASTSANDWICH_NAME
        m.insert(
            "Roast Sandwich",
            Item {
                guid: "520e9f55dc3432a498ec389a59d7401f",
                item_type: ItemType::Snack,
                name: "Roast Sandwich",
                order_priority: 30,
                max_quantity: 255,
                buy_price: 16,
                sell_price: 8,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 60,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "520e9f55dc3432a498ec389a59d7401f",
            Item {
                guid: "520e9f55dc3432a498ec389a59d7401f",
                item_type: ItemType::Snack,
                name: "Roast Sandwich",
                order_priority: 30,
                max_quantity: 255,
                buy_price: 16,
                sell_price: 8,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 60,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_SASHIMI_NAME
        m.insert(
            "Sashimi",
            Item {
                guid: "92dbbdececf32284eb6868f3529ff7ac",
                item_type: ItemType::Snack,
                name: "Sashimi",
                order_priority: 50,
                max_quantity: 255,
                buy_price: 18,
                sell_price: 9,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 155,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "92dbbdececf32284eb6868f3529ff7ac",
            Item {
                guid: "92dbbdececf32284eb6868f3529ff7ac",
                item_type: ItemType::Snack,
                name: "Sashimi",
                order_priority: 50,
                max_quantity: 255,
                buy_price: 18,
                sell_price: 9,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 155,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_TATAKI_NAME
        m.insert(
            "Tataki",
            Item {
                guid: "b504e699b3af5ab4faa0b252ff7d49f6",
                item_type: ItemType::Snack,
                name: "Tataki",
                order_priority: 65,
                max_quantity: 255,
                buy_price: 48,
                sell_price: 24,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 135,
                mp_to_restore: 8,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "b504e699b3af5ab4faa0b252ff7d49f6",
            Item {
                guid: "b504e699b3af5ab4faa0b252ff7d49f6",
                item_type: ItemType::Snack,
                name: "Tataki",
                order_priority: 65,
                max_quantity: 255,
                buy_price: 48,
                sell_price: 24,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 135,
                mp_to_restore: 8,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_TOMATOCLUB_NAME
        m.insert(
            "Tomato Club",
            Item {
                guid: "1aafd2f9435698349a63a99837e9104e",
                item_type: ItemType::Snack,
                name: "Tomato Club",
                order_priority: 10,
                max_quantity: 255,
                buy_price: 8,
                sell_price: 4,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 20,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "1aafd2f9435698349a63a99837e9104e",
            Item {
                guid: "1aafd2f9435698349a63a99837e9104e",
                item_type: ItemType::Snack,
                name: "Tomato Club",
                order_priority: 10,
                max_quantity: 255,
                buy_price: 8,
                sell_price: 4,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 20,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // SNACK_YAKITORISHRIMP_NAME
        m.insert(
            "Yakitori Shrimp",
            Item {
                guid: "547cb5586279b80489ace061c33eb861",
                item_type: ItemType::Snack,
                name: "Yakitori Shrimp",
                order_priority: 60,
                max_quantity: 255,
                buy_price: 18,
                sell_price: 9,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 15,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        m.insert(
            "547cb5586279b80489ace061c33eb861",
            Item {
                guid: "547cb5586279b80489ace061c33eb861",
                item_type: ItemType::Snack,
                name: "Yakitori Shrimp",
                order_priority: 60,
                max_quantity: 255,
                buy_price: 18,
                sell_price: 9,
                equippable_by: None,
                physical_attack: 0,
                magical_attack: 0,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 15,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 1.0,
                mp_percent_to_restore: 1.0,
            },
        );

        // WEAPON_SOLENSLIGHT_NAME
        m.insert(
            "Sun Blade",
            Item {
                guid: "8f96ab39454c54641812c57f7b332481",
                item_type: ItemType::Weapon,
                name: "Sun Blade",
                order_priority: 255,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 50,
                magical_attack: 35,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m.insert(
            "8f96ab39454c54641812c57f7b332481",
            Item {
                guid: "8f96ab39454c54641812c57f7b332481",
                item_type: ItemType::Weapon,
                name: "Sun Blade",
                order_priority: 255,
                max_quantity: 255,
                buy_price: 168,
                sell_price: 42,
                equippable_by: Some(&[PlayerPartyCharacter::Zale]),
                physical_attack: 50,
                magical_attack: 35,
                physical_defense: 0,
                magical_defense: 0,
                hp_to_restore: 0,
                mp_to_restore: 0,
                // Restore mode
                hp_use_percent: 0,
                mp_use_percent: 0,
                // 0.0-1.0
                hp_percent_to_restore: 0.0,
                mp_percent_to_restore: 0.0,
            },
        );

        m
    })
}

#[cfg(test)]
mod tests {
    // Include the library
    use crate::items::*;

    #[test]
    fn read_from_all_test() -> std::io::Result<()> {
        // initialize
        let items = all_items();

        assert!(items.get("Abacus").is_some());

        Ok(())
    }

    #[test]
    fn can_query_by_name_or_guid_test() -> std::io::Result<()> {
        // initialize
        let items = all_items();

        let abacus = items.get("Abacus").unwrap();
        let guid = items.get("6a39fd9b315b53a4faa9e3736d820eff").unwrap();

        assert_eq!(abacus.guid, guid.guid);
        assert!(items.get("NOT_IN_THE_LIST").is_none());

        Ok(())
    }

    #[test]
    fn can_query_by_armor_test() -> std::io::Result<()> {
        // initialize
        let armor = armor();

        assert!(armor.get("Abacus").is_none());
        assert!(armor.get("NOT_IN_THE_LIST").is_none());

        assert!(armor.get("Adventurer's Vest").is_some());
        assert!(armor.get("3ac3907f841cc2a40bd0fdce51cd52e2").is_some());

        Ok(())
    }
}
