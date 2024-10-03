use crate::memory::objects::character::PlayerPartyCharacter;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;

use log::info;

use memory::game_engine::il2cpp::unity_items::{UnityItem, UnityItems};
use memory::game_engine::il2cpp::{Class, Module};
use memory::memory_manager::unity::UnityMemoryManager;
use memory::process::Error;
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

#[derive(Default, Debug)]
pub struct TitleSequenceManagerData {
    /// Title Menu Option data
    pub title_menu_option_selected: TitleMenuOption,
    pub current_screen_name: String,
    /// Information on new game character selection
    pub new_game_characters: NewGameCharacters,
    // relicSelectionScreen -> relicButtons
    pub relic_buttons: UnityItems<RelicButton>,
    /// If saves are loaded and continue shows up on the title screen.
    pub load_save_done: bool,
    /// If the player has pressed start on the intro screen.
    pub pressed_start: bool,
}

impl MemoryManagerUpdate for TitleSequenceManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), Error> {
        if let Some(class) = manager.class {
            if let Some(process) = &ctx.process {
                if let Some(module) = &ctx.module {
                    if let Some(singleton) = manager.singleton {
                        self.update_current_screen_name(class, process, module, singleton)?;
                        self.update_title_menu(class, process, module, singleton)?;
                        self.update_pressed_start(class, process, module, singleton)?;
                        self.update_load_save_done(class, process, module, singleton)?;
                        self.update_relics(class, process, module, singleton)?;
                        self.update_new_game_characters(class, process, module, singleton)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl TitleSequenceManagerData {
    pub fn update_load_save_done(
        &mut self,
        class: Class,
        process: &Process,
        module: &Module,
        singleton: Class,
    ) -> Result<(), Error> {
        if let Ok(load_save_done) =
            class.follow_fields::<u8>(singleton, process, module, &["loadSaveDone"])
        {
            self.load_save_done = match load_save_done {
                1 => true,
                0 => false,
                _ => false,
            };
        }

        Ok(())
    }

    pub fn update_pressed_start(
        &mut self,
        class: Class,
        process: &Process,
        module: &Module,
        singleton: Class,
    ) -> Result<(), Error> {
        if let Ok(pressed_start) =
            class.follow_fields::<u8>(singleton, process, module, &["titleScreen", "startPressed"])
        {
            self.pressed_start = match pressed_start {
                1 => true,
                0 => false,
                _ => false,
            };
        }

        Ok(())
    }

    pub fn update_title_menu(
        &mut self,
        class: Class,
        process: &Process,
        module: &Module,
        singleton: Class,
    ) -> Result<(), Error> {
        if class
            .follow_fields::<u8>(
                singleton,
                process,
                module,
                &["titleScreen", "newGameButton", "selected"],
            )
            .ok()
            == Some(1)
        {
            self.title_menu_option_selected = TitleMenuOption::NewGame;
            return Ok(());
        }
        if class
            .follow_fields::<u8>(
                singleton,
                process,
                module,
                &["titleScreen", "newGamePlusButton", "selected"],
            )
            .ok()
            == Some(1)
        {
            self.title_menu_option_selected = TitleMenuOption::NewGamePlus;
            return Ok(());
        }
        if class
            .follow_fields::<u8>(
                singleton,
                process,
                module,
                &["titleScreen", "continueButton", "selected"],
            )
            .ok()
            == Some(1)
        {
            self.title_menu_option_selected = TitleMenuOption::Continue;
            return Ok(());
        }
        if class
            .follow_fields::<u8>(
                singleton,
                process,
                module,
                &["titleScreen", "loadGameButton", "selected"],
            )
            .ok()
            == Some(1)
        {
            self.title_menu_option_selected = TitleMenuOption::LoadGame;
            return Ok(());
        }
        if class
            .follow_fields::<u8>(
                singleton,
                process,
                module,
                &["titleScreen", "optionsButton", "selected"],
            )
            .ok()
            == Some(1)
        {
            self.title_menu_option_selected = TitleMenuOption::Options;
            return Ok(());
        }
        if class
            .follow_fields::<u8>(
                singleton,
                process,
                module,
                &["titleScreen", "quitGameButton", "selected"],
            )
            .ok()
            == Some(1)
        {
            self.title_menu_option_selected = TitleMenuOption::QuitGame
        }
        Ok(())
    }

    pub fn update_relics(
        &mut self,
        class: Class,
        process: &Process,
        module: &Module,
        singleton: Class,
    ) -> Result<(), Error> {
        if let Ok(relic_buttons) = class.follow_fields::<u64>(
            singleton,
            process,
            module,
            &["relicSelectionScreen", "relicButtons"],
        ) {
            let buttons = UnityItems::<RelicButton>::read(process, relic_buttons)?;
            self.relic_buttons = buttons;
        }

        Ok(())
    }

    pub fn update_current_screen_name(
        &mut self,
        class: Class,
        process: &Process,
        module: &Module,
        singleton: Class,
    ) -> Result<(), Error> {
        if let Ok(addr) =
            class.follow_fields::<u64>(singleton, process, module, &["currentScreenName"])
        {
            let name_str = process.read_pointer::<ArrayWString<128>>(addr + 0x14)?;
            match String::from_utf16(name_str.as_slice()) {
                Ok(value) => {
                    self.current_screen_name = value.clone();
                    Ok(value)
                }
                Err(_) => Err(Error),
            }?;
        }

        Ok(())
    }

    pub fn update_new_game_characters(
        &mut self,
        class: Class,
        process: &Process,
        module: &Module,
        singleton: Class,
    ) -> Result<(), Error> {
        let left_name_addr = class.follow_fields::<u64>(
            singleton,
            process,
            module,
            &[
                "characterSelectionScreen",
                "leftButton",
                "characterDefinitionId",
            ],
        )?;

        let left_name_str = process.read_pointer::<ArrayWString<16>>(left_name_addr + 0x14)?;
        let left_name = match String::from_utf16(left_name_str.as_slice()) {
            Ok(value) => Ok(value),
            Err(_) => Err(Error),
        }?;

        let left_selected = match class.follow_fields::<u8>(
            singleton,
            process,
            module,
            &["characterSelectionScreen", "leftButton", "selected"],
        )? {
            0 => false,
            1 => true,
            _ => false,
        };

        let right_name_addr = class.follow_fields::<u64>(
            singleton,
            process,
            module,
            &[
                "characterSelectionScreen",
                "rightButton",
                "characterDefinitionId",
            ],
        )?;

        let right_name_str = process.read_pointer::<ArrayWString<16>>(right_name_addr + 0x14)?;
        let right_name = match String::from_utf16(right_name_str.as_slice()) {
            Ok(value) => Ok(value),
            Err(_) => Err(Error),
        }?;

        let right_selected = match class.follow_fields::<u8>(
            singleton,
            process,
            module,
            &["characterSelectionScreen", "rightButton", "selected"],
        )? {
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

        let mut selected = PlayerPartyCharacter::None;
        if left.selected {
            selected = left.character.clone();
        }
        if right.selected {
            selected = right.character.clone();
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
    fn read(process: &Process, item_ptr: u64) -> Result<Self, Error> {
        let name_str =
            process.read_pointer_path::<ArrayWString<128>>(item_ptr, &[0x188, 0xD8, 0x14])?;
        let name = match String::from_utf16(name_str.as_slice()) {
            Ok(value) => Ok(value),
            Err(_) => Err(Error),
        }?;

        let enabled_str = process
            .read_pointer_path::<ArrayCString<16>>(item_ptr, &[0x1B0, 0xD8, 0x10, 0x30, 0x0])?;

        let enabled = !matches!(enabled_str.validate_utf8().unwrap(), "relic-switch-off");
        Ok(RelicButton { name, enabled })
    }
}
