use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;

use log::info;

use memory::game_engine::il2cpp::{Class, Module};
use memory::memory_manager::unity::UnityMemoryManager;
use memory::process::Error;
use memory::process::Process;

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
    /// Title Menu Object Data.
    pub title_menu: TitleMenu,
    // relicSelectionScreen -> relicButtons
    pub relic_buttons: RelicButtons,
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
                        self.update_title_menu(class, process, module, singleton)?;
                        self.update_pressed_start(class, process, module, singleton)?;
                        self.update_load_save_done(class, process, module, singleton)?;
                        self.update_relics(class, process, module, singleton)?;
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
            self.title_menu.selected = TitleMenuOption::NewGame;
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
            self.title_menu.selected = TitleMenuOption::NewGamePlus;
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
            self.title_menu.selected = TitleMenuOption::Continue;
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
            self.title_menu.selected = TitleMenuOption::LoadGame;
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
            self.title_menu.selected = TitleMenuOption::Options;
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
            self.title_menu.selected = TitleMenuOption::QuitGame
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
        let mut buttons = vec![];

        if let Ok(relic_buttons) = class.follow_fields::<u64>(singleton, process, module, &["relicSelectionScreen", "relicButtons"]) {
            let mut fields_base = 0x20;
            let offset = 0x8;

            let items_ptr = process.read_pointer::<u64>(relic_buttons + 0x10)?;
            let count = process.read_pointer::<u32>(items_ptr + 0x18)?;

            if count == 0 { return Ok(()) }
            
            // TODO(eein): try to convert this items reader to be generic somehow
            // maybe something like UnityItems<RelicButton> and have Relic Button implement how it
            // gets and stores its own data with a UnityItem trait
            for _index in 0..count {
                let item_ptr = process.read_pointer::<u64>(items_ptr + fields_base)?;
                if item_ptr == 0 { break; }

                // NAME
                let len = process.read_pointer_path::<u8>(item_ptr, &[0x188, 0xD8, 0x10])?;
                let addr = process.read_pointer_path_without_read(item_ptr, &[0x188, 0xD8, 0x14])?;
                
                // Get the buffer for the name and store it
                // TODO(eein): convert this utf8 string reader to a helper function
                let mut buf = vec![0; (len * 2) as usize];
                let current_read_buf = &mut buf[..(len * 2) as usize];
                let current_read_buf = process.read_into_uninit_buf(addr, current_read_buf)?;
                let name = std::str::from_utf8(current_read_buf).ok().unwrap();

                // ENABLED 
                // TODO(eein): rewrite this to a CSTR lookup looking for null terminator
                let addr = process.read_pointer_path_without_read(item_ptr, &[0x1B0, 0xD8, 0x10, 0x30, 0x0])?;
                let mut buf = [0; 16];
                let current_read_buf = &mut buf[..16];
                let current_read_buf = process.read_into_uninit_buf(addr, current_read_buf)?;
                let enabled_str = std::str::from_utf8(current_read_buf).ok().unwrap();

                let enabled = !matches!(enabled_str.trim(), "relic-switch-off");

                buttons.push(RelicButton {
                    name: name.to_string(),
                    enabled
                });

                fields_base += offset;
            }
            self.relic_buttons = RelicButtons {
                count,
                buttons,
            };
        }

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
pub struct TitleMenu {
    pub selected: TitleMenuOption,
}

#[derive(Default, Debug)]
pub struct RelicButtons {
    pub count: u32, // 0x18
    pub buttons: Vec<RelicButton>, // start at 0x20
}

#[derive(Default, Debug)]
pub struct RelicButton {
    // textfield -> m_Text -> Value
    pub name: String,
    // We check the on off switch state to determine if it is enabled or not.
    // onOffSwitchImage -> m_Sprite -> m_CachedPtr -> ptr to base -> 0x0 = string
    pub enabled: bool,

}
