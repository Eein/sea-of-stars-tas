use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;
use memory::string::ArrayWString;

impl Default for MemoryManager<LevelManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "LevelManager".to_string(),
            data: LevelManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Default, Debug)]
pub struct LevelManagerData {
    pub loading: bool,
    pub scene_guid: String,
    pub scene_name: String,
}

impl MemoryManagerUpdate for LevelManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_loading(&memory_context)?;
        self.update_scene_name(&memory_context)?;
        self.update_scene_guid(&memory_context)?;

        Ok(())
    }
}

impl LevelManagerData {
    pub fn update_loading(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(loading) = memory_context.follow_fields::<u8>(&["loadingLevel"]) {
            match loading {
                0 => self.loading = false,
                1 => self.loading = true,
                _ => self.loading = false,
            }
        }
        Ok(())
    }

    pub fn update_scene_name(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(addr) = memory_context.follow_fields::<u64>(&["levelLoader", "mainSceneName"]) {
            let name_str = memory_context.read_pointer::<ArrayWString<128>>(addr + 0x14)?;
            match String::from_utf16(name_str.as_slice()) {
                Ok(value) => {
                    self.scene_name = value.clone();
                }
                Err(_) => {
                    self.scene_name = "None".to_string();
                }
            };
        }
        Ok(())
    }

    pub fn update_scene_guid(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(addr) = memory_context.follow_fields::<u64>(&["currentLevel"]) {
            let name_str = memory_context.read_pointer::<ArrayWString<128>>(addr + 0x14)?;
            match String::from_utf16(name_str.as_slice()) {
                Ok(value) => {
                    self.scene_guid = value.clone();
                }
                Err(_) => {
                    self.scene_guid = "None".to_string();
                }
            };
        }
        Ok(())
    }
}
