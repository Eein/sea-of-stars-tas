use std::fmt::Display;
use delta::Timer;

use joystick::common::{JoystickBtnInterface, JoystickInterface};
use seq::prelude::*;

use crate::{control::SosAction, state::GameState};
use crate::seq::button::ButtonPress;

#[derive(Default, Debug)]
enum GameFsm {
    Combat,
    #[default]
    Route,
    Cutscene,
}

pub struct GameManager {
    sequencer: Sequencer<GameState>,
    fsm: GameFsm,
    btn: ButtonPress,
    timer: Timer,
}

impl Display for GameManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.fsm)
    }
}

impl GameManager {
    pub fn new(root: Box<dyn Node<GameState>>) -> Self {
        Self {
            sequencer: Sequencer::new(root),
            fsm: GameFsm::default(),
            btn: ButtonPress::default(),
            timer: delta::Timer::new(),
        }
    }

    pub fn run(&mut self, context: &mut GameState) -> bool {
        let cmd = &context.memory_managers.combat_manager.data;
        let csmd = &context.memory_managers.cutscene_manager.data;

        let dt = self.timer.mark_secs();

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
            }
            GameFsm::Route => {
                if csmd.is_in_cutscene && !self.sequencer.cutscene_control() {
                    self.fsm = GameFsm::Cutscene;
                } else {
                    // Sequencer has lower prio
                    return self.sequencer.run(context, dt);
                }
            }
            GameFsm::Cutscene => {
                context.gamepad.press(&SosAction::Cancel);
                context.gamepad.press(&SosAction::Turbo);
                if self.btn.update(&mut context.gamepad, dt) {
                    self.btn = ButtonPress::new(SosAction::Confirm);
                }
                if !csmd.is_in_cutscene {
                    context.gamepad.release_all();
                    self.fsm = GameFsm::Route;
                }
            }
        }
        false
    }

    pub fn is_running(&self) -> bool {
        self.sequencer.is_running()
    }
}
