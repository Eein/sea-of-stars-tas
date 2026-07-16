use crate::combat::CombatController;
use delta::Timer;
use log::info;
use std::fmt::Display;

use joystick::common::{JoystickBtnInterface, JoystickInterface};
use seq::prelude::*;

use super::level_up::LevelUpController;
use crate::seq::button::ButtonPress;
use crate::{
    control::SosAction,
    state::{GameEvent, GameState},
};

#[derive(Default, Debug, PartialEq, Eq)]
enum GameFsm {
    Combat,
    #[default]
    Route,
    Cutscene,
    LevelUp,
}

/// The top-level TAS orchestrator: runs the route sequencer and hands frames
/// to the matching controller (combat, level-up, cutscene mash) whenever the
/// game state demands one. `*Manager` names are reserved for the mirrors of
/// the game's own Unity managers in `memory::`.
pub struct TasRunner {
    sequencer: Sequencer<GameState, GameEvent>,
    level_up_controller: Option<LevelUpController>,
    combat_controller: Option<CombatController>,
    fsm: GameFsm,
    btn: [ButtonPress; 3],
    timer: Timer,
    paused: bool,
    /// Whether the *game* was paused last frame (a player pressing Start
    /// mid-run), to log and clear inputs once per transition.
    game_paused: bool,
}

impl Display for TasRunner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSM: {:?}\nRoot: {}", self.fsm, self.sequencer)
    }
}

impl TasRunner {
    pub fn new(root: Box<dyn Node<GameState, GameEvent>>) -> Self {
        Self {
            sequencer: Sequencer::new(root),
            fsm: GameFsm::default(),
            btn: [
                ButtonPress::default(),
                ButtonPress::default(),
                ButtonPress::default(),
            ],
            timer: delta::Timer::new(),
            paused: false,
            game_paused: false,
            level_up_controller: None,
            combat_controller: None,
        }
    }

    pub fn start(&mut self, context: &mut GameState) {
        self.sequencer.start(context);
    }

    pub fn pause(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// The live combat controller, present while an encounter is being driven.
    pub fn combat_controller(&self) -> Option<&CombatController> {
        self.combat_controller.as_ref()
    }

    pub fn advance_to_checkpoint(&mut self, context: &mut GameState, checkpoint: &str) -> bool {
        self.sequencer.advance_to_checkpoint(context, checkpoint)
    }

    pub fn run(&mut self, context: &mut GameState) -> bool {
        let dt = self.timer.mark_secs();

        if self.paused {
            context.release_all();
            return false;
        }

        // The pause *menu* being up (a player pressing Start mid-run) freezes
        // everything while the sequences would otherwise keep driving blind —
        // hold the whole FSM and tap Cancel on every pad until it's dismissed,
        // then resume where we left off. Keyed off the PauseMenu view, not
        // `PauseManager.isPaused`: other screens (cooking, shops) also pause
        // gameplay, and those the TAS drives through.
        let game_paused = context.memory_managers.ui_manager.data.pause_menu_open();
        if game_paused != self.game_paused {
            self.game_paused = game_paused;
            context.release_all();
            if game_paused {
                info!("Game paused mid-run - holding the TAS and cancelling the pause menu");
            } else {
                info!("Game unpaused - resuming the TAS");
            }
        }
        if game_paused {
            for player in 0..self.btn.len() {
                if self.btn[player].update(&mut context.gamepads[player], dt) {
                    self.btn[player] = ButtonPress::new(SosAction::Cancel);
                }
            }
            return false;
        }

        let cmd = &context.memory_managers.combat_manager.data;
        let csmd = &context.memory_managers.cutscene_manager.data;
        let lumd = &context.memory_managers.level_up_manager.data;

        // TODO(orkaboy): detect game over?
        if cmd.encounter_active {
            // Stop whatever we're doing and enter combat. Only clear inputs on
            // the *transition* into combat, not every frame: once in combat the
            // controller owns the pads, and a per-frame `release_all` would blip
            // a release between frames — fatal to a held input like the Sunball
            // charge, which fires the instant the game reads Confirm as released.
            if self.fsm != GameFsm::Combat {
                for gamepad in context.gamepads.iter_mut() {
                    gamepad.release_all();
                }
                self.fsm = GameFsm::Combat;
            }
        } else if lumd.active {
            self.fsm = GameFsm::LevelUp;
        }

        match self.fsm {
            GameFsm::Combat => {
                if !cmd.encounter_active {
                    context.release_all();
                    self.combat_controller = None;
                    self.fsm = GameFsm::Route;
                    // Signal return to sequencer
                    self.sequencer.on_event(context, &GameEvent::Combat);
                } else if let Some(combat) = self.combat_controller.as_mut() {
                    if combat.update(context, dt) {
                        self.combat_controller = None;
                        self.fsm = GameFsm::Route;
                    }
                } else {
                    context.release_all();
                    self.combat_controller = Some(CombatController::default());
                }
            }
            GameFsm::LevelUp => {
                if let Some(level_up) = self.level_up_controller.as_mut() {
                    if level_up.update(context, dt) {
                        self.level_up_controller = None;
                        self.fsm = GameFsm::Route;
                    }
                } else {
                    context.release_all();
                    self.level_up_controller = Some(LevelUpController::default());
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
                for player in 0..self.btn.len() {
                    context.gamepads[player].press(&SosAction::Cancel);
                    context.gamepads[player].press(&SosAction::Turbo);
                    if self.btn[player].update(&mut context.gamepads[player], dt) {
                        self.btn[player] = ButtonPress::new(SosAction::Confirm);
                    }
                }
                if !csmd.is_in_cutscene {
                    context.release_all();
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
