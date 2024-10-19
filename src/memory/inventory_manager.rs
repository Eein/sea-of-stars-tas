use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use data::prelude::all_items;
use log::info;
use memory::game_engine::il2cpp::unity_serializable_dictionary::*;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::{MemoryError, Process};
use memory::string::ArrayWString;
use std::hash::Hash;

#[derive(Default, Debug)]
pub struct InventoryManagerData {
    // This references QuantityByInventoryItemReference which is a
    // SerializableDictionary<InventoryItemReference, int> which is
    // T1 = InventoryItemName
    // T2 = InventoryItemQuantity
    // In memory you may see the values packed within 0x20 Dictionary 0x18 Entries
    // With a starting point of 0x20; from there the memory is laid out like:
    // 0x20 Metadata (unused, for internal stuff)
    // 0x28 InventoryItemReference
    //   (first 20 bytes are metadata)
    //   0x14 - beginning of utf8 GUID
    // 0x30 Quantity
    // ... so on until count is met or NULL pointer
    // These types can theoretically originally have a `next` link for key collisions
    // so be on the watch for missing items
    pub items: UnitySerializableDictionary<InventoryItemName, InventoryItemQuantity>,
}

impl Default for MemoryManager<InventoryManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "InventoryManager".to_string(),
            data: InventoryManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

impl MemoryManagerUpdate for InventoryManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_items(&memory_context)?;

        Ok(())
    }
}

impl InventoryManagerData {
    pub fn update_items(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        // If currentInventory != 0x0
        if let Ok(items_ptr) = memory_context.follow_fields::<u64>(&["ownedInventoryItems"]) {
            if items_ptr != 0x0 {
                if let Ok(items) = UnitySerializableDictionary::<
                    InventoryItemName,
                    InventoryItemQuantity,
                >::read(memory_context.process, items_ptr)
                {
                    self.items = items
                };
            }
        }

        Ok(())
    }
}

#[derive(Default, Debug, Eq, PartialEq, Hash)]
pub struct InventoryItemName(pub String);

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct InventoryItemQuantity(pub u64);

impl UnitySerializableDictKey for InventoryItemName {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        if let Ok(guid) = process.read::<ArrayWString<128>>(item_ptr + 0x14) {
            match String::from_utf16(guid.as_slice()) {
                Ok(value) => match all_items().get(value.as_str()) {
                    Some(item) => Ok(InventoryItemName(item.name.to_string())),
                    None => Ok(InventoryItemName("UNKNOWN ITEM NAME".to_string())),
                },
                Err(_) => Err(MemoryError::Unset),
            }
        } else {
            Err(MemoryError::ReadError)
        }
    }
}

impl UnitySerializableDictValue for InventoryItemQuantity {
    fn read(_process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        // For this, the value is simply stored at the pointer
        Ok(InventoryItemQuantity(item_ptr))
    }
}
