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
        self.title_sequence_manager.update(ctx);
        self.player_party_manager.update(ctx);
    }
}

pub trait MemoryManager {
    fn update(&mut self, ctx: &StateContext) {
        self.update_manager(ctx);
        self.update_memory(ctx);
    }

    fn update_manager(&mut self, state: &StateContext);
    fn update_memory(&mut self, state: &StateContext);
}
