use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use memory::memory_manager::unity::UnityMemoryManager;
use memory::process::Error;
use vec3_rs::Vector3;
use log::info;

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
    ) -> Result<(), Error> {
        match self.update_position(ctx, manager) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
}

impl PlayerPartyManagerData {
    // This function updates the users position for nav helper
    // Unfortunately because some classes here dont have true objects
    // this by using follow_fields since while we are going up the chain
    // we may be unable to query an object without a fields_base
    pub fn update_position(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), Error> {
        if let Some(class) = manager.class {
            if let Some(process) = &ctx.process {
                if let Some(module) = &ctx.module {
                    if let Some(singleton) = manager.singleton {
                        if let Some(leader) = class.get_field_offset(process, module, "leader") {
                            let current_position_ptr = process.read_pointer_path_without_read(
                                singleton.class,
                                &[leader.into(), 0x90, 0x84],
                            )?;
                            let x = process.read_pointer::<f32>(current_position_ptr)?;
                            let y = process.read_pointer::<f32>(current_position_ptr + 0x4)?;
                            let z = process.read_pointer::<f32>(current_position_ptr + 0x8)?;
                            self.position = Vector3::new(x, y, z);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
