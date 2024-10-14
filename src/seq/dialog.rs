use crate::control::SosAction;
use crate::memory::player_party_manager::PlayerMovementState;
use crate::seq::button::ButtonPress;
use crate::state::GameState;

use joystick::prelude::*;
use seq::prelude::*;

#[derive(Default)]
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
    option: usize,
    skip_dialog_check: bool,
    fsm: SelectFsm,
    timer: f64,
}

impl SeqSelectOption {
    pub fn create(option: usize, skip_dialog_check: bool) -> Box<Self> {
        Box::new(Self {
            btn: ButtonPress::new(SosAction::Confirm),
            option,
            skip_dialog_check,
            fsm: Default::default(),
            timer: 0.0,
        })
    }
}

const HOLD_TIME: f64 = 0.3;

impl Node<GameState> for SeqSelectOption {
    fn enter(&mut self, state: &mut GameState) {
        state.gamepad.release_all();
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
                if option == self.option {
                    self.fsm = SelectFsm::Answer;
                    self.btn = ButtonPress::new(SosAction::Confirm);
                } else if self.btn.update(&mut state.gamepad, delta) {
                    self.fsm = SelectFsm::ToAnswer(option + 1);
                    self.btn = ButtonPress::new(SosAction::MenuDown);
                }
            }
            SelectFsm::Answer => {
                if self.btn.update(&mut state.gamepad, delta) {
                    return true;
                }
            }
        }
        false
    }

    fn exit(&self, state: &mut GameState) {
        state.gamepad.release_all();
    }
}

// Hold skip until we are out of any cutscene and back in control
pub struct SeqSkipUntilIdle;

impl SeqSkipUntilIdle {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl Node<GameState> for SeqSkipUntilIdle {
    fn execute(&mut self, state: &mut GameState, _delta: f64) -> bool {
        state.gamepad.press(&SosAction::Cancel);
        let ppmd = &state.memory_managers.player_party_manager.data;
        ppmd.movement_state == PlayerMovementState::Idle
    }

    fn exit(&self, state: &mut GameState) {
        state.gamepad.release_all();
    }
}