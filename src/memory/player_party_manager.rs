use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;
use vec3_rs::Vector3;

impl Default for MemoryManager<PlayerPartyManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "PlayerPartyManager".to_string(),
            data: PlayerPartyManagerData::default(),
            manager: UnityMemoryManager {
                ..UnityMemoryManager::default()
            },
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Default, Debug)]
pub struct PlayerPartyManagerData {
    pub position: Vector3<f32>,
}

impl MemoryManagerUpdate for PlayerPartyManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_position(&memory_context)?;

        Ok(())
    }
}

impl PlayerPartyManagerData {
    // This function updates the users position for nav helper
    // Unfortunately because some classes here dont have true objects
    // this by using follow_fields since while we are going up the chain
    // we may be unable to query an object without a fields_base
    pub fn update_position(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Some(leader) = memory_context.get_field_offset("leader") {
            let current_position_ptr =
                memory_context.read_pointer_path_without_read(&[leader.into(), 0x90, 0x84])?;
            let x = memory_context.read_pointer::<f32>(current_position_ptr)?;
            let y = memory_context.read_pointer::<f32>(current_position_ptr + 0x4)?;
            let z = memory_context.read_pointer::<f32>(current_position_ptr + 0x8)?;
            self.position = Vector3::new(x, y, z);
        }
        Ok(())
    }
}
