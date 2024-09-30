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
