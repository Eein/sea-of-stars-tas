use crate::memory::MemoryManager;
use crate::state::StateContext;
use memory::memory_manager::unity::*;
use memory::process::Error;
use vec3_rs::Vector3;

pub struct PlayerPartyManager {
    pub name: String,
    pub manager: UnityMemoryManager,
    pub data: PlayerPartyManagerData,
}

impl Default for PlayerPartyManager {
    fn default() -> Self {
        let manager = Self {
            name: "PlayerPartyManager".to_string(),
            data: PlayerPartyManagerData::default(),
            manager: UnityMemoryManager {
                ..UnityMemoryManager::default()
            },
        };
        println!("{} Loaded", manager.name);
        manager
    }
}

impl MemoryManager for PlayerPartyManager {
    fn ready_for_updates(&mut self, _ctx: &StateContext) -> bool {
        if let Some(class) = self.manager.singleton {
            if class.class == 0 {
                return false;
            } 
            return true;
        }
        true
    }

    fn update_manager(&mut self, ctx: &StateContext) {
        if let Some(process) = &ctx.process {
            if let Some(module) = &ctx.module {
                if let Some(image) = &ctx.image {
                    self.manager.update(process, module, image, &self.name);
                }
            }
        }
    }

    fn update_memory(&mut self, ctx: &StateContext) {
        if self.ready_for_updates(ctx) {
            match self.data.update(ctx, &mut self.manager) {
                Ok(_) => (),
                Err(_error) => self.manager.reset()
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct PlayerPartyManagerData {
    pub position: Vector3<f32>,
}

impl PlayerPartyManagerData {
    pub fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), Error> {
        match self.update_position(ctx, manager) {
            Ok(_) => Ok(()),
            Err(error) => Err(error)
        }
    }

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
                            let current_position_ptr = process.read_pointer_path_without_read(singleton.class, &[leader.into(), 0x90, 0x84])?;
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
