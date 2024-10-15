use crate::control::SosAction;
use crate::memory::player_party_manager::PlayerMovementState;
use crate::seq::button::ButtonPress;
use crate::state::GameState;

use joystick::prelude::*;
use seq::prelude::*;

#[derive(Default)]
enum LootFsm {
    #[default]
    Grab,
    Clear,
}

pub struct SeqLoot {
    btn: ButtonPress,
    fsm: LootFsm,
}

impl SeqLoot {
    pub fn create() -> Box<Self> {
        Box::new(Self {
            btn: ButtonPress::new(SosAction::Confirm),
            fsm: LootFsm::default(),
        })
    }
}

impl Node<GameState> for SeqLoot {
    fn execute(&mut self, state: &mut GameState, delta: f64) -> bool {
        match self.fsm {
            LootFsm::Grab => {
                if self.btn.update(&mut state.gamepad, delta) {
                    self.fsm = LootFsm::Clear;
                }
            }
            LootFsm::Clear => {
                // Skip past dialog box
                state.gamepad.press(&SosAction::Turbo);
                state.gamepad.press(&SosAction::Confirm);

                let ppmd = &state.memory_managers.player_party_manager.data;
                if ppmd.movement_state == PlayerMovementState::Idle {
                    state.gamepad.release_all();
                    // TODO(orkaboy): Optionally, Equip item. Requires more inventory memory data
                    return true;
                }
            }
        }
        false
    }
}
