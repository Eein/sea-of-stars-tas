use crate::control::SosAction;
use crate::memory::player_party_manager::PlayerMovementState;
use crate::seq::button::ButtonPress;
use crate::state::GameState;
use joystick::common::JoystickBtnInterface;
use joystick::common::JoystickInterface;
use libm::fabs;
use seq::prelude::*;

pub struct SeqChangeTimeTutorial {
    btn: ButtonPress,
    right_time: bool,
}

impl SeqChangeTimeTutorial {
    pub fn create() -> Box<Self> {
        Box::new(Self {
            btn: ButtonPress::new(SosAction::Confirm),
            right_time: false,
        })
    }
}

impl Node<GameState> for SeqChangeTimeTutorial {
    fn execute(&mut self, state: &mut GameState, delta: f64) -> bool {
        // Hold cancel to skip cut-scenes
        state.gamepad.press(&SosAction::Cancel);
        // Hold time inc to pass tutorial
        if !self.right_time {
            let todm = &state.memory_managers.time_of_day_manager.data;
            const TARGET_TIME: f64 = 16.7;
            const TIME_EPSILON: f32 = 0.3;
            let time_diff = fabs(todm.current_time as f64 - TARGET_TIME) as f32;
            state.gamepad.press(&SosAction::TimeDec);
            if time_diff < TIME_EPSILON {
                self.right_time = true;
                state.gamepad.release(&SosAction::TimeDec);
            }
        }
        // Mash confirm to skip tutorial popup
        if self.btn.update(&mut state.gamepad, delta) {
            self.btn = ButtonPress::new(SosAction::Confirm);
        }
        // Done when we are back in control
        let ppmd = &state.memory_managers.player_party_manager.data;
        self.right_time && ppmd.movement_state == PlayerMovementState::Idle
    }

    fn cutscene_control(&self) -> bool {
        true
    }

    fn exit(&self, state: &mut GameState) {
        // Cleanup, release buttons
        state.gamepad.release_all();
    }
}
