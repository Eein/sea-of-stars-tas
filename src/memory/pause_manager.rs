//! `PauseManager` — the game's pause singleton. Read so the TAS holds its
//! sequences while the game is paused (e.g. a co-op player pressing Start
//! mid-route) instead of blindly driving inputs into a frozen game.

use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;

#[derive(Default, Debug)]
pub struct PauseManagerData {
    /// The game's `isPaused` flag (pause menu up / a pause request active).
    pub is_paused: bool,
}

impl Default for MemoryManager<PauseManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "PauseManager".to_string(),
            data: PauseManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

impl MemoryManagerUpdate for PauseManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;
        self.is_paused = memory_context
            .follow_fields::<u8>(&["isPaused"])
            .map(|b| b == 1)
            .unwrap_or(false);
        Ok(())
    }
}
