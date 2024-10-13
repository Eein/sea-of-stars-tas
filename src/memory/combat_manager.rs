use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;

impl Default for MemoryManager<CombatManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "CombatManager".to_string(),
            data: CombatManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Default, Debug)]
pub struct CombatManagerData {
    pub encounter_active: bool,
}

impl MemoryManagerUpdate for CombatManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_encounter_active(&memory_context)?;

        Ok(())
    }
}

impl CombatManagerData {
    pub fn update_encounter_active(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(encounter_done) =
            memory_context.follow_fields::<u8>(&["currentEncounter", "encounterDone"])
        {
            self.encounter_active = matches!(encounter_done, 0)
        }

        Ok(())
    }
}
