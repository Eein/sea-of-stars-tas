use std::collections::btree_map::Values;
use std::collections::hash_map::Values;

use crate::memory::memory_context::MemoryContext;
use crate::memory::objects::character::PlayerPartyCharacter;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;

use log::info;

use memory::game_engine::il2cpp::{UnityPointer, unity_list::{UnityItem, UnityList}};
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;
use memory::process::Process;
use memory::string::{ArrayCString, ArrayWString};

impl Default for MemoryManager<TitleSequenceManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "TitleSequenceManager".to_string(),
            data: TitleSequenceManagerData::default(),
            manager: UnityMemoryManager {
                ..UnityMemoryManager::default()
            },
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Debug)]
pub struct TitleSequenceManagerData {
    /// Title Menu Option data
    pub new_game_selected: UnityPointer<3>,
    pub new_game_plus_selected: UnityPointer<3>,
    pub continue_selected: UnityPointer<3>,
    pub load_game_selected: UnityPointer<3>,
    pub options_selected: UnityPointer<3>,
    pub quit_game_selected: UnityPointer<3>,

    pub current_screen_name: UnityPointer<1>,
    pub relic_buttons: UnityPointer<2>,

    pub left_character: UnityPointer<3>,
    pub left_character_selected: UnityPointer<3>,
    pub right_character: UnityPointer<3>,
    pub right_character_selected: UnityPointer<3>,
    pub selected_character: UnityPointer<3>,
    // relicSelectionScreen -> relicButtons
    /// If saves are loaded and continue shows up on the title screen.
    pub load_save_done: UnityPointer<1>,
    /// If the player has pressed start on the intro screen.
    pub pressed_start: UnityPointer<2>,
}

impl Default for TitleSequenceManagerData {
    fn default() -> Self {
        Self {
            new_game_selected: UnityPointer::new("TitleSequenceManager", 0, &["titleScreen", "newGameButton", "selected"]),
            new_game_plus_selected: UnityPointer::new("TitleSequenceManager", 0, &["titleScreen", "newGamePlusButton", "selected"]),
            continue_selected: UnityPointer::new("TitleSequenceManager", 0, &["titleScreen", "continueButton", "selected"]),
            load_game_selected: UnityPointer::new("TitleSequenceManager", 0, &["titleScreen", "loadGameButton", "selected"]),
            options_selected: UnityPointer::new("TitleSequenceManager", 0, &["titleScreen", "optionsButton", "selected"]),
            quit_game_selected: UnityPointer::new("TitleSequenceManager", 0, &["titleScreen", "quitGameButton", "selected"]),
            current_screen_name: UnityPointer::new("TitleSequenceManager", 0, &["currentScreenName"]),
            relic_buttons: UnityPointer::new("TitleSequenceManager", 0, &["relicSelectionScreen", "relicButtons"]),

            left_character: UnityPointer::new("TitleSequenceManager", 0, &["characterSelectionScreen", "leftButton", "characterDefinitionId"]),
            left_character_selected: UnityPointer::new("TitleSequenceManager", 0, &["characterSelectionScreen", "leftButton", "selected"]),
            right_character: UnityPointer::new("TitleSequenceManager", 0, &["characterSelectionScreen", "rightButton", "characterDefinitionId"]),
            right_character_selected: UnityPointer::new("TitleSequenceManager", 0, &["characterSelectionScreen", "rightButton", "selected"]),
            selected_character: UnityPointer::new("TitleSequenceManager", 0, &["characterSelectionScreen", "selectedCharacter", "characterDefinitionId"]),

            load_save_done: UnityPointer::new("TitleSequenceManager", 0, &["loadSaveDone"]),
            pressed_start: UnityPointer::new("TitleSequenceManager", 0, &["titleScreen", "startPressed"])
        }

    }

}

impl MemoryManagerUpdate for TitleSequenceManagerData {
    fn update(
        &mut self,
        _ctx: &StateContext,
        _manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        Ok(())
    }
}

impl TitleSequenceManagerData {
    pub fn load_save_done(
        &mut self,
    ) -> kkk{
        self.load_save_done
    }

    pub fn update_pressed_start(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(pressed_start) =
            memory_context.follow_fields::<u8>(&["titleScreen", "startPressed"])
        {
            self.pressed_start = match pressed_start {
                1 => true,
                0 => false,
                _ => false,
            };
        }

        Ok(())
    }

    pub fn update_title_menu(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        Ok(())
    }

    pub fn update_relics(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(relic_buttons) =
            memory_context.follow_fields::<u64>(&["relicSelectionScreen", "relicButtons"])
        {
            let buttons = UnityList::<RelicButton>::read(memory_context.process, relic_buttons)?;
            self.relic_buttons = buttons;
        }

        Ok(())
    }

    pub fn update_current_screen_name(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(addr) = memory_context.follow_fields::<u64>(&["currentScreenName"]) {
            let name_str = memory_context.read_pointer::<ArrayWString<128>>(addr + 0x14)?;
            match String::from_utf16(name_str.as_slice()) {
                Ok(value) => {
                    self.current_screen_name = value.clone();
                }
                Err(_) => {
                    self.current_screen_name = "None".to_string();
                }
            };
        }

        Ok(())
    }

    pub fn update_new_game_characters(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        // Sanity check for escaping a missing dangling address
        match memory_context.follow_fields::<u64>(&["characterSelectionScreen", "leftButton"]) {
            Ok(value) => {
                if value == 0 {
                    return Ok(());
                };
            }
            Err(_) => return Ok(()),
        }
        let left_name_addr = memory_context.follow_fields::<u64>(&[
            "characterSelectionScreen",
            "leftButton",
            "characterDefinitionId",
        ])?;

        let left_name_str =
            memory_context.read_pointer::<ArrayWString<16>>(left_name_addr + 0x14)?;
        let left_name = match String::from_utf16(left_name_str.as_slice()) {
            Ok(value) => value,
            Err(_) => "None".to_string(),
        };

        let left_selected = match memory_context.follow_fields::<u8>(&[
            "characterSelectionScreen",
            "leftButton",
            "selected",
        ])? {
            0 => false,
            1 => true,
            _ => false,
        };

        let right_name_addr = memory_context.follow_fields::<u64>(&[
            "characterSelectionScreen",
            "rightButton",
            "characterDefinitionId",
        ])?;

        let right_name_str =
            memory_context.read_pointer::<ArrayWString<16>>(right_name_addr + 0x14)?;
        let right_name = match String::from_utf16(right_name_str.as_slice()) {
            Ok(value) => value,
            Err(_) => {
                return Ok(());
            }
        };

        let right_selected = match memory_context.follow_fields::<u8>(&[
            "characterSelectionScreen",
            "rightButton",
            "selected",
        ])? {
            0 => false,
            1 => true,
            _ => false,
        };

        let mut selected = match memory_context
            .follow_fields::<u64>(&["characterSelectionScreen", "selectedCharacter"])
        {
            Ok(value) => {
                if value == 0 {
                    PlayerPartyCharacter::None
                } else {
                    let selected_name_addr = memory_context.follow_fields::<u64>(&[
                        "characterSelectionScreen",
                        "selectedCharacter",
                        "characterDefinitionId",
                    ])?;

                    let selected_name_str = memory_context
                        .read_pointer::<ArrayWString<16>>(selected_name_addr + 0x14)?;

                    let selected_name = match String::from_utf16(selected_name_str.as_slice()) {
                        Ok(value) => value,
                        Err(_) => {
                            return Ok(());
                        }
                    };
                    PlayerPartyCharacter::parse(&selected_name)
                }
            }
            Err(_) => PlayerPartyCharacter::None,
        };

        let character_selected = match memory_context
            .follow_fields::<u8>(&["characterSelectionScreen", "characterSelected"])?
        {
            0 => false,
            1 => true,
            _ => false,
        };

        let left = NewGameCharacter {
            character: PlayerPartyCharacter::parse(&left_name),
            selected: left_selected,
        };

        let right = NewGameCharacter {
            character: PlayerPartyCharacter::parse(&right_name),
            selected: right_selected,
        };

        // character_selected means a character has been selected on the character
        // select screen and is looking at relics, in this case None
        //
        // if character_selected is true in any case, it should be None
        //
        // check if left is 1 and right is 0: return left
        // check if left is 0 and right is 1: return right
        if character_selected {
            selected = PlayerPartyCharacter::None;
        } else {
            // left selected but not right
            if left.selected && !right.selected {
                selected = left.character.clone()
            }
            // right selected but not left
            if right.selected && !left.selected {
                selected = right.character.clone()
            }
        }

        self.new_game_characters = NewGameCharacters {
            left,
            right,
            selected,
        };

        Ok(())
    }
}

#[derive(Default, Debug)]
pub enum TitleMenuOption {
    #[default]
    None,
    Continue,
    NewGame,
    NewGamePlus,
    LoadGame,
    Options,
    QuitGame,
}

#[derive(Default, Debug)]
pub struct NewGameCharacters {
    //  characterSelectionScreen -> leftButton -> characterDefinitionId
    pub left: NewGameCharacter,
    //  characterSelectionScreen -> rightButton -> characterDefinitionId
    pub right: NewGameCharacter,
    //  characterSelectionScreen -> selectedCharacter -> selected (by definition id == right or
    //  left)
    pub selected: PlayerPartyCharacter,
}

#[derive(Default, Debug)]
pub struct NewGameCharacter {
    pub character: PlayerPartyCharacter,
    pub selected: bool,
}

#[derive(Default, Debug)]
pub struct RelicButton {
    // textfield -> m_Text -> Value
    pub name: String,
    // We check the on off switch state to determine if it is enabled or not.
    // onOffSwitchImage -> m_Sprite -> m_CachedPtr -> ptr to base -> 0x0 = string
    pub enabled: bool,
}

impl UnityItem for RelicButton {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        let name_str =
            process.read_pointer_path::<ArrayWString<128>>(item_ptr, &[0x188, 0xD8, 0x14])?;

        if let Ok(name) = String::from_utf16(name_str.as_slice()) {
            let enabled_str = process
                .read_pointer_path::<ArrayCString<16>>(item_ptr, &[0x1B0, 0xD8, 0x10, 0x30, 0x0])?;

            let enabled = !matches!(enabled_str.validate_utf8().unwrap(), "relic-switch-off");

            Ok(RelicButton { name, enabled })
        } else {
            Err(MemoryError::ReadError)
        }
    }
}
