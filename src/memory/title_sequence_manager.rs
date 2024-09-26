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
        if self.manager.instance.is_some() {
            self.data.update(ctx, &mut self.manager)
        }
    }
}

#[derive(Default, Debug)]
pub struct TitleSequenceManagerData {
    pub relics: Vec<Relic>,
    pub title_menu: TitleMenu,
}

impl TitleSequenceManagerData {
    pub fn update(&mut self, ctx: &StateContext, manager: &mut UnityMemoryManager) {
        self.update_title_menu(ctx, manager);
        self.update_relics(ctx, manager);
    }

    pub fn update_title_menu(&mut self, ctx: &StateContext, manager: &mut UnityMemoryManager) {
        if let Some(class) = manager.class {
            if let Some(process) = &ctx.process {
                if let Some(module) = &ctx.module {
                    if let Some(singleton) = manager.singleton {
                        if class
                            .follow_fields(
                                singleton,
                                process,
                                module,
                                &["titleScreen", "newGameButton", "selected"],
                            )
                            .ok()
                            == Some(1)
                        {
                            self.title_menu.selected = TitleMenuOption::NewGame;
                            return;
                        }
                        if class
                            .follow_fields(
                                singleton,
                                process,
                                module,
                                &["titleScreen", "newGamePlusButton", "selected"],
                            )
                            .ok()
                            == Some(1)
                        {
                            self.title_menu.selected = TitleMenuOption::NewGamePlus;
                            return;
                        }
                        if class
                            .follow_fields(
                                singleton,
                                process,
                                module,
                                &["titleScreen", "continueButton", "selected"],
                            )
                            .ok()
                            == Some(1)
                        {
                            self.title_menu.selected = TitleMenuOption::Continue;
                            return;
                        }
                        if class
                            .follow_fields(
                                singleton,
                                process,
                                module,
                                &["titleScreen", "loadGameButton", "selected"],
                            )
                            .ok()
                            == Some(1)
                        {
                            self.title_menu.selected = TitleMenuOption::LoadGame;
                            return;
                        }
                        if class
                            .follow_fields(
                                singleton,
                                process,
                                module,
                                &["titleScreen", "optionsButton", "selected"],
                            )
                            .ok()
                            == Some(1)
                        {
                            self.title_menu.selected = TitleMenuOption::Options;
                            return;
                        }
                        if class
                            .follow_fields(
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
    }

    pub fn update_relics(&mut self, _ctx: &StateContext, _manager: &UnityMemoryManager) {
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
    selected: TitleMenuOption,
}

#[derive(Default, Debug)]
pub struct Relic {
    name: String,
    enabled: bool,
    selected: bool,
}

#[derive(Default, Debug)]
pub struct UnityItems;
impl UnityItems {
    pub fn count(process: &Process, items_ptr: u64) -> Result<u32, Error> {
        process.read::<u32>(items_ptr + 0x18)
    }
}
