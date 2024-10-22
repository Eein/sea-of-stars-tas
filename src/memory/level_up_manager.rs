use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use data::prelude::PlayerPartyCharacter;
use log::info;
use memory::game_engine::il2cpp::unity_list::{UnityItem, UnityList};
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;
use memory::process::Process;
use memory::string::ArrayWString;

#[derive(Default, Debug)]
pub struct LevelUpManagerData {
    pub active: bool,
    pub current_character: PlayerPartyCharacter,
    pub current_upgrades: UnityList<LevelUpUpgradeButton>,
    pub upgrade_index: u32,
}

impl Default for MemoryManager<LevelUpManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "LevelUpSceneController".to_string(),
            data: LevelUpManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Default, Debug)]
pub struct LevelUpUpgradeButton {
    pub upgrade: LevelUpUpgrade,
    pub selected: bool,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum LevelUpUpgrade {
    #[default]
    None = -1,
    HitPoint = 0,
    SkillPoint = 1,
    PhysicalAttack = 3,
    PhysicalDefense = 4,
    MagicalAttack = 5,
    MagicalDefense = 6,
}

impl LevelUpUpgrade {
    fn from_u8(value: u8) -> LevelUpUpgrade {
        match value {
            0 => LevelUpUpgrade::HitPoint,
            1 => LevelUpUpgrade::SkillPoint,
            3 => LevelUpUpgrade::PhysicalAttack,
            4 => LevelUpUpgrade::PhysicalDefense,
            5 => LevelUpUpgrade::MagicalAttack,
            6 => LevelUpUpgrade::MagicalDefense,
            _ => LevelUpUpgrade::None,
        }
    }
}

impl MemoryManagerUpdate for LevelUpManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_level_up_screen_active(&memory_context)?;
        if self.active {
            self.update_current_character(&memory_context)?;
            self.update_current_upgrades(&memory_context)?;
        }

        Ok(())
    }
}

impl LevelUpManagerData {
    // This address checks the onLevelUpDone callback pointer
    // If this is 0x0, it is not loaded, if it is not 0x0 then its pointing to
    // a function the game expects to call when the level up screen is over.
    pub fn update_level_up_screen_active(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(active) = memory_context.follow_fields::<u64>(&["onLevelUpDone"]) {
            self.active = active != 0x0;
        }
        Ok(())
    }

    pub fn update_current_character(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(current_character_ptr) = memory_context.follow_fields::<u64>(&[
            "currentCharacter",
            "stateMachine",
            "currentState",
            "player",
            "characterDefinitionId",
        ]) {
            let current_character_str =
                memory_context.read_pointer::<ArrayWString<16>>(current_character_ptr + 0x14)?;

            let current_character = match String::from_utf16(current_character_str.as_slice()) {
                Ok(value) => value,
                Err(_) => {
                    return Ok(());
                }
            };
            self.current_character = PlayerPartyCharacter::parse(&current_character)
        }
        Ok(())
    }

    pub fn update_current_upgrades(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(upgrades) = memory_context.follow_fields::<u64>(&["currentLevelUpUpgrades"]) {
            let current_upgrades =
                UnityList::<LevelUpUpgradeButton>::read(memory_context.process, upgrades)?;
            self.current_upgrades = current_upgrades;
            // convert to enumerator and take first with selected true and return its index
            let active = self
                .current_upgrades
                .items
                .iter()
                .enumerate()
                .find(|(_i, u)| u.selected)
                .map(|(i, _u)| i)
                .take();
            if let Some(active_index) = active {
                self.upgrade_index = active_index as u32;
            }
        }
        Ok(())
    }
}

impl UnityItem for LevelUpUpgradeButton {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        let upgrade = process.read_pointer::<u8>(item_ptr + 0x38)?;
        let selected = process.read_pointer_path::<u8>(item_ptr, &[0x50, 0xA0])?;

        Ok(LevelUpUpgradeButton {
            upgrade: LevelUpUpgrade::from_u8(upgrade),
            selected: matches!(selected, 0),
        })
    }
}
