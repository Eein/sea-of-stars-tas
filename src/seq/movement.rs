use std::fmt::{Display, Formatter, Result};

use crate::control::SosAction;
use crate::seq::button::ButtonPress;
use crate::state::{GameEvent, GameState};
use crate::util::vec3_ext::Vector3Ext;

use joystick::prelude::*;
use log::{debug, info, warn};
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
    /// Hold the left stick in a fixed direction until the player reaches a
    /// position: `HoldDir(dir, target)` presses `dir` (a normalized `[x, z]`
    /// joystick vector, e.g. `[0.0, -1.0]` = straight down) every frame and
    /// advances once the player is within 1.0 units of `target` (looser than
    /// `To`'s 0.2, since the path isn't steered). Unlike `To`, the direction
    /// never re-aims — use it where dead-reckoning beats steering: fixed-angle
    /// slopes/stairs, conveyor-like segments, screen transitions where the
    /// position readout jumps, or anywhere `To`'s course correction would
    /// zig-zag. The stick stays held when the step advances, so back-to-back
    /// `HoldDir` steps chain without a neutral frame.
    HoldDir([f32; 2], [f32; 3]),
    /// [`HoldDir`](Move::HoldDir), but the target is checked against the
    /// world-map position instead of the level-local one.
    HoldDirWorld([f32; 2], [f32; 3]),
    Confirm,
    Cancel,                  // Press the Cancel (B) button once
    HoldConfirm(f64),        // Hold Confirm for this many seconds, then release
    AwaitView(&'static str), // Wait until the named UI view (e.g. "CookingScreen") is up
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
            Move::Cancel => write!(f, "Move::Cancel"),
            Move::HoldConfirm(duration) => write!(f, "Move::HoldConfirm({:.3})", duration),
            Move::AwaitView(view) => write!(f, "Move::AwaitView(\"{}\")", view),
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
    dir: Option<Vector3<f32>>,
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
    /// Park the path at `step` with all transient driving state cleared, for
    /// the tree's play-from-here jump.
    fn jump_to(&mut self, step: usize) {
        self.step = step;
        self.btn = None;
        self.dir = None;
        self.timer = 0.0;
        self.semaphore.clear();
        self.sent_signal = false;
    }

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
            dir: None,
            step: 0,
            timer: 0.0,
            btn: None,
            player,
            semaphore: vec![],
            sent_signal: false,
        }
    }

    // Checks if we've overshot the movement target by comparing our directional vector
    // to the one originally calculated at the start of the Move segment.
    fn check_overshoot(&mut self, player: &Vector3<f32>, target: &Vector3<f32>) -> bool {
        /// A real overshoot ends up *near* the target (we ran past it at
        /// running speed — a frame or two of travel). A flipped direction
        /// while far away means the character was moved under us (scripted
        /// scene-entry runs, teleports): re-anchor on the current position
        /// and keep steering instead of skipping the move.
        const OVERSHOOT_RANGE: f32 = 2.5;

        let v1 = *target - *player;
        if let Some(v2) = &self.dir {
            let dot_product = v1.dot(v2);
            // The dot product is > 0 when the vectors are pointing in the same direction,
            // and < 0 when they are pointing more than 90 degrees away from each other.
            if dot_product < 0.0 {
                if v1.magnitude() <= OVERSHOOT_RANGE {
                    warn!("Overshoot! {}", self.coords[self.step]);
                    return true;
                }
                debug!(
                    "{}: direction flipped {:.1} units from target (game moved us?) — re-anchoring {}",
                    self.name,
                    v1.magnitude(),
                    self.coords[self.step],
                );
                self.dir = Some(v1);
            }
            false
        } else {
            // First iteration on this segment, assign dir to the vector between the player and target.
            self.dir = Some(v1);
            false
        }
    }

    fn is_close(player: &Vector3<f32>, target: &Vector3<f32>, precision: Option<f64>) -> bool {
        const PRECISION: f64 = 0.2;
        let diff = *target - *player;
        f64::from(diff.magnitude()) < precision.unwrap_or(PRECISION)
    }

    /// A position that reads exactly (0,0,0) is the memory resetting mid
    /// scene-swap (e.g. world map → level), not a real location — treat it as
    /// unreadable so a move neither steers at the origin nor "overshoots"
    /// past a target it never reached (the overshoot check would otherwise
    /// anchor its direction on the bogus origin read).
    fn live_position(position: &Option<Vector3<f32>>) -> Option<&Vector3<f32>> {
        position
            .as_ref()
            .filter(|p| p.get_x() != 0.0 || p.get_y() != 0.0 || p.get_z() != 0.0)
    }

    /// Arm a single tap of `action` (0.1s press, 0.2s release) as the
    /// in-flight button press.
    fn setup_button(&mut self, action: SosAction) {
        const PRESS_TIMEOUT: f64 = 0.1;
        const RELEASE_TIMEOUT: f64 = 0.2;
        self.btn = Some(ButtonPress {
            action,
            press_time: PRESS_TIMEOUT,
            release_time: RELEASE_TIMEOUT,
            ..Default::default()
        });
    }

    fn mash(&mut self, gamepad: &mut GenericJoystick, delta: f64) {
        if let Some(btn) = self.btn.as_mut() {
            if btn.update(gamepad, delta) {
                self.setup_button(SosAction::Confirm);
            }
        } else {
            self.setup_button(SosAction::Confirm);
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
        // While the game owns the character (teleporting back to the leader
        // after falling too far behind), pad input is ignored — treat the
        // position as unreadable so moves wait for control to come back
        // instead of steering at (or overshooting past) a character that
        // isn't listening.
        let game_owned = sppmd.players.items[self.player].state.contains("Teleport");
        let player = if game_owned {
            None
        } else {
            MovePath::live_position(&sppmd.players.items[self.player].gameobject_position)
        };

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
                let me = &sppmd.players.items[self.player];
                let game_paused = state.memory_managers.ui_manager.data.pause_menu_open();
                if me.playing {
                    // Joined — but the game may still own the character (the
                    // spawn walk-in, then a teleport-to-leader if they ended
                    // far away). Hold everything until they're actually
                    // controllable; advancing on `playing` alone drove the
                    // next moves into a character that ignored the pad while
                    // walking itself out of range.
                    gamepad.release_all();
                    if me.state == "PlayerDefaultState" {
                        self.step += 1;
                    }
                } else if game_paused || !me.can_join_leave {
                    // Join maps to Start. Pressed while the pause menu is up —
                    // or while the game won't accept a join — it opens the
                    // pause menu instead, which the runner then cancels and
                    // this press would immediately re-open, forever. Hold off
                    // until the game reports joining possible.
                    gamepad.release(&SosAction::Join);
                } else {
                    gamepad.press(&SosAction::Join);
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
                // Proceed only once *every* listed (and playing) player has
                // the boost — the flags don't set on the same frame, and a
                // participant advancing on its own flag alone walks away from
                // a partner still mid hi-five. Absent/non-playing players
                // don't block (e.g. a co-op route run with fewer pads).
                let all_boosted = list.iter().all(|&p| {
                    sppmd
                        .players
                        .items
                        .get(p)
                        .is_none_or(|player| !player.playing || player.has_boost)
                });
                if !list.contains(&self.player) || all_boosted {
                    gamepad.release_all();
                    self.step += 1;
                } else {
                    // Stand still while hi-fiving: clear any held stick or
                    // buttons from the previous move, then offer the hi-five
                    // until everyone's boost lands.
                    gamepad.release_all();
                    if !sppmd.players.items[self.player].has_boost {
                        gamepad.press(&SosAction::HiFive);
                    }
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
                if mash {
                    self.mash(gamepad, delta);
                }
                if let Some(player) = player {
                    if MovePath::is_close(player, &target, Some(1.0)) {
                        gamepad.release_all();
                        self.btn = None;
                        self.step += 1;
                    } else {
                        let joy_dir = MovePath::get_dir(player, &anchor, false);
                        gamepad.set_ljoy(joy_dir);
                    }
                }
            }
            // Move towards the target coordinate until it's reached
            Move::To(x, y, z) => {
                let target = Vector3::new(x, y, z);
                if let Some(player) = player {
                    if MovePath::is_close(player, &target, None)
                        || self.check_overshoot(player, &target)
                    {
                        self.step += 1;
                        self.dir = None;
                    } else {
                        let joy_dir = MovePath::get_dir(player, &target, false);
                        gamepad.set_ljoy(joy_dir);
                        // Steering telemetry (`--log-level debug`): what this
                        // path sees and sends, to separate bad reads from bad
                        // steering when a player runs the wrong way.
                        debug!(
                            "{}: To({x:.2},{y:.2},{z:.2}) from ({:.2},{:.2},{:.2}) ljoy=[{:.2},{:.2}]",
                            self.name,
                            player.get_x(),
                            player.get_y(),
                            player.get_z(),
                            joy_dir[0],
                            joy_dir[1],
                        );
                    }
                }
            }
            // Climb towards the target coordinate until it's reached (mash to get on wall)
            Move::Climb(x, y, z) => {
                let target = Vector3::new(x, y, z);
                self.mash(gamepad, delta);
                if let Some(player) = player {
                    if MovePath::is_close(player, &target, None)
                        || self.check_overshoot(player, &target)
                    {
                        gamepad.release_all();
                        self.btn = None;
                        self.step += 1;
                        self.dir = None;
                    } else {
                        let joy_dir = MovePath::get_dir(player, &target, true);
                        gamepad.set_ljoy(joy_dir);
                    }
                }
            }
            // Move towards the target while mashing
            Move::Interact(x, y, z) => {
                let target = Vector3::new(x, y, z);
                // If we are close to target, stop mashing to prevent unintended jumps
                if let Some(player) = player {
                    const INTERACT_PRECISION: f64 = 1.0;
                    if !MovePath::is_close(player, &target, Some(INTERACT_PRECISION)) {
                        self.mash(gamepad, delta);
                    } else {
                        gamepad.release(&SosAction::Confirm);
                    }
                    // If we are even closer, proceed.
                    if MovePath::is_close(player, &target, None)
                        || self.check_overshoot(player, &target)
                    {
                        gamepad.release_all();
                        self.btn = None;
                        self.step += 1;
                        self.dir = None;
                    } else {
                        let joy_dir = MovePath::get_dir(player, &target, false);
                        gamepad.set_ljoy(joy_dir);
                    }
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
                let world_pos = MovePath::live_position(&sppmd.players.items[self.player].position);
                if let Some(world_pos) = world_pos {
                    if MovePath::is_close(world_pos, &target, None) {
                        self.step += 1;
                    } else {
                        let joy_dir = MovePath::get_dir(world_pos, &target, false);
                        gamepad.set_ljoy(joy_dir);
                    }
                }
            }
            Move::HoldDir(dir, target) => {
                gamepad.set_ljoy(dir);
                let target = Vector3::new(target[0], target[1], target[2]);
                if let Some(player) = player
                    && MovePath::is_close(player, &target, Some(1.0))
                {
                    self.step += 1;
                }
            }
            Move::HoldDirWorld(dir, target) => {
                gamepad.set_ljoy(dir);
                let target = Vector3::new(target[0], target[1], target[2]);
                let world_pos = MovePath::live_position(&sppmd.players.items[self.player].position);
                if let Some(world_pos) = world_pos
                    && MovePath::is_close(world_pos, &target, Some(1.0))
                {
                    self.step += 1;
                }
            }
            // Change Time of Day
            Move::ChangeTime(target_time) => self.change_time(state, target_time),
            // Press confirm or cancel (B) once, e.g. to advance a dialog or
            // back out of a menu
            Move::Confirm | Move::Cancel => {
                if let Some(btn) = self.btn.as_mut() {
                    if btn.update(gamepad, delta) {
                        self.btn = None;
                        self.step += 1;
                        gamepad.release_all();
                    }
                } else {
                    gamepad.release_all(); // Release held joystick direction
                    self.setup_button(if matches!(coord, Move::Cancel) {
                        SosAction::Cancel
                    } else {
                        SosAction::Confirm
                    });
                }
            }
            // Wait (inputs released) until the named UI view is live in
            // `UIManager.screensByType` — e.g. gate a cooking sequence on
            // "CookingScreen" before holding Confirm to cook.
            Move::AwaitView(view) => {
                gamepad.release_all();
                if state.memory_managers.ui_manager.data.view_open(view) {
                    self.step += 1;
                }
            }
            // Hold Confirm for a duration, then release and advance. Uses the
            // shared step timer (like WaitFor) rather than a ButtonPress, so
            // the hold is continuous — no release blips mid-hold.
            Move::HoldConfirm(duration) => {
                if self.timer == 0.0 {
                    gamepad.release_all(); // Release held joystick direction
                }
                gamepad.press(&SosAction::Confirm);
                self.timer += delta;
                if self.timer >= duration {
                    self.timer = 0.0;
                    gamepad.release_all();
                    self.step += 1;
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
                    self.dir = None;
                }
            }
            GameEvent::Cutscene => {
                match coord {
                    Move::AwaitCutscene(_) => {
                        self.btn = None;
                        self.step += 1;
                        self.dir = None;
                    }
                    // A Confirm whose press was in flight is what opened the
                    // dialog/cutscene the masher just resolved. Resuming would
                    // replay the rest of the press cycle — a stray Confirm
                    // that re-opens the dialog. The press landed; step done.
                    Move::Confirm if self.btn.is_some() => {
                        self.btn = None;
                        self.step += 1;
                    }
                    _ => {}
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

    /// Jump within the moves, mirroring [`tree`](Self::tree)'s child order:
    /// single-player `[i]` parks player 0 on move `i`; the bare node restarts
    /// every path. Co-op `[p]`/`[p, i]` repositions only player `p`'s path —
    /// best-effort, since the other players' positions and sync points can't
    /// be derived from a single node address.
    fn advance_to_path(&mut self, path: &[usize]) -> bool {
        if path.is_empty() {
            for move_path in &mut self.paths {
                move_path.jump_to(0);
            }
            return true;
        }
        if let [move_path] = self.paths.as_mut_slice() {
            let [step] = path else {
                return false;
            };
            if *step >= move_path.coords.len() {
                return false;
            }
            move_path.jump_to(*step);
            return true;
        }
        let (&player, rest) = path.split_first().expect("checked non-empty");
        let Some(move_path) = self.paths.get_mut(player) else {
            return false;
        };
        match rest {
            [] => move_path.jump_to(0),
            [step] if *step < move_path.coords.len() => move_path.jump_to(*step),
            _ => return false,
        }
        true
    }

    /// Expose the moves as tree children: single-player paths list their
    /// `Move`s directly under the node; co-op shows one subtree per player's
    /// path. `active_child` marks the move each path is currently driving.
    /// Every node lists its moves — collapsed sections aren't rendered, but
    /// they must be openable by hand (to inspect, and later to pick a move to
    /// play from).
    /// (Co-op paths run in parallel, which the tree's single active index
    /// can't express — the parallel node marks player 0's path.)
    fn tree(&self, active: bool) -> SeqTreeNode {
        let path_tree = |path: &MovePath| {
            let in_bounds = path.step < path.coords.len();
            SeqTreeNode {
                label: format!("{} ({}/{})", path.name, path.step, path.coords.len()),
                children: path
                    .coords
                    .iter()
                    .enumerate()
                    .map(|(i, m)| SeqTreeNode {
                        label: m.to_string(),
                        children: Vec::new(),
                        active_child: None,
                        active: active && in_bounds && i == path.step,
                        completed: i < path.step,
                    })
                    .collect(),
                active_child: (active && in_bounds).then_some(path.step),
                active,
                completed: !in_bounds,
            }
        };
        match self.paths.as_slice() {
            [path] => path_tree(path),
            paths => SeqTreeNode {
                label: format!("SeqMove({})", self.name),
                children: paths.iter().map(path_tree).collect(),
                active_child: Some(0),
                active,
                completed: paths.first().is_none_or(|p| p.step >= p.coords.len()),
            },
        }
    }

    fn on_event(&mut self, state: &mut GameState, event: &GameEvent) {
        for path in &mut self.paths {
            path.on_event(state, event);
        }
    }

    fn execute(&mut self, state: &mut GameState, delta: f64) -> bool {
        let mut sync_signals: Vec<(usize, Vec<usize>)> = Vec::new();
        let paths_len = self.paths.len();
        for (player, path) in self.paths.iter_mut().enumerate() {
            let done = match path.execute(state, delta) {
                // Signal all paths on done, in case we skipped ending AwaitSync with Leave
                PathStatus::Done => {
                    let mut players = Vec::new();
                    for i in 0..paths_len {
                        players.push(i);
                    }
                    sync_signals.push((player, players));
                    true
                }
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

#[cfg(test)]
mod tests {
    use super::MovePath;
    use vec3_rs::Vector3;

    #[test]
    fn overshoot_test() -> std::io::Result<()> {
        use super::Move;
        // Coords back the warn's display of the current move.
        let mut m1 = MovePath::new("test".to_owned(), 0, vec![Move::To(100.0, 10.0, 0.0)]);
        // Set up target position and initial player position
        let target_pos = Vector3::from((100.0_f32, 10.0, 0.0));
        let mut player_pos = Vector3::from((0.0_f32, 0.0, 0.0));
        // Initially, dir should be uninitialized
        assert!(m1.dir.is_none());
        assert!(!m1.check_overshoot(&player_pos, &target_pos));
        // After checking overshoot, dir should be initialized to Some(target - player)
        assert!(m1.dir.is_some());
        if let Some(dir) = &m1.dir {
            // Since player starts at (0,0,0), dir should be target
            assert_eq!(dir, &target_pos);
        }
        // Move the player and test
        player_pos = Vector3::from((50.0_f32, 5.0, 0.0));
        assert!(!m1.check_overshoot(&player_pos, &target_pos));
        // Move the player and test
        player_pos = Vector3::from((10.0_f32, 50.0, 0.0));
        assert!(!m1.check_overshoot(&player_pos, &target_pos));
        // Just past the target (within running distance): a real overshoot.
        player_pos = Vector3::from((101.0_f32, 11.0, 0.5));
        assert!(m1.check_overshoot(&player_pos, &target_pos));

        // A direction flip *far* from the target is not an overshoot — it
        // means the game moved the character (scripted scene-entry runs,
        // teleports). The anchor re-derives and steering resumes.
        let mut m2 = MovePath::new("test".to_owned(), 0, vec![Move::To(100.0, 10.0, 0.0)]);
        let start = Vector3::from((0.0_f32, 0.0, 0.0));
        assert!(!m2.check_overshoot(&start, &target_pos));
        let dragged_past = Vector3::from((110.0_f32, 20.0, 4.0));
        assert!(!m2.check_overshoot(&dragged_past, &target_pos));
        // Walking back toward the target with the re-derived anchor is fine...
        let returning = Vector3::from((104.0_f32, 14.0, 1.0));
        assert!(!m2.check_overshoot(&returning, &target_pos));
        // ...and genuinely running past it now trips the overshoot.
        let past_again = Vector3::from((99.0_f32, 9.0, 0.0));
        assert!(m2.check_overshoot(&past_again, &target_pos));
        Ok(())
    }
}
