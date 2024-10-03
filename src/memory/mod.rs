pub mod objects;
pub mod player_party_manager;
pub mod title_sequence_manager;

use log::error;

use crate::state::StateContext;

use memory::memory_manager::unity::{UnityMemoryManagement, UnityMemoryManager};
use memory::process::Error;
use player_party_manager::PlayerPartyManagerData;
use title_sequence_manager::TitleSequenceManagerData;

pub trait MemoryManagerUpdate {
    fn update(&mut self, ctx: &StateContext, manager: &mut UnityMemoryManager)
        -> Result<(), Error>;
}

pub struct MemoryManager<T: MemoryManagerUpdate> {
    pub name: String,
    pub manager: UnityMemoryManager,
    pub data: T,
}

#[derive(Default)]
pub struct MemoryManagers {
    pub title_sequence_manager: MemoryManager<TitleSequenceManagerData>,
    pub player_party_manager: MemoryManager<PlayerPartyManagerData>,
}

impl MemoryManagers {
    pub fn update(&mut self, ctx: &StateContext) {
        if self.ready_for_updates(ctx) {
            self.title_sequence_manager.update(ctx);
            self.player_party_manager.update(ctx);
        }
    }
    pub fn ready_for_updates(&mut self, ctx: &StateContext) -> bool {
        ctx.process.is_some() && ctx.module.is_some() && ctx.image.is_some()
    }
}

impl<T: MemoryManagerUpdate> MemoryManager<T> {
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
        match self.data.update(ctx, &mut self.manager) {
            Ok(_) => (),
            Err(_error) => {
                error!("Memory Update Error in {}", self.name);
                self.manager.reset()
            }
        }
    }

    fn update(&mut self, ctx: &StateContext) {
        self.update_manager(ctx);
        if self.ready_for_updates(ctx) {
            self.update_memory(ctx);
        }
    }
}
