use delta::Timer;
use std::fmt::Display;

use joystick::common::{JoystickBtnInterface, JoystickInterface};
use seq::prelude::*;

use crate::seq::button::ButtonPress;
use crate::{
    control::SosAction,
    state::{GameEvent, GameState},
};

#[derive(Default, Debug)]
enum GameFsm {
    Combat,
    #[default]
    Route,
    Cutscene,
}

pub struct GameManager {
    sequencer: Sequencer<GameState, GameEvent>,
    fsm: GameFsm,
    btn: ButtonPress,
    timer: Timer,
    paused: bool,
}

impl Display for GameManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.fsm)
    }
}

impl GameManager {
    pub fn new(root: Box<dyn Node<GameState, GameEvent>>) -> Self {
        Self {
            sequencer: Sequencer::new(root),
            fsm: GameFsm::default(),
            btn: ButtonPress::default(),
            timer: delta::Timer::new(),
            paused: false,
        }
    }

    pub fn pause(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn advance_to_checkpoint(&mut self, context: &mut GameState, checkpoint: &str) -> bool {
        self.sequencer.advance_to_checkpoint(context, checkpoint)
    }

    pub fn run(&mut self, context: &mut GameState) -> bool {
        let dt = self.timer.mark_secs();

        if self.paused {
            context.gamepad.release_all();
            return false;
        }

        let cmd = &context.memory_managers.combat_manager.data;
        let csmd = &context.memory_managers.cutscene_manager.data;

        // TODO(orkaboy): detect game over?
        // TODO(orkaboy): detect level up screen
        if cmd.encounter_active {
            // Stop whatever we're doing and enter combat controller
            context.gamepad.release_all();
            self.fsm = GameFsm::Combat;
        }

        match self.fsm {
            GameFsm::Combat => {
                // TODO(orkaboy): actually handle combat. For now, mash!
                if self.btn.update(&mut context.gamepad, dt) {
                    self.btn = ButtonPress {
                        action: SosAction::Confirm,
                        press_time: 0.1,
                        release_time: 0.2,
                        ..Default::default()
                    };
                }

                if !cmd.encounter_active {
                    context.gamepad.release_all();
                    self.fsm = GameFsm::Route;
                    // Signal return to sequencer
                    self.sequencer.on_event(context, &GameEvent::Combat);
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
                    // Signal return to sequencer
                    self.sequencer.on_event(context, &GameEvent::Cutscene);
                }
            }
        }
        false
    }

    pub fn is_running(&self) -> bool {
        self.sequencer.is_running()
    }
}
