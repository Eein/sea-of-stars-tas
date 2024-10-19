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
pub struct LiveMana {
    pub big: u32,
    pub small: u32,
}

#[derive(Default, Debug)]
pub struct CombatManagerData {
    pub encounter_active: bool,
    pub combat_controller_type: CombatControllerType,
    pub live_mana: LiveMana,
    pub combo_points: u32,
    pub combo_point_progress: u32,
    pub ultimate_progress: f32,
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
            self.update_live_mana(&memory_context)?;
            self.update_combo_points_and_ultimates(&memory_context)?;
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
            if let Ok(controller_type_c_str) = memory_context
                .read_pointer_path::<ArrayCString<200>>(&[controller.into(), 0x0, 0x10, 0x0])
            {
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

    pub fn update_live_mana(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(small_live_mana_ptr) = memory_context.follow_fields::<u64>(&[
            "currentEncounter",
            "liveManaHandler",
            "smallLiveManaParticles",
        ]) {
            if let Ok(small_mana) =
                memory_context.read_pointer_path::<u32>(&[small_live_mana_ptr, 0x18])
            {
                self.live_mana.small = small_mana
            }
        } else {
            self.live_mana.small = 0;
        }
        if let Ok(big_live_mana_ptr) = memory_context.follow_fields::<u64>(&[
            "currentEncounter",
            "liveManaHandler",
            "bigLiveManaParticles",
        ]) {
            if let Ok(big_mana) =
                memory_context.read_pointer_path::<u32>(&[big_live_mana_ptr, 0x18])
            {
                self.live_mana.big = big_mana
            }
        } else {
            self.live_mana.big = 0;
        }

        Ok(())
    }

    pub fn update_combo_points_and_ultimates(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(combo_points_panel_ptr) = memory_context.follow_fields::<u64>(&[
            "currentEncounter",
            "controller",
            "battleUI",
            "comboPointsPanel",
        ]) {
            // comboPointsPanel -> ultMeter -> targetFill
            if let Ok(combo_point_progress) = memory_context
                .process
                .read_pointer_path::<u32>(combo_points_panel_ptr, &[0x30, 0x58])
            {
                self.combo_point_progress = combo_point_progress
            } else {
                self.combo_point_progress = 0
            }
            if let Ok(combo_points) = memory_context
                .process
                .read_pointer_path::<u32>(combo_points_panel_ptr, &[0x30, 0x5C])
            {
                self.combo_points = combo_points
            } else {
                self.combo_points = 0
            }
            // comboPointsPanel -> comboPointsMeter -> currentComboPoints
            if let Ok(ultimate_progress) = memory_context
                .process
                .read_pointer_path::<f32>(combo_points_panel_ptr, &[0x28, 0x40])
            {
                self.ultimate_progress = ultimate_progress
            } else {
                self.ultimate_progress = 0.0
            }
        }

        Ok(())
    }
}
