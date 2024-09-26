use crate::memory::memory_manager::MemoryManager;
use crate::state::StateContext;
use memory::memory_manager::unity::*;
use memory::process::{Error, Process};

#[derive(Default)]
pub struct TitleSequenceManager {
    pub name: String,
    pub manager: UnityMemoryManager,
    pub data: TitleSequenceManagerData,
}

impl TitleSequenceManager {
    pub fn new() -> Box<Self> {
        let manager = Self {
            name: "TitleSequenceManager".to_string(),
            data: TitleSequenceManagerData::default(),
            manager: UnityMemoryManager {
                ..UnityMemoryManager::default()
            },
        };
        println!("{} Loaded", manager.name);
        Box::new(manager)
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

// self._read_relics()
// self._read_load_save_done()
// self._read_new_game_characters()
// self._read_pressed_start()
// self._read_continue_selected()
// self._read_new_game_selected()
// self._read_new_game_plus_selected()
// self._read_load_game_selected()
// self._read_options_selected()
// self._read_quit_selected()
// self.title_screen = self.memory.get_field(self.fields_base, "titleScreen")
// self.character_selection_screen = self.memory.get_field(
//     self.fields_base, "characterSelectionScreen"
// )
// self.relic_selection_screen = self.memory.get_field(
//     self.fields_base, "relicSelectionScreen"
// )
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
        // if let Some(class) = manager.manager.class {
        //     if let Some(process) = &ctx.process {
        //         if let Some(module) = &ctx.module {
        //             let items_ptr_base = class.follow_fields(process, module, &["relicSelectionScreen".to_string(), "relicButtons".to_string()]).ok();
        //             println!("0x{:x}", items_ptr_base.unwrap());
        //         }
        //     }
        // }
        // items_ptr_base = self.manager.claslass.follow_fields(self, ["relicSelectionScreen", "relicButtons"])
        // items_ptr = self.memory.follow_pointer(items_ptr_base, [0x0, 0x10, 0x0])
        // relics = []
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
