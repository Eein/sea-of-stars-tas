use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;

impl Default for MemoryManager<NewDialogManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "NewDialogManager".to_string(),
            data: NewDialogManagerData::default(),
            manager: UnityMemoryManager {
                ..UnityMemoryManager::default()
            },
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Default, Debug)]
pub struct NewDialogManagerData {
    pub dialog_visible: bool,
}

impl MemoryManagerUpdate for NewDialogManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_dialog_visible(&memory_context)?;

        Ok(())
    }
}

impl NewDialogManagerData {
    pub fn update_dialog_visible(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(dialog_box_pointer) = memory_context.follow_fields::<u64>(&["onDialogCompleted"])
        {
            self.dialog_visible = dialog_box_pointer != 0x0;
        }

        Ok(())
    }
}
