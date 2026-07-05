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
    pub position: Vector3<f32>,
    pub rotation: Quaternion,
    pub speed: f32,
    pub max_speed: f32,
}

impl Default for MemoryManager<BoatManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "BoatManager",
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

        // Each read is guarded and no-ops until the boat instance exists, so we
        // no longer need a separate backing-field existence gate.
        self.update_position(&memory_context)?;
        self.update_rotation(&memory_context)?;
        self.update_speed(&memory_context)?;

        Ok(())
    }
}

impl BoatManagerData {
    pub fn update_position(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok([x, y, z]) = memory_context.read::<[f32; 3]>(&[
            "<BoatInstance>k__BackingField",
            "boatController",
            "currentTargetPosition",
        ]) {
            self.position = Vector3::new(x, y, z);
        }

        Ok(())
    }

    pub fn update_rotation(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok([x, y, z, w]) = memory_context.read::<[f32; 4]>(&[
            "<BoatInstance>k__BackingField",
            "boatSnapRotation",
            "pitchRollLocalRotation",
        ]) {
            self.rotation = Quaternion { x, y, z, w };
        }

        Ok(())
    }

    pub fn update_speed(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(max_speed) =
            memory_context.read::<f32>(&["<BoatInstance>k__BackingField", "boatSpeed"])
        {
            self.max_speed = max_speed;
        }
        if let Ok(speed) =
            memory_context.read::<f32>(&["<BoatInstance>k__BackingField", "previousSpeed"])
        {
            self.speed = speed;
        }
        Ok(())
    }
}
