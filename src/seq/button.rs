use crate::control::SosAction;
use joystick::prelude::*;

pub struct ButtonPress {
    pub action: SosAction,
    pub press_time: f64,
    pub release_time: f64,
    pub timer: f64,
}

impl Default for ButtonPress {
    fn default() -> Self {
        Self {
            action: SosAction::Confirm,
            press_time: 0.25,
            release_time: 0.5,
            timer: 0.0,
        }
    }
}

impl ButtonPress {
    pub fn new(action: SosAction) -> Self {
        Self {
            action,
            ..Default::default()
        }
    }

    pub fn update(&mut self, gamepad: &mut GenericJoystick, delta: f64) -> bool {
        self.timer += delta;
        // First press the button
        if self.timer < self.press_time {
            gamepad.press(&self.action);
        } else {
            gamepad.release(&self.action);
            if self.done() {
                return true;
            }
        }
        false
    }

    pub fn done(&self) -> bool {
        self.timer >= self.release_time
    }
}
