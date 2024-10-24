use std::fmt::Display;

use crate::control::SosAction;
use crate::seq::button::ButtonPress;
use crate::state::{GameEvent, GameState};

use joystick::prelude::*;
use seq::prelude::*;

#[derive(Default, Debug)]
enum SelectFsm {
    #[default]
    Approach,
    WaitForDialog,
    ClearPrompt,
    ToAnswer(usize),
    Answer,
}

pub struct SeqSelectOption {
    btn: ButtonPress,
    option: Vec<usize>,
    skip_dialog_check: bool,
    fsm: SelectFsm,
    timer: f64,
    step: usize,
}

impl SeqSelectOption {
    pub fn create(option: Vec<usize>, skip_dialog_check: bool) -> Box<Self> {
        Box::new(Self {
            btn: ButtonPress::new(SosAction::Confirm),
            option,
            skip_dialog_check,
            fsm: Default::default(),
            timer: 0.0,
            step: 0,
        })
    }
}

const HOLD_TIME: f64 = 0.3;

impl Display for SeqSelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SelectOption({:?})", self.fsm)
    }
}

impl Node<GameState, GameEvent> for SeqSelectOption {
    fn enter(&mut self, state: &mut GameState) {
        state.gamepad.release_all();
    }

    fn cutscene_control(&self) -> bool {
        true
    }

    fn execute(&mut self, state: &mut GameState, delta: f64) -> bool {
        match self.fsm {
            SelectFsm::Approach => {
                if self.btn.update(&mut state.gamepad, delta) {
                    self.fsm = SelectFsm::WaitForDialog;
                }
            }
            SelectFsm::WaitForDialog => {
                let ndmd = &state.memory_managers.new_dialog_manager.data;
                if ndmd.dialog_visible || self.skip_dialog_check {
                    state.gamepad.press(&SosAction::Turbo);
                    state.gamepad.press(&SosAction::Confirm);
                    self.fsm = SelectFsm::ClearPrompt;
                }
            }
            SelectFsm::ClearPrompt => {
                self.timer += delta;
                if self.timer >= HOLD_TIME {
                    self.fsm = SelectFsm::ToAnswer(0);
                    self.btn = ButtonPress::new(SosAction::MenuDown);
                    state.gamepad.release_all();
                }
            }
            SelectFsm::ToAnswer(option) => {
                if let Some(opt) = self.option.get(self.step) {
                    if option == *opt {
                        self.fsm = SelectFsm::Answer;
                        self.btn = ButtonPress::new(SosAction::Confirm);
                    } else if self.btn.update(&mut state.gamepad, delta) {
                        self.fsm = SelectFsm::ToAnswer(option + 1);
                        self.btn = ButtonPress::new(SosAction::MenuDown);
                    }
                } else {
                    return true;
                }
            }
            SelectFsm::Answer => {
                if self.btn.update(&mut state.gamepad, delta) {
                    self.step += 1;
                    self.fsm = SelectFsm::ToAnswer(0);
                    self.btn = ButtonPress::new(SosAction::MenuDown);
                }
            }
        }
        false
    }

    fn exit(&self, state: &mut GameState) {
        state.gamepad.release_all();
    }
}
