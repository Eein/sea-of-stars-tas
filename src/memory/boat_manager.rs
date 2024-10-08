use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use vec3_rs::Vector3;
use crate::util::quaternion::Quaternion;

use log::info;

use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;

impl Default for MemoryManager<BoatManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "BoatManager".to_string(),
            data: BoatManagerData::default(),
            manager: UnityMemoryManager {
                ..UnityMemoryManager::default()
            },
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Default, Debug)]
pub struct BoatManagerData {
    backing_field: u64,
    pub position: Vector3<f32>,
    pub rotation: Quaternion,
    pub speed: f32,
    pub max_speed: f32,
}

impl MemoryManagerUpdate for BoatManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;
        self.update_backing_field(&memory_context)?;
        if self.backing_field != 0 {
            self.update_position(&memory_context)?;
            self.update_rotation(&memory_context)?;
            self.update_speed(&memory_context)?;
        }

        Ok(())
    }
}

impl BoatManagerData {
    pub fn update_backing_field(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(backing_field_ptr) =
            memory_context.follow_fields_without_read(&["<BoatInstance>k__BackingField"])
        {
            self.backing_field = memory_context.read_pointer::<u64>(backing_field_ptr)?;
        }
        Ok(())
    }

    pub fn update_position(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(boat_position_ptr) = memory_context.follow_fields_without_read(&[
            "<BoatInstance>k__BackingField",
            "boatController",
            "currentTargetPosition",
        ]) {
            let x = memory_context.read_pointer::<f32>(boat_position_ptr)?;
            let y = memory_context.read_pointer::<f32>(boat_position_ptr + 0x4)?;
            let z = memory_context.read_pointer::<f32>(boat_position_ptr + 0x8)?;

            self.position = Vector3::new(x, y, z);
        }

        Ok(())
    }

    pub fn update_rotation(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(boat_rotation_ptr) = memory_context.follow_fields_without_read(&[
            "<BoatInstance>k__BackingField",
            "boatSnapRotation",
            "pitchRollLocalRotation",
        ]) {
            let x = memory_context.read_pointer::<f32>(boat_rotation_ptr + 0x44)?;
            let y = memory_context.read_pointer::<f32>(boat_rotation_ptr + 0x48)?;
            let z = memory_context.read_pointer::<f32>(boat_rotation_ptr + 0x4C)?;
            let w = memory_context.read_pointer::<f32>(boat_rotation_ptr + 0x50)?;

            self.rotation = Quaternion { x, y, z, w };
        }

        Ok(())
    }

    // TODO(eein): This update is missing field information
    // Update to use follow_fields when fields are known
    pub fn update_speed(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(speed_ptr) = memory_context.read_pointer_path_without_read(&[0x40]) {
            let max_speed = memory_context.read_pointer::<f32>(speed_ptr + 0x78)?;
            let speed = memory_context.read_pointer::<f32>(speed_ptr + 0x134)?;

            self.max_speed = max_speed;
            self.speed = speed;
        }
        Ok(())
    }
}
