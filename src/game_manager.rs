use delta::Timer;
use std::fmt::Display;

use joystick::common::{JoystickBtnInterface, JoystickInterface};
use seq::prelude::*;

use super::level_up::LevelUpManager;
use crate::seq::button::ButtonPress;
use crate::{
    control::SosAction,
    state::{GameEvent, GameState},
};

#[derive(Default, Debug)]
pub enum GameFsm {
    Combat,
    #[default]
    Route,
    Cutscene,
    LevelUp,
    Title,
}

pub struct GameManager {
    sequencer: Sequencer<GameState, GameEvent>,
    level_up: Option<LevelUpManager>,
    fsm: GameFsm,
    btn: ButtonPress,
    timer: Timer,
    paused: bool,
}

impl Display for GameManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSM: {:?}\nRoot: {}", self.fsm, self.sequencer)
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
            level_up: None,
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
        let lumd = &context.memory_managers.level_up_manager.data;

        // TODO(orkaboy): detect game over?
        // TODO(orkaboy): detect level up screen
        if cmd.encounter_active {
            // Stop whatever we're doing and enter combat controller
            context.gamepad.release_all();
            self.fsm = GameFsm::Combat;
        } else if lumd.active {
            self.fsm = GameFsm::LevelUp;
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
            GameFsm::LevelUp => {
                if let Some(level_up) = self.level_up.as_mut() {
                    if level_up.update(context, dt) {
                        self.level_up = None;
                        self.fsm = GameFsm::Route;
                    }
                } else {
                    context.gamepad.release_all();
                    self.level_up = Some(LevelUpManager::default());
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
            GameFsm::Title => {}
        }
        false
    }

    pub fn is_running(&self) -> bool {
        self.sequencer.is_running()
    }
}
