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
    pub gameobject_position: Vector3<f32>,
    pub leader_offset: Option<u32>,
    pub movement_state: PlayerMovementState,
}

impl MemoryManagerUpdate for PlayerPartyManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_leader_offset(&memory_context)?;

        if self.leader_offset.is_some() {
            self.update_position(&memory_context)?;
            self.update_gameobject_position(&memory_context)?;
            self.update_movement_state(&memory_context)?;
        }

        Ok(())
    }
}

impl PlayerPartyManagerData {
    // Updates the dangling `leader` offset for the controller
    // TODO(eein): is this actually true? It might have been one off issue.
    // This value does not have a true field name so can't be queried with
    // follow fields.
    //
    // NOTE(eein): I believe the internal memory function captures any
    // errors but thats also not clear. Maybe make it a little more aggressive
    // and error
    pub fn update_leader_offset(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        self.leader_offset = memory_context.get_field_offset("leader");
        Ok(())
    }

    pub fn update_position(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        let leader_offset = self.leader_offset.unwrap();

        let current_position_ptr =
            memory_context.read_pointer_path_without_read(&[leader_offset.into(), 0x90, 0x84])?;

        let x = memory_context.read_pointer::<f32>(current_position_ptr)?;
        let y = memory_context.read_pointer::<f32>(current_position_ptr + 0x4)?;
        let z = memory_context.read_pointer::<f32>(current_position_ptr + 0x8)?;

        self.position = Vector3::new(x, y, z);

        Ok(())
    }

    pub fn update_gameobject_position(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        let leader_offset = self.leader_offset.unwrap();

        let gameobject_ptr = memory_context.read_pointer_path_without_read(&[
            leader_offset.into(),
            0x30,
            0x48,
            0x1C,
        ])?;

        let x = memory_context.read_pointer::<f32>(gameobject_ptr)?;
        let y = memory_context.read_pointer::<f32>(gameobject_ptr + 0x4)?;
        let z = memory_context.read_pointer::<f32>(gameobject_ptr + 0x8)?;

        self.gameobject_position = Vector3::new(x, y, z);

        Ok(())
    }

    pub fn update_movement_state(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        let leader_offset = self.leader_offset.unwrap();

        if let Ok(movement_state) =
            memory_context.read_pointer_path::<u8>(&[leader_offset.into(), 0x88, 0x50, 0x8C])
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
}

// PlayerDefaultState.EState
// #
#[derive(Default, Debug)]
pub enum PlayerMovementState {
    #[default]
    None = 0,
    Running = 1,
    Walking = 2,
    Idle = 3,
}
