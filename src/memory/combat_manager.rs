use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;
use memory::string::ArrayCString;

#[derive(Default, Debug)]
pub enum CombatControllerType {
    #[default]
    Basic,
    FirstEncounter,
    SecondEncounter,
    DwellerOfStrife,
    DwellerOfDread,
    KOTutorial,
    LiveManaTutorial,
    ManaRegenTutorial,
    RoundsTutorial,
    SpellLockTutorial,
    TimedBlocksTutorial,
    TimedHitsTutorial,
}

#[derive(Default, Debug)]
pub struct CombatManagerData {
    pub encounter_active: bool,
    pub combat_controller_type: CombatControllerType
}

impl Default for MemoryManager<CombatManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "CombatManager".to_string(),
            data: CombatManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

impl MemoryManagerUpdate for CombatManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_encounter_active(&memory_context)?;

        // Check if the encounter is active, then run the rest
        // of the updates.
        if self.encounter_active {
            self.update_combat_controller_type(&memory_context)?;

        }

        Ok(())
    }
}

impl CombatManagerData {
    pub fn update_encounter_active(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(encounter_done) =
            memory_context.follow_fields::<u8>(&["currentEncounter", "encounterDone"])
        {
            self.encounter_active = matches!(encounter_done, 0)
        } else {
            self.encounter_active = false;
        }

        Ok(())
    }

    pub fn update_combat_controller_type(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {

                    // [self.current_encounter_base, 0x128, 0x0, 0x10, 0x0],
        if let Ok(controller) =
            memory_context.follow_fields::<u8>(&["currentEncounter", "controller"])
        {
            // This code reaches into the base types of the controller thats active to find the
            // name
            if let Ok(controller_type_c_str) = memory_context.read_pointer_path::<ArrayCString<200>>(&[controller.into(), 0x0, 0x10, 0x0]) {
                if let Ok(controller_type) = controller_type_c_str.validate_utf8() {
                    self.combat_controller_type = match controller_type {
                        "EncounterController" => CombatControllerType::Basic,
                        "FirstEncounter" => CombatControllerType::FirstEncounter,
                        "SecondEncounter" => CombatControllerType::SecondEncounter,
                        "DwellerOfStrife" => CombatControllerType::DwellerOfStrife,
                        "DwellerOfDread" => CombatControllerType::DwellerOfDread,
                        "KOTutorial" => CombatControllerType::KOTutorial,
                        "LiveManaTutorial" => CombatControllerType::LiveManaTutorial,
                        "ManaRegenTutorial" => CombatControllerType::ManaRegenTutorial,
                        "RoundsTutorial" => CombatControllerType::RoundsTutorial,
                        "SpellLockTutorial" => CombatControllerType::SpellLockTutorial,
                        "TimedBlocksTutorial" => CombatControllerType::TimedBlocksTutorial,
                        "TimedHitsTutorial" => CombatControllerType::TimedHitsTutorial,
                        _ => CombatControllerType::Basic,
                    }
                }
            }

        }

        Ok(())
    }
}
