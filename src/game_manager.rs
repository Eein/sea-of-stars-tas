use seq::prelude::*;

use crate::state::GameState;

pub struct GameManager {
    sequencer: Sequencer<GameState>,
}

impl GameManager {
    pub fn create(root: Box<dyn Node<GameState>>) -> Self {
        Self {
            sequencer: Sequencer::create(root),
        }
    }

    pub fn start(&mut self) {
        self.sequencer.start();
    }

    pub fn run(&mut self, context: &mut GameState) -> bool {
        let cmd = &context.memory_managers.combat_manager.data;

        // TODO(orkaboy): level up screen
        // TODO(orkaboy): actually handle combat
        if cmd.encounter_active {
            return false;
        }
        // Sequencer has lower prio
        self.sequencer.run(context)
    }

    pub fn is_running(&self) -> bool {
        self.sequencer.is_running()
    }
}
