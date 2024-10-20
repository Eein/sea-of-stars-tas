use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use crate::util::quaternion::Quaternion;
use vec3_rs::Vector3;

use log::info;

use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;

#[derive(Default, Debug)]
pub struct BoatManagerData {
    backing_field: Option<u32>,
    pub position: Vector3<f32>,
    pub rotation: Quaternion,
    pub speed: f32,
    pub max_speed: f32,
}

impl Default for MemoryManager<BoatManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "BoatManager".to_string(),
            data: BoatManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

impl MemoryManagerUpdate for BoatManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_backing_field(&memory_context)?;

        if self.backing_field.is_some() {
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
        self.backing_field = memory_context.get_field_offset("<BoatInstance>k__BackingField");

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
            let x = memory_context.read_pointer::<f32>(boat_rotation_ptr)?;
            let y = memory_context.read_pointer::<f32>(boat_rotation_ptr + 0x4)?;
            let z = memory_context.read_pointer::<f32>(boat_rotation_ptr + 0x8)?;
            let w = memory_context.read_pointer::<f32>(boat_rotation_ptr + 0xC)?;

            self.rotation = Quaternion { x, y, z, w };
        }

        Ok(())
    }

    pub fn update_speed(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(max_speed) =
            memory_context.follow_fields::<f32>(&["<BoatInstance>k__BackingField", "boatSpeed"])
        {
            self.max_speed = max_speed;
        }
        if let Ok(speed) =
            memory_context.follow_fields::<f32>(&["<BoatInstance>k__BackingField", "previousSpeed"])
        {
            self.speed = speed;
        }
        Ok(())
    }
}
