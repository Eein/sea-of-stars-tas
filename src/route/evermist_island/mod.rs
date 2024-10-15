use crate::control::SosAction;
use crate::memory::player_party_manager::PlayerMovementState;
use crate::seq::button::ButtonPress;
use crate::seq::dialog::*;
use crate::seq::movement::*;
use crate::state::GameState;
use joystick::common::JoystickBtnInterface;
use joystick::common::JoystickInterface;
use libm::fabs;
use seq::prelude::*;

struct SeqChangeTimeTutorial {
    btn: ButtonPress,
    right_time: bool,
}

impl SeqChangeTimeTutorial {
    fn create() -> Box<Self> {
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
            const TARGET_TIME: f64 = 21.0;
            const TIME_EPSILON: f32 = 0.3;
            let time_diff = fabs(todm.current_time as f64 - TARGET_TIME) as f32;
            state.gamepad.press(&SosAction::TimeInc);
            if time_diff < TIME_EPSILON {
                self.right_time = true;
                state.gamepad.release(&SosAction::TimeInc);
            }
        }
        // Mash confirm to skip tutorial popup
        if self.btn.update(&mut state.gamepad, delta) {
            self.btn = ButtonPress::new(SosAction::Confirm);
        }
        // Done when we are back in control
        let ppmd = &state.memory_managers.player_party_manager.data;
        ppmd.movement_state == PlayerMovementState::Idle
    }

    fn exit(&self, state: &mut GameState) {
        // Cleanup, release buttons
        state.gamepad.release_all();
    }
}

pub fn create() -> Box<dyn Node<GameState>> {
    SeqList::create(
        "Sleeper Island",
        vec![
            SeqLog::create("MOVE START"),
            // TODO: more stuff
            SeqMove::create(
                "Leave dream world",
                vec![
                    Move::To(49.555, 1.002, 31.458),
                    Move::Interact(49.555, 1.002, 25.086),
                    Move::To(54.487, 1.002, 12.457),
                    Move::Interact(54.487, 1.002, 4.794),
                    Move::Interact(51.519, 1.002, 4.493),
                    Move::To(50.460, 1.002, 3.457),
                    Move::Interact(50.460, 1.002, 0.519),
                    Move::To(48.460, 1.002, -1.543),
                    Move::Interact(48.460, 1.002, -4.481),
                    Move::To(49.572, 1.002, -10.738),
                ],
            ),
            SeqSelectOption::create(0, false),
            SeqChangeTimeTutorial::create(),
            SeqMove::create(
                "Get Y'eeted",
                vec![
                    // Note, after Elder Mist time tutorial
                    Move::Log("Cross the bridges"),
                    Move::ChangeTime(21.0),
                    Move::To(46.352, 97.002, 170.300),
                    Move::ChangeTime(9.0),
                    Move::To(20.483, 97.000, 170.430),
                    Move::To(7.655, 97.000, 172.225),
                    Move::HoldDirWorld([-1.0, 0.0], [107.500, 6.002, 85.498]),
                    Move::Log("Leaving Elder Mist"),
                    Move::ToWorld(104.000, 6.002, 85.500),
                    Move::Log("Enter Y'eet"),
                    Move::Confirm,
                    Move::To(-421.597, 27.002, 175.500),
                    Move::To(-428.018, 27.002, 180.034),
                ],
            ),
            SeqSelectOption::create(0, false),
            SeqSkipUntilIdle::create(),
        ],
    )
}
