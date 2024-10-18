use crate::memory::objects::character::PlayerPartyCharacter;
use crate::state::GameState;

use seq::prelude::*;

// Test condition. Checks if value is greater than
#[derive(Default)]
pub struct CondMainChar {
    #[allow(dead_code)]
    pub leader: PlayerPartyCharacter,
}

impl SeqCondition<GameState> for CondMainChar {
    fn evaluate(&self, _state: &GameState) -> bool {
        //TODO(orkaboy): Implement
        true
        //let ppmd = &state.memory_managers.player_party_manager.data;
        //self.leader == ppmd.leader_character
    }
}
