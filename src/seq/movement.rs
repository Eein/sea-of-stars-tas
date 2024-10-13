use crate::control::SosAction;
use crate::seq::button::ButtonPress;
use crate::state::GameState;

use joystick::prelude::*;
use log::info;
use seq::prelude::*;

use vec3_rs::Vector3;

#[derive(Clone, Debug)]
pub enum Move {
    To(f32, f32, f32),
    ToWorld(f32, f32, f32),
    Climb(f32, f32, f32),
    Interact(f32, f32, f32),
    #[allow(dead_code)]
    WaitFor(f64),
    HoldDir([f32; 2], [f32; 3]),
    HoldDirWorld([f32; 2], [f32; 3]),
    Confirm,
    Log(&'static str),
    ChangeTime(f32), // 0.0-24.0
}

pub struct SeqMove {
    name: &'static str,
    coords: Vec<Move>,
    step: usize, // TODO: Refactor this to be able to go back and forth?
    btn: Option<ButtonPress>,
    timer: f64,
}

impl SeqMove {
    pub fn create(name: &'static str, coords: Vec<Move>) -> Box<Self> {
        Box::new(Self {
            name,
            coords,
            step: 0,
            timer: 0.0,
            btn: None,
        })
    }

    fn is_close(player: &Vector3<f32>, target: &Vector3<f32>, precision: Option<f64>) -> bool {
        const PRECISION: f64 = 0.2;
        let diff = *target - *player;
        diff.magnitude() < precision.unwrap_or(PRECISION)
    }

    fn setup_confirm(&mut self) {
        const PRESS_TIMEOUT: f64 = 0.1;
        const RELEASE_TIMEOUT: f64 = 0.2;
        self.btn = Some(ButtonPress {
            action: SosAction::Confirm,
            press_time: PRESS_TIMEOUT,
            release_time: RELEASE_TIMEOUT,
            ..Default::default()
        });
    }

    fn mash(&mut self, gamepad: &mut GenericJoystick, delta: f64) {
        if let Some(btn) = self.btn.as_mut() {
            if btn.update(gamepad, delta) {
                self.setup_confirm();
            }
        } else {
            self.setup_confirm();
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
            // Put text entry in log
            Move::Log(text) => {
                info!("{}: {}", self.name, text);
                self.step += 1;
            }
            // Move towards the target coordinate until it's reached
            Move::To(x, y, z) => {
                let target = Vector3::new(x, y, z);
                let joy_dir = SeqMove::get_dir(player, &target, false);
                state.gamepad.set_ljoy(joy_dir);
                if SeqMove::is_close(player, &target, None) {
                    self.step += 1;
                }
            }
            // Climb towards the target coordinate until it's reached (mash to get on wall)
            Move::Climb(x, y, z) => {
                let target = Vector3::new(x, y, z);
                let joy_dir = SeqMove::get_dir(player, &target, true);
                state.gamepad.set_ljoy(joy_dir);
                self.mash(&mut state.gamepad, delta);
                if SeqMove::is_close(player, &target, None) {
                    state.gamepad.release_all();
                    self.btn = None;
                    self.step += 1;
                }
            }
            // Move towards the target while mashing
            Move::Interact(x, y, z) => {
                let target = Vector3::new(x, y, z);
                let joy_dir = SeqMove::get_dir(player, &target, false);
                state.gamepad.set_ljoy(joy_dir);
                self.mash(&mut state.gamepad, delta);
                if SeqMove::is_close(player, &target, None) {
                    state.gamepad.release_all();
                    self.btn = None;
                    self.step += 1;
                }
            }
            // Hold still for a period of time
            Move::WaitFor(timeout) => {
                state.gamepad.set_ljoy([0.0, 0.0]); // Make sure we're standing still
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
                if SeqMove::is_close(world_pos, &target, None) {
                    self.step += 1;
                }
            }
            Move::HoldDir(dir, target) => {
                state.gamepad.set_ljoy(dir);
                let target = Vector3::new(target[0], target[1], target[2]);
                if SeqMove::is_close(player, &target, Some(1.0)) {
                    self.step += 1;
                }
            }
            Move::HoldDirWorld(dir, target) => {
                state.gamepad.set_ljoy(dir);
                let target = Vector3::new(target[0], target[1], target[2]);
                let world_pos = &ppmd.position;
                if SeqMove::is_close(world_pos, &target, Some(1.0)) {
                    self.step += 1;
                }
            }
            // Change Time of Day
            Move::ChangeTime(target_time) => self.change_time(state, target_time),
            // Press confirm once
            Move::Confirm => {
                if let Some(btn) = self.btn.as_mut() {
                    if btn.update(&mut state.gamepad, delta) {
                        self.btn = None;
                        self.step += 1;
                        state.gamepad.release_all();
                    }
                } else {
                    self.setup_confirm();
                    state.gamepad.press(&SosAction::Turbo);
                }
            }
        }

        false
    }

    fn exit(&self, state: &mut GameState) {
        state.gamepad.release_all();
    }
}
