use crate::memory::MemoryManager;
use crate::state::StateContext;
use memory::memory_manager::unity::*;
use memory::process::{Error, Process};
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
        if self.manager.instance.is_some() {
            self.data.update(ctx, &mut self.manager)
        }
    }
}

#[derive(Default, Debug)]
pub struct PlayerPartyManagerData {
    pub position: Vector3<f32>,
}

impl PlayerPartyManagerData {
    pub fn update(&mut self, ctx: &StateContext, manager: &mut UnityMemoryManager) {
        self.update_position(ctx, manager);
    }

    pub fn update_position(&mut self, ctx: &StateContext, manager: &mut UnityMemoryManager) {
        if let Some(class) = manager.class {
            if let Some(process) = &ctx.process {
                if let Some(module) = &ctx.module {
                    if let Some(singleton) = manager.singleton {
                        if let Ok(current_position_ptr) = class.follow_fields_without_read(
                            singleton,
                            process,
                            module,
                            &["leader", "controller", "currentTargetPosition"],
                        ) {
                            let x = process
                                .read_pointer::<f32>(current_position_ptr)
                                .ok()
                                .unwrap();
                            let y = process
                                .read_pointer::<f32>(current_position_ptr + 0x4)
                                .ok()
                                .unwrap();
                            let z = process
                                .read_pointer::<f32>(current_position_ptr + 0x8)
                                .ok()
                                .unwrap();

                            self.position = Vector3::new(x, y, z);
                        }
                    }
                }
            }
        }
    }
}
