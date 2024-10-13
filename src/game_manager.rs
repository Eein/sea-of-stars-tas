use joystick::common::JoystickInterface;
use seq::prelude::*;

use crate::state::GameState;

#[derive(Default)]
enum GameFsm {
    Combat,
    #[default]
    Route,
}

pub struct GameManager {
    sequencer: Sequencer<GameState>,
    fsm: GameFsm,
}

impl GameManager {
    pub fn create(root: Box<dyn Node<GameState>>) -> Self {
        Self {
            sequencer: Sequencer::create(root),
            fsm: GameFsm::default(),
        }
    }

    pub fn start(&mut self) {
        self.sequencer.start();
    }

    pub fn run(&mut self, context: &mut GameState) -> bool {
        let cmd = &context.memory_managers.combat_manager.data;

        // TODO(orkaboy): detect game over?
        // TODO(orkaboy): detect level up screen
        if cmd.encounter_active {
            // Stop whatever we're doing and enter combat controller
            context.gamepad.release_all();
            self.fsm = GameFsm::Combat;
        }

        match self.fsm {
            GameFsm::Combat => {
                // TODO(orkaboy): actually handle combat. For now, handle manually
                if !cmd.encounter_active {
                    // TODO(orkaboy): Relinquish to sequencer immediately or hold until we're sure combat isn't done
                    // TODO(orkaboy): Signal return to sequencer?
                    self.fsm = GameFsm::Route;
                }
            },
            GameFsm::Route => {
                // Sequencer has lower prio
                return self.sequencer.run(context)
            }
        }
        false
    }

    pub fn is_running(&self) -> bool {
        self.sequencer.is_running()
    }
}
