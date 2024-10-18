use crate::state::GameState;

use data::prelude::*;
use seq::prelude::*;

// Test condition. Checks if value is greater than
#[derive(Default)]
pub struct CondMainChar {
    pub leader: PlayerPartyCharacter,
}

impl SeqCondition<GameState> for CondMainChar {
    fn evaluate(&self, state: &GameState) -> bool {
        let ppmd = &state.memory_managers.player_party_manager.data;
        self.leader == ppmd.leader_character
    }
}
