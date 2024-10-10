use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;

impl Default for MemoryManager<CurrencyManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "CurrencyManager".to_string(),
            data: CurrencyManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Default, Debug)]
pub struct CurrencyManagerData {
    pub money: u64,
}

impl MemoryManagerUpdate for CurrencyManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_currencies(&memory_context)?;

        Ok(())
    }
}

impl CurrencyManagerData {
    // Currencies are in a dict, which a
    pub fn update_currencies(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(currency_ptr) = memory_context.read_pointer_path::<u64>(&[0x28, 0x18]) {
            let count = memory_context.read_pointer::<u64>(currency_ptr + 0x30)?;
            self.money = count;
        }
        Ok(())
    }
}
