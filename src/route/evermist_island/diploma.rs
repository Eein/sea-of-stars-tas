use std::fmt::Display;
use crate::control::SosAction;
use crate::seq::button::ButtonPress;
use crate::state::{GameEvent, GameState};
use joystick::common::JoystickBtnInterface;
use joystick::common::JoystickInterface;
use seq::prelude::*;

#[derive(Debug)]
enum DiplomaFsm {
    Wait(f64),
    TapRight,
    Confirm,
}

pub struct SeqDiploma {
    btn: ButtonPress,
    fsm: DiplomaFsm,
}

impl SeqDiploma {
    pub fn create() -> Box<Self> {
        Box::new(Self {
            btn: ButtonPress::new(SosAction::MenuRight),
            fsm: DiplomaFsm::Wait(0.0),
        })
    }
}

const TIMEOUT: f64 = 3.0;

impl Display for SeqDiploma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Diploma({:?})", self.fsm)
    }
}

impl Node<GameState, GameEvent> for SeqDiploma {
    fn execute(&mut self, state: &mut GameState, delta: f64) -> bool {
        // Hold cancel to skip cut-scenes
        state.gamepad.press(&SosAction::Cancel);
        // Done when we are back in control
        match &self.fsm {
            DiplomaFsm::Wait(timer) => {
                if *timer >= TIMEOUT {
                    self.fsm = DiplomaFsm::TapRight;
                } else {
                    self.fsm = DiplomaFsm::Wait(timer + delta);
                }
            }
            DiplomaFsm::TapRight => {
                if self.btn.update(&mut state.gamepad, delta) {
                    self.btn = ButtonPress::new(SosAction::Confirm);
                    self.fsm = DiplomaFsm::Confirm;
                }
            }
            DiplomaFsm::Confirm => {
                if self.btn.update(&mut state.gamepad, delta) {
                    return true;
                }
            }
        }
        false
    }

    fn cutscene_control(&self) -> bool {
        true
    }

    fn exit(&self, state: &mut GameState) {
        // Cleanup, release buttons
        state.gamepad.release_all();
    }
}
