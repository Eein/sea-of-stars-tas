use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;

impl Default for MemoryManager<CutsceneManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "CutsceneManager".to_string(),
            data: CutsceneManagerData::default(),
            manager: UnityMemoryManager {
                ..UnityMemoryManager::default()
            },
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Default, Debug)]
pub struct CutsceneManagerData {
    // Toggles during every loss of control event;
    // TODO(eein): determine if these events are skippable.
    pub is_in_cutscene: bool,

    // This value breifly sets to true after skipping cutscene bar fills
    // During this time, it runs a few functions to decide where to go after the
    // skip is finished.
    pub is_skipping_cutscene: bool,

    // This toggles from 0 to 1 when the player is locked from movement and doesn't
    // always indicate that the cutscene can be skipped or not (for example child
    // cutscene).
    pub skip_cutscene_locked: bool,

    // This is a pointer to the SkipCutsceneTransitionScreen object
    // This briefly flashes after a cutscene is over and is generally here
    // for debugging purposes only.
    pub skip_cutscene_transition_screen: u64,
}

impl MemoryManagerUpdate for CutsceneManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_is_in_cutscene(&memory_context)?;
        self.update_is_skipping_cutscene(&memory_context)?;
        self.update_skip_cutscene_locked(&memory_context)?;
        self.update_skip_cutscene_transition_screen(&memory_context)?;

        Ok(())
    }
}

impl CutsceneManagerData {
    pub fn update_is_in_cutscene(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(is_in_cutscene) =
            memory_context.follow_fields::<u8>(&["<IsInCutscene>k__BackingField"])
        {
            match is_in_cutscene {
                1 => self.is_in_cutscene = true,
                0 => self.is_in_cutscene = false,
                _ => self.is_in_cutscene = false,
            }
        }

        Ok(())
    }

    pub fn update_is_skipping_cutscene(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(is_skipping_cutscene) =
            memory_context.follow_fields::<u8>(&["isSkippingCutscene"])
        {
            match is_skipping_cutscene {
                1 => self.is_skipping_cutscene = true,
                0 => self.is_skipping_cutscene = false,
                _ => self.is_skipping_cutscene = false,
            }
        }

        Ok(())
    }

    pub fn update_skip_cutscene_locked(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(skip_cutscene_locked) =
            memory_context.follow_fields::<u8>(&["skipCutsceneLocked"])
        {
            match skip_cutscene_locked {
                1 => self.skip_cutscene_locked = true,
                0 => self.skip_cutscene_locked = false,
                _ => self.skip_cutscene_locked = false,
            }
        }

        Ok(())
    }

    pub fn update_skip_cutscene_transition_screen(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(skip_cutscene_transition_screen) =
            memory_context.follow_fields::<u64>(&["skipCutsceneTransitionScreen"])
        {
            self.skip_cutscene_transition_screen = skip_cutscene_transition_screen;
        }

        Ok(())
    }
}
