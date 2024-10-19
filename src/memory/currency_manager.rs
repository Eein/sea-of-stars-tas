use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;

#[derive(Default, Debug)]
pub struct CurrencyManagerData {
    pub money: u64,
}

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
    // Currencies are in a C# Dictionary<K,V> - this is complicated to parse out of memory so we'll
    // get the value directly.
    // In this case I was unable to locate the currency reference and it did not seem to be
    // following standard C# Dict protocol for Dictionary<CurrencyReference, Int>
    // Interally on CE it looked like the following:
    // 0x18 - _entries
    //   0x18 - Count -> 3
    //   0x20 - Item[0] -> Pointer to unusable (P->FFFFFFFFFFFFFFXXXX)
    //   0x28 - Item[1] -> Pointer to unusable (P->valid looking pointer)
    //   0x30 - Item[3] -> Currency Int (correct value in this case)
    //   0x38 - Item[4] -> P->00000000
    // This doesn't seem to match up to the capabilities of C# in the version used to compile the
    // game. Possibly SOA?
    //
    // TODO(eein): attempt to implement UnityDict another time.
    pub fn update_currencies(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(currency_ptr) = memory_context.read_pointer_path::<u64>(&[0x28, 0x18]) {
            if currency_ptr != 0x0 {
                let money = memory_context.read_pointer::<u64>(currency_ptr + 0x30)?;
                self.money = money;
            }
        }
        Ok(())
    }
}
