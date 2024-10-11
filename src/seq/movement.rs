use crate::control::SosAction;
use crate::seq::button::ButtonPress;
use crate::state::GameState;

use joystick::prelude::*;
use seq::prelude::*;

use vec3_rs::Vector3;

#[derive(Clone, Debug)]
pub enum Move {
    To(f32, f32, f32),
    #[allow(dead_code)]
    Climb(f32, f32, f32),
    #[allow(dead_code)]
    Interact(f32, f32, f32),
    #[allow(dead_code)]
    HoldFor(f64),
    #[allow(dead_code)]
    ToWorld(f32, f32, f32),
    ChangeTime(f32), // 0.0-24.0
                     // TODO: Scene transitions
                     // TODO: World map to scene (and back). Need HoldDir?
}

pub struct SeqMove {
    coords: Vec<Move>,
    step: usize, // TODO: Refactor this to be able to go back and forth?
    btn: ButtonPress,
    timer: f64,
}

impl SeqMove {
    pub fn create(coords: Vec<Move>) -> Box<Self> {
        Box::new(Self {
            coords,
            step: 0,
            timer: 0.0,
            btn: ButtonPress::new(SosAction::Confirm),
        })
    }

    fn is_close(player: &Vector3<f32>, target: &Vector3<f32>) -> bool {
        const PRECISION: f64 = 0.1; // TODO: variable?
        let diff = *target - *player;
        diff.magnitude() < PRECISION
    }

    fn mash(&mut self, gamepad: &mut GenericJoystick, delta: f64) {
        const PRESS_TIMEOUT: f64 = 0.1;
        const RELEASE_TIMEOUT: f64 = 0.2;
        if self.btn.update(gamepad, delta) {
            self.btn = ButtonPress {
                action: SosAction::Confirm,
                press_time: PRESS_TIMEOUT,
                release_time: RELEASE_TIMEOUT,
                ..Default::default()
            };
        }
    }

    fn get_dir(player: &Vector3<f32>, target: &Vector3<f32>, climb: bool) -> [f32; 2] {
        let mut diff = *target - *player;
        diff.normalize();
        if climb {
            [diff.get_x(), diff.get_y()]
        } else {
            [diff.get_x(), diff.get_z()]
        }
    }

    fn change_time(&mut self, state: &mut GameState, target_time: f32) {
        const TIME_EPSILON: f32 = 0.3;
        const FULLDAY: f32 = 24.0;
        const MIDDAY: f32 = 12.0;
        let todm = &state.memory_managers.time_of_day_manager.data;
        let cur_time = todm.current_time;

        // Difference in time
        let mut diff_time = target_time - cur_time;
        // Check if done
        if diff_time.abs() < TIME_EPSILON {
            state.gamepad.release_all();
            self.step += 1;
        } else {
            // Adjust time to be in the range 0-24
            if diff_time < 0.0 {
                diff_time += FULLDAY;
            }
            // If diff is in range 0-12, hold RT
            if diff_time < MIDDAY {
                state.gamepad.press(&SosAction::TimeInc);
                state.gamepad.release(&SosAction::TimeDec);
            } else {
                state.gamepad.press(&SosAction::TimeDec);
                state.gamepad.release(&SosAction::TimeInc);
            }
        }
    }
}

impl Node<GameState> for SeqMove {
    fn enter(&mut self, state: &mut GameState) {
        state.gamepad.release_all();
    }

    fn execute(&mut self, state: &mut GameState, delta: f64) -> bool {
        if self.step >= self.coords.len() {
            return true;
        }

        let coord = self.coords[self.step].clone();

        let ppmd = &state.memory_managers.player_party_manager.data;
        let player = &ppmd.gameobject_position;

        match coord {
            // Move towards the target coordinate until it's reached
            Move::To(x, y, z) => {
                let target = Vector3::new(x, y, z);
                let joy_dir = SeqMove::get_dir(player, &target, false);
                state.gamepad.set_ljoy(joy_dir);
                if SeqMove::is_close(player, &target) {
                    self.step += 1;
                }
            }
            // Climb towards the target coordinate until it's reached (mash to get on wall)
            Move::Climb(x, y, z) => {
                let target = Vector3::new(x, y, z);
                let joy_dir = SeqMove::get_dir(player, &target, true);
                state.gamepad.set_ljoy(joy_dir);
                self.mash(&mut state.gamepad, delta);
                if SeqMove::is_close(player, &target) {
                    state.gamepad.release_all();
                    self.step += 1;
                }
            }
            // Move towards the target while mashing
            Move::Interact(x, y, z) => {
                let target = Vector3::new(x, y, z);
                let joy_dir = SeqMove::get_dir(player, &target, false);
                state.gamepad.set_ljoy(joy_dir);
                self.mash(&mut state.gamepad, delta);
                if SeqMove::is_close(player, &target) {
                    state.gamepad.release_all();
                    self.step += 1;
                }
            }
            // Hold still for a period of time
            Move::HoldFor(timeout) => {
                self.timer += delta;
                if self.timer >= timeout {
                    self.timer = 0.0;
                    self.step += 1;
                }
            }
            // Move towards the target coordinate until it's reached (World map, uses different coords)
            Move::ToWorld(x, y, z) => {
                let target = Vector3::new(x, y, z);
                let world_pos = &ppmd.position;
                let joy_dir = SeqMove::get_dir(world_pos, &target, false);
                state.gamepad.set_ljoy(joy_dir);
                if SeqMove::is_close(world_pos, &target) {
                    self.step += 1;
                }
            }
            // Change Time of Day
            Move::ChangeTime(target_time) => self.change_time(state, target_time),
        }

        false
    }

    fn exit(&self, state: &mut GameState) {
        state.gamepad.release_all();
    }
}
