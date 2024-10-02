use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::unity::UnityMemoryManager;
use memory::process::Error;

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
    pub title_menu: TitleMenu,
}

impl MemoryManagerUpdate for TitleSequenceManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), Error> {
        match self.update_title_menu(ctx, manager) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
}

impl TitleSequenceManagerData {
    pub fn update_title_menu(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), Error> {
        if let Some(class) = manager.class {
            if let Some(process) = &ctx.process {
                if let Some(module) = &ctx.module {
                    if let Some(singleton) = manager.singleton {
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
                    }
                }
            }
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
