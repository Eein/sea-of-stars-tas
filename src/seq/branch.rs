use crate::state::GameState;

use data::prelude::*;
use seq::prelude::*;

use log::warn;

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

#[derive(Default)]
pub struct CondRelic {
    pub name: &'static str,
}

impl SeqCondition<GameState> for CondRelic {
    fn evaluate(&self, state: &GameState) -> bool {
        match state.config.relics.get(self.name) {
            // If the relic was found in the configuration, return its state
            Some(relic) => {
                *relic
            }
            None => {
                warn!("Relic '{}' not in config, ignored!", self.name);
                false
            }
        }
    }
}
