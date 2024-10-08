use crate::control::SosAction;
use crate::state::GameState;
use joystick::prelude::*;
use seq::prelude::*;

// Temp, press the confirm button for x time
pub struct SeqConfirm {
    timer: f64,
    timeout: f64,
}

impl SeqConfirm {
    pub fn create(timeout: f64) -> Box<Self> {
        Box::new(Self {
            timer: 0.0,
            timeout,
        })
    }
}

impl Node<GameState> for SeqConfirm {
    fn enter(&mut self, state: &mut GameState) {
        state.gamepad.press(&SosAction::Confirm);
    }

    fn execute(&mut self, _state: &mut GameState, delta: f64) -> bool {
        self.timer += delta;
        self.timer >= self.timeout
    }

    fn exit(&self, state: &mut GameState) {
        state.gamepad.release(&SosAction::Confirm);
    }
}
