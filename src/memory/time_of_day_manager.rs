use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;

///  We are reaching into the AudioManager for Time of Day instead of using
///  Sabotage.TODManager simply as a way to avoid having to load the other module
///  into memory. The currentTimeOfDay field is provided on the AudioManager as well.
///  `current_time` is a `float` from 0.0 to 23.99~.
impl Default for MemoryManager<TimeOfDayManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "AudioManager".to_string(),
            data: TimeOfDayManagerData::default(),
            manager: UnityMemoryManager {
                ..UnityMemoryManager::default()
            },
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Default, Debug)]
pub struct TimeOfDayManagerData {
    pub current_time: f32,
}

impl MemoryManagerUpdate for TimeOfDayManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_current_time(&memory_context)?;

        Ok(())
    }
}

impl TimeOfDayManagerData {
    pub fn update_current_time(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(current_time) = memory_context.follow_fields::<f32>(&["currentTimeOfDay"]) {
            self.current_time = current_time;
        }

        Ok(())
    }
}
