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
        // Check *before* accumulating so the press phase always gets at least
        // one frame: accumulating first meant a frame longer than
        // `press_time` skipped the press entirely and the tap silently
        // vanished (short menu taps dying whenever the update loop ran slow).
        if self.timer < self.press_time {
            gamepad.press(&self.action);
        } else {
            gamepad.release(&self.action);
            if self.done() {
                return true;
            }
        }
        self.timer += delta;
        false
    }

    /// Like [`update`](Self::update) but drives *every* gamepad in lockstep.
    /// Used to mash a prompt when we don't know which player owns it — scripted
    /// fights don't route through `current_player_index`, and the party has
    /// several players, so a single-pad mash can land on the wrong controller.
    pub fn update_all(&mut self, gamepads: &mut [GenericJoystick], delta: f64) -> bool {
        // Same press-before-accumulate guarantee as `update`.
        let pressing = self.timer < self.press_time;
        for gamepad in gamepads.iter_mut() {
            if pressing {
                gamepad.press(&self.action);
            } else {
                gamepad.release(&self.action);
            }
        }
        self.timer += delta;
        !pressing && self.done()
    }

    pub fn done(&self) -> bool {
        self.timer >= self.release_time
    }
}
