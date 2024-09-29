pub mod player_party_manager;
pub mod title_sequence_manager;

use crate::memory::player_party_manager::PlayerPartyManager;
use crate::memory::title_sequence_manager::TitleSequenceManager;
use crate::state::StateContext;

#[derive(Default)]
pub struct MemoryManagers {
    pub title_sequence_manager: TitleSequenceManager,
    pub player_party_manager: PlayerPartyManager,
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

pub trait MemoryManager {
    fn update(&mut self, ctx: &StateContext) {
        self.update_manager(ctx);
        if self.ready_for_updates(ctx) {
            self.update_memory(ctx);
        }
    }

    fn ready_for_updates(&mut self, _state: &StateContext) -> bool {
        true
    }

    fn update_manager(&mut self, state: &StateContext);
    fn update_memory(&mut self, state: &StateContext);
}
