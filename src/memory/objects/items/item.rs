use crate::memory::objects::character::PlayerPartyCharacter;

#[derive(Debug, Default)]
pub struct Item<'a, T> {
    pub guid: &'a str,
    pub item_type: ItemType,
    pub name: &'a str,
    pub order_prio: u32,
    pub cost: u32,
    pub sell_value: u32,
    pub meta: T,
}

// Menu that the item belongs to.

#[derive(Default, Debug)]
pub struct EquippableItem {
    pub equippable_by: Option<&'static [PlayerPartyCharacter]>,
    pub phy_def: u32,
    pub mag_def: u32,
    pub phy_atk: u32,
    pub mag_atk: u32,
}

#[derive(Default, Debug)]
pub struct FoodItem {
    pub is_aoe: bool,
    pub hp_to_restore: u32,
    pub mp_to_restore: u32,
    // Restore mode
    pub hp_use_percent: bool,
    pub mp_use_percent: bool,
    // 0.0-1.0
    pub hp_percent_to_restore: f32,
    pub mp_percent_to_restore: f32,
}

// No inner values
#[derive(Default, Debug)]
pub struct StandardItem;
