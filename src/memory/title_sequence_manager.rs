use crate::memory::MemoryManager;
use crate::state::StateContext;
use memory::memory_manager::unity::*;
use memory::process::{Error, Process};

pub struct TitleSequenceManager {
    pub name: String,
    pub manager: UnityMemoryManager,
    pub data: TitleSequenceManagerData,
}

impl Default for TitleSequenceManager {
    fn default() -> Self {
        let manager = Self {
            name: "TitleSequenceManager".to_string(),
            data: TitleSequenceManagerData::default(),
            manager: UnityMemoryManager {
                ..UnityMemoryManager::default()
            },
        };
        println!("{} Loaded", manager.name);
        manager
    }
}

impl MemoryManager for TitleSequenceManager {
    fn ready_for_updates(&mut self, _ctx: &StateContext) -> bool {
        if let Some(class) = self.manager.singleton {
            if class.class == 0 {
                return false;
            }
            return true;
        }
        false
    }

    fn update_manager(&mut self, ctx: &StateContext) {
        if let Some(process) = &ctx.process {
            if let Some(module) = &ctx.module {
                if let Some(image) = &ctx.image {
                    self.manager.update(process, module, image, &self.name);
                }
            }
        }
    }

    fn update_memory(&mut self, ctx: &StateContext) {
        if self.ready_for_updates(ctx) {
            match self.data.update(ctx, &mut self.manager) {
                Ok(_) => (),
                Err(_error) => self.manager.reset(),
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct TitleSequenceManagerData {
    pub relics: Vec<Relic>,
    pub title_menu: TitleMenu,
}

impl TitleSequenceManagerData {
    pub fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), Error> {
        match self.update_title_menu(ctx, manager) {
          Ok(_) => Ok(()),
          Err(error) => Err(error)
        }
    }

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

#[derive(Default, Debug)]
pub struct Relic {
    pub name: String,
    pub enabled: bool,
    pub selected: bool,
}

#[derive(Default, Debug)]
pub struct UnityItems;
impl UnityItems {
    pub fn count(process: &Process, items_ptr: u64) -> Result<u32, Error> {
        process.read::<u32>(items_ptr + 0x18)
    }
}
