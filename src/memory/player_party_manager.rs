use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use data::prelude::*;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;
use memory::string::ArrayWString;
use vec3_rs::Vector3;

#[derive(Default, Debug)]
pub struct PlayerPartyManagerData {
    pub position: Vector3<f32>,
    pub gameobject_position: Vector3<f32>,
    pub movement_state: PlayerMovementState,
    pub leader_character: PlayerPartyCharacter,
}

impl Default for MemoryManager<PlayerPartyManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "PlayerPartyManager",
            data: PlayerPartyManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

impl MemoryManagerUpdate for PlayerPartyManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        // `leader` is a real field name; the trailing entries are raw offsets
        // inside the leader object. `UnityPointer` resolves mixed name/offset
        // paths, and each read no-ops until the party leader exists, so no
        // separate offset-resolution gate is needed.
        self.update_position(&memory_context)?;
        self.update_gameobject_position(&memory_context)?;
        self.update_movement_state(&memory_context)?;
        self.update_leader_character(&memory_context)?;

        Ok(())
    }
}

impl PlayerPartyManagerData {
    pub fn update_position(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok([x, y, z]) = memory_context.read::<[f32; 3]>(&["leader", "0x90", "0x84"]) {
            self.position = Vector3::new(x, y, z);
        };

        Ok(())
    }

    pub fn update_gameobject_position(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok([x, y, z]) =
            memory_context.read::<[f32; 3]>(&["leader", "0x30", "0x48", "0x1C"])
        {
            self.gameobject_position = Vector3::new(x, y, z);
        };

        Ok(())
    }

    pub fn update_movement_state(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(movement_state) =
            memory_context.read::<u8>(&["leader", "0x88", "0x50", "0x8C"])
        {
            self.movement_state = match movement_state {
                0 => PlayerMovementState::None,
                1 => PlayerMovementState::Running,
                2 => PlayerMovementState::Walking,
                3 => PlayerMovementState::Idle,
                _ => PlayerMovementState::None,
            }
        };

        Ok(())
    }

    pub fn update_leader_character(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(character) = memory_context.read::<ArrayWString<128>>(&["leaderID", "0x14"])
            && let Ok(name) = String::from_utf16(character.as_slice())
        {
            self.leader_character = PlayerPartyCharacter::parse(&name)
        }

        Ok(())
    }
}

// PlayerDefaultState.EState
// #
#[derive(Default, Debug, PartialEq, Eq)]
pub enum PlayerMovementState {
    #[default]
    None = 0,
    Running = 1,
    Walking = 2,
    Idle = 3,
}
