use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::game_engine::il2cpp::unity_list::{UnityItem, UnityList};
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::{MemoryError, Process};
use memory::string::ArrayWString;

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

#[derive(Default, Debug)]
pub struct InventoryManagerData {
    pub items: UnityList<InventoryItem>,
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
        if let Ok(items_ptr) =
            memory_context.follow_fields::<u64>(&["currentInventory", "itemsToSell"])
        {
            if items_ptr != 0x0 {
                let items = UnityList::<InventoryItem>::read(memory_context.process, items_ptr)?;
                self.items = items;
            }
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct InventoryItem {
    // currentInventory -> itemsToSell -> _items -> item[x] -> guid
    pub guid: String,
}

impl UnityItem for InventoryItem {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        if let Ok(guid) = process.read_pointer_path::<ArrayWString<128>>(item_ptr, &[0x14]) {
            match String::from_utf16(guid.as_slice()) {
                Ok(value) => Ok(InventoryItem { guid: value }),
                Err(_) => Err(MemoryError::Unset),
            }
        } else {
            Err(MemoryError::ReadError)
        }
    }
}
