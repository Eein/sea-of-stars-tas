use memory::game_engine::il2cpp::*;
use crate::memory::memory_manager::MemoryManager;
use crate::state::StateContext;
use memory::memory_manager::unity::*;
use memory::process::{Process, Error};
use memory::watcher::Watcher;

#[derive(Default)]
pub struct TitleSequenceManager {
    pub name: String,
    pub manager: UnityMemoryManager,
    pub data: TitleSequenceManagerData
}

impl TitleSequenceManager {
    pub fn new() -> Box<Self> {
        Box::new(
            Self { 
                name: "TitleSequenceManager".to_string(),
                data: TitleSequenceManagerData::default(),
                manager: UnityMemoryManager {
                    ..UnityMemoryManager::default() 
                }
            }
        )
    }
}

impl MemoryManager for TitleSequenceManager {
    fn update(&mut self, ctx: &StateContext) {
        self.update_manager(&ctx);
        self.update_memory(&ctx);
    }

    fn update_manager(&mut self, ctx: &StateContext) {
        if let Some(process) = &ctx.process {
            if let Some(module) = &ctx.module {
                if let Some(image) = &ctx.image {
                    self.manager.update(&process, &module, &image, &self.name);
                }
            }
        }
    }

    fn update_memory(&mut self, ctx: &StateContext) {
        if self.manager.instance.is_some() {
            self.data.update(&ctx)

        }
    }
}

#[derive(Default, Debug)]

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
pub struct TitleSequenceManagerData {
    pub relics: Vec<Relic>,
}

impl TitleSequenceManagerData {
    pub fn update(&self, ctx: &StateContext) {
        self.update_relics(ctx)
    }

    pub fn update_relics(&self, ctx: &StateContext) {
        // items_ptr_base = self.memory.follow_fields(self, ["relicSelectionScreen", "relicButtons"])
        // items_ptr = self.memory.follow_pointer(items_ptr_base, [0x0, 0x10, 0x0])
        // relics = []

    }
}


#[derive(Default, Debug)]
pub struct Relic {
    name: String,
    enabled: bool,
    selected: bool,
}


pub struct UnityItems;
impl UnityItems {
    pub fn count(process: &Process, items_ptr: u64) -> Result<u32, Error> {
       process.read::<u32>(items_ptr + 0x18)
    }
}

