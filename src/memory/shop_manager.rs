use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::game_engine::il2cpp::unity_list::{UnityItem, UnityList};
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::{MemoryError, Process};
use memory::string::ArrayWString;

impl Default for MemoryManager<ShopManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "ShopManager".to_string(),
            data: ShopManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Default, Debug)]
pub struct ShopManagerData {
    pub items: UnityList<ShopItem>,
}

impl MemoryManagerUpdate for ShopManagerData {
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

impl ShopManagerData {
    pub fn update_items(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        // If currentShop != 0x0
        if let Ok(items_ptr) = memory_context.follow_fields::<u64>(&["currentShop", "itemsToSell"])
        {
            if items_ptr != 0x0 {
                let items = UnityList::<ShopItem>::read(memory_context.process, items_ptr)?;
                self.items = items;
            }
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ShopItem {
    // currentShop -> itemsToSell -> _items -> item[x] -> guid
    pub guid: String,
}

impl UnityItem for ShopItem {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        if let Ok(guid) = process.read_pointer_path::<ArrayWString<128>>(item_ptr, &[0x14]) {
            match String::from_utf16(guid.as_slice()) {
                Ok(value) => Ok(ShopItem { guid: value }),
                Err(_) => Err(MemoryError::Unset),
            }
        } else {
            Err(MemoryError::ReadError)
        }
    }
}
