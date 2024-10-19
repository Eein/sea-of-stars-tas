use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::game_engine::il2cpp::unity_list::{UnityItem, UnityList};
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;
use memory::process::Process;
use memory::string::*;

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

#[derive(Debug, Default, Clone)]
pub enum CombatDamageType {
    #[default]
    None = 0,
    Any = 1,
    Sword = 2,
    Sun = 4,
    Moon = 8,
    Eclipse = 16,
    Poison = 32,
    Arcane = 64,
    Stun = 128,
    Blunt = 256,
    Magical = 252,
}

impl CombatDamageType {
    fn from_u32(value: u32) -> CombatDamageType {
        match value {
            0 => CombatDamageType::None,
            1 => CombatDamageType::Any,
            2 => CombatDamageType::Sword,
            4 => CombatDamageType::Sun,
            8 => CombatDamageType::Moon,
            16 => CombatDamageType::Eclipse,
            32 => CombatDamageType::Poison,
            64 => CombatDamageType::Arcane,
            128 => CombatDamageType::Stun,
            256 => CombatDamageType::Blunt,
            252 => CombatDamageType::Magical,
            _ => CombatDamageType::None,
        }
    }
}

#[derive(Default, Debug)]
pub struct LiveMana {
    pub big: u32,
    pub small: u32,
}

#[derive(Debug, Clone, Default)]
pub struct CombatEnemy {
    pub current_hp: u32,
    pub unique_id: String,
    pub guid: String,
    pub max_hp: u32,
    pub speed: u32,
    pub physical_attack: u32,
    pub physical_defense: u32,
    pub magical_attack: u32,
    pub magical_defense: u32,
    pub turns_to_action: u8,
    pub total_spell_locks: u8,
    pub spell_locks: UnityList<CombatDamageType>,
}

#[derive(Default, Debug)]
pub struct CombatManagerData {
    pub encounter_active: bool,
    pub combat_controller_type: CombatControllerType,
    pub live_mana: LiveMana,
    pub combo_points: u32,
    pub combo_point_progress: u32,
    pub ultimate_progress: f32,
    pub enemies: UnityList<CombatEnemy>,
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
            self.update_enemies(&memory_context)?;
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

    pub fn update_enemies(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(enemies) =
            memory_context.follow_fields::<u64>(&["currentEncounter", "enemyTargets"])
        {
            let enemies = UnityList::<CombatEnemy>::read(memory_context.process, enemies)?;
            self.enemies = enemies;
        }
        Ok(())
    }
}

impl UnityItem for CombatEnemy {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        // Top level pointers
        let enemy_data = process.read_pointer_path::<u64>(item_ptr, &[0x80, 0x108])?;
        let casting_data = process.read_pointer_path::<u64>(item_ptr, &[0x80, 0x120])?;
        let current_hp = process.read_pointer_path::<u32>(item_ptr, &[0x94])?;
        let guid_w_str =
            process.read_pointer_path::<ArrayWString<128>>(enemy_data, &[0x18, 0x14])?;
        let guid = String::from_utf16(guid_w_str.as_slice()).unwrap_or("Unknown".to_string());

        // enemy data
        let unique_id_w_str = process
            .read_pointer_path::<ArrayWString<36>>(item_ptr, &[0x80, 0xF8, 0xF0, 0x18, 0x14])?;
        let unique_id =
            String::from_utf16(unique_id_w_str.as_slice()).unwrap_or("Unknown".to_string());
        let max_hp = process.read_pointer::<u32>(enemy_data + 0x20)?;
        let speed = process.read_pointer::<u32>(enemy_data + 0x24)?;
        let physical_attack = process.read_pointer::<u32>(enemy_data + 0x2C)?;
        let physical_defense = process.read_pointer::<u32>(enemy_data + 0x28)?;
        let magical_attack = process.read_pointer::<u32>(enemy_data + 0x30)?;
        let magical_defense = process.read_pointer::<u32>(enemy_data + 0x34)?;

        // casting data
        let turns_to_action = process.read_pointer::<u8>(casting_data + 0x24)?;
        let total_spell_locks = process.read_pointer::<u8>(casting_data + 0x28)?;
        let spell_locks = if let Ok(locks) = process.read_pointer_path::<u64>(casting_data, &[0x18])
        {
            UnityList::<CombatDamageType>::read(process, locks)?
        } else {
            UnityList::<CombatDamageType>::default()
        };

        Ok(CombatEnemy {
            guid,
            unique_id,
            current_hp,
            max_hp,
            speed,
            physical_attack,
            physical_defense,
            magical_attack,
            magical_defense,
            turns_to_action,
            total_spell_locks,
            spell_locks,
        })
    }
}

impl UnityItem for CombatDamageType {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        let lock = process.read_pointer::<u32>(item_ptr + 0x40)?;
        Ok(CombatDamageType::from_u32(lock))
    }
}
