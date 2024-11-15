use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use data::prelude::PlayerPartyCharacter;
use log::info;
use memory::game_engine::il2cpp::unity_list::*;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;
use memory::process::Process;
use memory::string::*;
use vec3_rs::Vector3;

#[derive(Default, Debug)]
pub struct Player {
    pub name: String,
    pub state: String,
    pub first_player: bool,
    pub position: Vector3<f32>,
    pub gameobject_position: Vector3<f32>,
    pub index: i32,
    pub can_join_leave: bool,
    pub playing: bool,
    pub character: PlayerPartyCharacter,
    pub bubble_fill_amount: f32, // 0.0-1.0
    pub has_boost: bool,
    pub is_doing_high_five: bool,
    pub can_replenish_boost: bool,
    pub in_bubble: bool,
}

#[derive(Default, Debug)]
pub struct SinglePlayerPlusManagerData {
    pub players: UnityList<Player>,
}

impl Default for MemoryManager<SinglePlayerPlusManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "SinglePlayerPlusManager".to_string(),
            data: SinglePlayerPlusManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

impl MemoryManagerUpdate for SinglePlayerPlusManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_players(&memory_context)?;

        Ok(())
    }
}

impl SinglePlayerPlusManagerData {
    pub fn update_players(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(players) = memory_context.follow_fields::<u64>(&["allPlayers"]) {
            let players = UnityList::<Player>::read(memory_context.process, players)?;
            self.players = players;
        }
        Ok(())
    }
}

impl UnityItem for Player {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        let name = if let Ok(name_ptr) =
            process.read_pointer_path::<ArrayWString<128>>(item_ptr, &[0xA0, 0x14])
        {
            if let Ok(name_str) = String::from_utf16(name_ptr.as_slice()) {
                name_str
            } else {
                "Unassigned".to_string()
            }
        } else {
            "Unassigned".to_string()
        };

        let first_player = if let Ok(fp) = process.read_pointer_path::<u8>(item_ptr, &[0x9C]) {
            matches!(fp, 1)
        } else {
            false
        };
        let position = if let Ok(position_ptr) =
            process.read_pointer_path_without_read(item_ptr, &[0x88, 0x98, 0x84])
        {
            let x = process.read_pointer::<f32>(position_ptr)?;
            let y = process.read_pointer::<f32>(position_ptr + 0x4)?;
            let z = process.read_pointer::<f32>(position_ptr + 0x8)?;
            Vector3::new(x, y, z)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        };

        let gameobject_position = if let Ok(gameobject_ptr) =
            process.read_pointer_path_without_read(item_ptr, &[0x88, 0x30, 0x48, 0x1C])
        {
            let x = process.read_pointer::<f32>(gameobject_ptr)?;
            let y = process.read_pointer::<f32>(gameobject_ptr + 0x4)?;
            let z = process.read_pointer::<f32>(gameobject_ptr + 0x8)?;
            Vector3::new(x, y, z)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        };
        let index = process.read_pointer_path::<i32>(item_ptr, &[0x80])?;

        let can_join_leave =
            if let Ok(join_leave) = process.read_pointer_path::<u8>(item_ptr, &[0x9D]) {
                matches!(join_leave, 1)
            } else {
                false
            };

        let playing = if let Ok(playing_bool) = process.read_pointer_path::<u8>(item_ptr, &[0x70]) {
            matches!(playing_bool, 1)
        } else {
            false
        };

        let character = if let Ok(char) =
            process.read_pointer_path::<ArrayWString<128>>(item_ptr, &[0x88, 0x40, 0x14])
        {
            if let Ok(name) = String::from_utf16(char.as_slice()) {
                PlayerPartyCharacter::parse(&name)
            } else {
                PlayerPartyCharacter::None
            }
        } else {
            PlayerPartyCharacter::None
        };

        let has_boost = if let Ok(boost_bool) =
            process.read_pointer_path::<u8>(item_ptr, &[0x88, 0x128, 0xF0])
        {
            matches!(boost_bool, 1)
        } else {
            false
        };

        let is_doing_high_five = if let Ok(high_five_bool) =
            process.read_pointer_path::<u8>(item_ptr, &[0x88, 0x128, 0xC1])
        {
            matches!(high_five_bool, 1)
        } else {
            false
        };

        let can_replenish_boost = if let Ok(replenish_bool) =
            process.read_pointer_path::<u8>(item_ptr, &[0x88, 0x128, 0xF1])
        {
            matches!(replenish_bool, 1)
        } else {
            false
        };

        let bubble_fill_amount =
            if let Ok(bubble_handler_ptr) = process.read_pointer_path::<u64>(item_ptr, &[0x58]) {
                // safely exit if the handler is missing
                if bubble_handler_ptr == 0x0 {
                    0.0
                } else {
                    process
                        .read_pointer_path::<f32>(bubble_handler_ptr, &[0x30, 0x18, 0xF4])
                        .unwrap_or(0.0_f32)
                }
            } else {
                0.0
            };

        let state_str: String =
            match process.read_pointer_path::<u64>(item_ptr, &[0x88, 0x90, 0x58, 0x0, 0x10]) {
                Ok(binding) => match process.read_pointer::<ArrayCString<200>>(binding) {
                    Ok(c_str) => match c_str.validate_utf8() {
                        Ok(str) => str.to_string(),
                        Err(_) => "Unknown State".to_string(),
                    },
                    _ => "Unknown State".to_string(),
                },
                _ => "Uknown State".to_string(),
            };

        let in_bubble = state_str == "SinglePlayerPlusBubbleState";
        let state = state_str;

        Ok(Player {
            name,
            state,
            first_player,
            position,
            gameobject_position,
            index,
            can_join_leave,
            playing,
            character,
            bubble_fill_amount,
            in_bubble,
            has_boost,
            is_doing_high_five,
            can_replenish_boost,
        })
    }
}
