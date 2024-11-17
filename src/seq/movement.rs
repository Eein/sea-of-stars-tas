use std::fmt::{Display, Formatter, Result};

use crate::control::SosAction;
use crate::seq::button::ButtonPress;
use crate::state::{GameEvent, GameState};

use joystick::prelude::*;
use log::info;
use seq::prelude::*;

use vec2;
use vec3_rs::Vector3;

#[derive(Clone, Debug)]
pub enum Move {
    Join,
    Leave([f32; 2]), // Argument is a joystick direction to hold
    To(f32, f32, f32),
    ToWorld(f32, f32, f32),
    Towards([f32; 3], [f32; 3], bool),
    Climb(f32, f32, f32),
    Interact(f32, f32, f32),
    WaitFor(f64),
    HoldDir([f32; 2], [f32; 3]),
    HoldDirWorld([f32; 2], [f32; 3]),
    Confirm,
    Log(&'static str),
    ChangeTime(f32),          // 0.0-24.0
    AwaitCombat(Box<Move>),   // Break inner Move when combat is done
    AwaitCutscene(Box<Move>), // Break inner Move when cutscene is done
    AwaitSync(Vec<usize>),    // Await GameEvent::CoopSync from list of player IDs
    SpeedBoost(Vec<usize>),   // Note: Need to sync with another player to trigger
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Move::Join => write!(f, "Move::Join"),
            Move::Leave(joy) => write!(f, "Move::Leave({:?})", joy),
            Move::To(x, y, z) => write!(f, "Move::To({:.3}, {:.3}, {:.3})", x, y, z),
            Move::ToWorld(x, y, z) => write!(f, "Move::ToWorld({:.3}, {:.3}, {:.3})", x, y, z),
            Move::Towards(target, anchor, mash) => {
                write!(f, "Move::Towards({:#?}, {:#?}, {})", target, anchor, mash)
            }
            Move::Climb(x, y, z) => write!(f, "Move::Climb({:.3}, {:.3}, {:.3})", x, y, z),
            Move::Interact(x, y, z) => write!(f, "Move::Interact({:.3}, {:.3}, {:.3})", x, y, z),
            Move::WaitFor(duration) => write!(f, "Move::WaitFor({:.3})", duration),
            Move::HoldDir(joy, target) => write!(f, "Move::HoldDir({:?}, {:?})", joy, target),
            Move::HoldDirWorld(joy, target) => {
                write!(f, "Move::HoldDirWorld({:?}, {:?})", joy, target)
            }
            Move::Confirm => write!(f, "Move::Confirm"),
            Move::Log(text) => write!(f, "Move::Log(\"{}\")", text),
            Move::ChangeTime(time) => write!(f, "Move::ChangeTime({:.3})", time),
            Move::AwaitCombat(inner) => write!(f, "Move::AwaitCombat(Box::new({}))", inner),
            Move::AwaitCutscene(inner) => write!(f, "Move::AwaitCutscene(Box::new({}))", inner),
            Move::AwaitSync(list) => write!(f, "Move::AwaitSync({:?})", list),
            Move::SpeedBoost(list) => write!(f, "Move::SpeedBoost({:?})", list),
        }
    }
}

pub struct MovePath {
    name: String,
    coords: Vec<Move>,
    step: usize,
    btn: Option<ButtonPress>,
    timer: f64,
    player: usize,
    // Sync stuff
    semaphore: Vec<usize>,
    sent_signal: bool,
}

enum PathStatus {
    Running,
    Sync(Vec<usize>), // Signal to others when we reach an AwaitSync
    Done,
}

impl MovePath {
    pub fn new(name: String, player: usize, coords: Vec<Move>) -> Self {
        if player > 2 {
            panic!(
                "MovePath({}) initialized with a player index > 2: {}!",
                name, player
            );
        }
        Self {
            name,
            coords,
            step: 0,
            timer: 0.0,
            btn: None,
            player,
            semaphore: vec![],
            sent_signal: false,
        }
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
        let diff = *target - *player;
        let mut diff2d = if climb {
            [diff.get_x(), diff.get_y()]
        } else {
            [diff.get_x(), diff.get_z()]
        };
        vec2::norm_mut(&mut diff2d);
        diff2d
    }

    fn change_time(&mut self, state: &mut GameState, target_time: f32) {
        const TIME_EPSILON: f32 = 0.3;
        const FULLDAY: f32 = 24.0;
        const MIDDAY: f32 = 12.0;
        let todm = &state.memory_managers.time_of_day_manager.data;
        let cur_time = todm.current_time;

        let gamepad = &mut state.gamepads[self.player];

        // Difference in time
        let mut diff_time = target_time - cur_time;
        // Check if done
        if diff_time.abs() < TIME_EPSILON {
            gamepad.release_all();
            self.step += 1;
        } else {
            // Adjust time to be in the range 0-24
            if diff_time < 0.0 {
                diff_time += FULLDAY;
            }
            // If diff is in range 0-12, hold RT
            if diff_time < MIDDAY {
                gamepad.press(&SosAction::TimeInc);
                gamepad.release(&SosAction::TimeDec);
            } else {
                gamepad.press(&SosAction::TimeDec);
                gamepad.release(&SosAction::TimeInc);
            }
        }
    }

    fn handle_coord(&mut self, state: &mut GameState, coord: Move, delta: f64) -> PathStatus {
        let sppmd = &state.memory_managers.single_player_plus_manager.data;
        let player = &sppmd.players.items[self.player].gameobject_position;

        let gamepad = &mut state.gamepads[self.player];

        match coord {
            // Run the inner command
            Move::AwaitCombat(inner) => {
                return self.handle_coord(state, *inner, delta);
            }
            Move::AwaitCutscene(inner) => {
                return self.handle_coord(state, *inner, delta);
            }
            // Leave/Join
            Move::Join => {
                if sppmd.players.items[self.player].playing {
                    gamepad.release_all();
                    self.step += 1;
                } else {
                    self.timer += delta;
                    // Stagger join by means of a timer
                    if self.timer >= (self.player as f64 - 1.0) * 1.0 {
                        self.timer = 0.0;
                        gamepad.press(&SosAction::Join);
                    }
                }
            }
            Move::Leave(joy) => {
                if self.player == 0 {
                    self.step += 1;
                } else if !sppmd.players.items[self.player].playing {
                    gamepad.release_all();
                    self.step = self.coords.len();
                } else {
                    gamepad.set_ljoy(joy);
                    gamepad.press(&SosAction::Leave);
                }
            }
            // Synchronize with a list of other players
            Move::AwaitSync(list) => {
                gamepad.release_all();
                if !self.sent_signal {
                    self.sent_signal = true;
                    return PathStatus::Sync(list.clone());
                }
                // First, check if all the players we are waiting for have signalled us
                let mut wait_done = true;
                for c in &list {
                    if !self.semaphore.contains(c) {
                        wait_done = false;
                        break;
                    }
                }
                if wait_done {
                    self.step += 1;
                    self.sent_signal = false;
                    // Remove the players we were waiting on from the semaphore list
                    for c in list {
                        if let Some(pos) = self.semaphore.iter().position(|x| *x == c) {
                            self.semaphore.swap_remove(pos);
                        }
                    }
                }
            }
            Move::SpeedBoost(list) => {
                if !list.contains(&self.player) || sppmd.players.items[self.player].has_boost {
                    gamepad.release_all();
                    self.step += 1;
                } else {
                    gamepad.press(&SosAction::HiFive);
                }
            }
            // Put text entry in log
            Move::Log(text) => {
                info!("{}: {}", self.name, text);
                self.step += 1;
            }
            // Move towards an anchor, until reached target. Optionally, mash
            Move::Towards(target, anchor, mash) => {
                let target = Vector3::new(target[0], target[1], target[2]);
                let anchor = Vector3::new(anchor[0], anchor[1], anchor[2]);
                let joy_dir = MovePath::get_dir(player, &anchor, false);
                gamepad.set_ljoy(joy_dir);
                if mash {
                    self.mash(gamepad, delta);
                }
                if MovePath::is_close(player, &target, Some(1.0)) {
                    gamepad.release_all();
                    self.btn = None;
                    self.step += 1;
                }
            }
            // Move towards the target coordinate until it's reached
            Move::To(x, y, z) => {
                let target = Vector3::new(x, y, z);
                let joy_dir = MovePath::get_dir(player, &target, false);
                gamepad.set_ljoy(joy_dir);
                if MovePath::is_close(player, &target, None) {
                    self.step += 1;
                }
            }
            // Climb towards the target coordinate until it's reached (mash to get on wall)
            Move::Climb(x, y, z) => {
                let target = Vector3::new(x, y, z);
                let joy_dir = MovePath::get_dir(player, &target, true);
                gamepad.set_ljoy(joy_dir);
                self.mash(gamepad, delta);
                if MovePath::is_close(player, &target, None) {
                    gamepad.release_all();
                    self.btn = None;
                    self.step += 1;
                }
            }
            // Move towards the target while mashing
            Move::Interact(x, y, z) => {
                let target = Vector3::new(x, y, z);
                let joy_dir = MovePath::get_dir(player, &target, false);
                gamepad.set_ljoy(joy_dir);
                // If we are close to target, stop mashing to prevent unintended jumps
                const INTERACT_PRECISION: f64 = 1.0;
                if !MovePath::is_close(player, &target, Some(INTERACT_PRECISION)) {
                    self.mash(gamepad, delta);
                } else {
                    gamepad.release(&SosAction::Confirm);
                }
                // If we are even closer, proceed.
                if MovePath::is_close(player, &target, None) {
                    gamepad.release_all();
                    self.btn = None;
                    self.step += 1;
                }
            }
            // Hold still for a period of time
            Move::WaitFor(timeout) => {
                gamepad.set_ljoy([0.0, 0.0]); // Make sure we're standing still
                self.timer += delta;
                if self.timer >= timeout {
                    self.timer = 0.0;
                    self.step += 1;
                }
            }
            // Move towards the target coordinate until it's reached (World map, uses different coords)
            Move::ToWorld(x, y, z) => {
                let target = Vector3::new(x, y, z);
                let world_pos = &sppmd.players.items[self.player].position;
                let joy_dir = MovePath::get_dir(world_pos, &target, false);
                gamepad.set_ljoy(joy_dir);
                if MovePath::is_close(world_pos, &target, None) {
                    self.step += 1;
                }
            }
            Move::HoldDir(dir, target) => {
                gamepad.set_ljoy(dir);
                let target = Vector3::new(target[0], target[1], target[2]);
                if MovePath::is_close(player, &target, Some(1.0)) {
                    self.step += 1;
                }
            }
            Move::HoldDirWorld(dir, target) => {
                gamepad.set_ljoy(dir);
                let target = Vector3::new(target[0], target[1], target[2]);
                let world_pos = &sppmd.players.items[self.player].position;
                if MovePath::is_close(world_pos, &target, Some(1.0)) {
                    self.step += 1;
                }
            }
            // Change Time of Day
            Move::ChangeTime(target_time) => self.change_time(state, target_time),
            // Press confirm once
            Move::Confirm => {
                if let Some(btn) = self.btn.as_mut() {
                    if btn.update(gamepad, delta) {
                        self.btn = None;
                        self.step += 1;
                        gamepad.release_all();
                    }
                } else {
                    gamepad.release_all(); // Release held joystick direction
                    self.setup_confirm();
                    //TODO: gamepad.press(&SosAction::Turbo);
                }
            }
        }
        PathStatus::Running
    }

    fn on_event(&mut self, _state: &mut GameState, event: &GameEvent) {
        if self.step >= self.coords.len() {
            return;
        }

        let coord = self.coords[self.step].clone();

        match event {
            GameEvent::Combat => {
                if let Move::AwaitCombat(_) = coord {
                    self.btn = None;
                    self.step += 1;
                }
            }
            GameEvent::Cutscene => {
                if let Move::AwaitCutscene(_) = coord {
                    self.btn = None;
                    self.step += 1;
                }
            }
            GameEvent::CoopSync(player) => {
                self.semaphore.push(*player);
            }
        }
    }

    fn execute(&mut self, state: &mut GameState, delta: f64) -> PathStatus {
        if self.step >= self.coords.len() {
            return PathStatus::Done;
        }

        let coord = self.coords[self.step].clone();
        self.handle_coord(state, coord, delta)
    }
}

impl Display for MovePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        if self.step < self.coords.len() {
            ret = format!(
                "[{}/{}] -> {}",
                self.step + 1,
                self.coords.len(),
                self.coords[self.step]
            );
        }
        write!(f, "{}", ret)
    }
}

pub struct SeqMove {
    name: &'static str,
    paths: Vec<MovePath>,
}

impl SeqMove {
    pub fn create(name: &'static str, coords: Vec<Move>) -> Box<Self> {
        Box::new(Self {
            name,
            paths: vec![MovePath::new(name.to_owned(), 0, coords)],
        })
    }

    pub fn create_parallel(name: &'static str, coords: Vec<Move>, players: usize) -> Box<Self> {
        let mut ret = Self {
            name,
            paths: vec![],
        };
        for i in 0..players {
            ret.paths
                .push(MovePath::new(format!("{}[{}]", name, i), i, coords.clone()));
        }

        Box::new(ret)
    }

    pub fn create_coop(name: &'static str, paths: Vec<Vec<Move>>) -> Box<Self> {
        let mut ret = Self {
            name,
            paths: vec![],
        };
        for (i, path) in paths.iter().enumerate() {
            ret.paths
                .push(MovePath::new(format!("{}[{}]", name, i), i, path.clone()));
        }

        Box::new(ret)
    }
}

impl Display for SeqMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = format!("SeqMove({})", self.name);
        for path in &self.paths {
            ret = format!("{}\n{}", ret, path);
        }
        write!(f, "{}", ret)
    }
}

impl Node<GameState, GameEvent> for SeqMove {
    fn enter(&mut self, state: &mut GameState) {
        state.release_all();
    }

    fn on_event(&mut self, state: &mut GameState, event: &GameEvent) {
        for path in &mut self.paths {
            path.on_event(state, event);
        }
    }

    fn execute(&mut self, state: &mut GameState, delta: f64) -> bool {
        let mut sync_signals: Vec<(usize, Vec<usize>)> = Vec::new();
        for (player, path) in self.paths.iter_mut().enumerate() {
            let done = match path.execute(state, delta) {
                PathStatus::Done => true,
                PathStatus::Sync(list) => {
                    sync_signals.push((player, list));
                    false
                }
                _ => false,
            };
            // Require main path to return true (done)
            if player == 0 && done {
                return true;
            }
        }
        // Signal to any waiting players
        for (player, list) in sync_signals {
            for p in list {
                self.paths[p].on_event(state, &GameEvent::CoopSync(player));
            }
        }
        false
    }

    fn exit(&self, state: &mut GameState) {
        state.release_all();
    }
}
